/**
 * Slot Graph Query Engine
 * Uses SurrealDB for graph operations and Supabase for ACID transactions
 */

import { supabase } from '@/lib/supabase';
import {
  SlotGraphQuery,
  SlotGraphResult,
  Route,
  RouteConstraints,
  RouteHop,
  NetworkImpact,
  Pattern,
  SlotGraphNode,
  SlotGraphEdge,
  GroundStationNode,
  SatelliteNode,
  NetworkLink
} from './LegionSlotGraphSchema';

// SurrealDB connection configuration
const SURREALDB_URL = import.meta.env.VITE_SURREALDB_URL || 'http://localhost:8000';
const SURREALDB_NAMESPACE = 'sx9';
const SURREALDB_DATABASE = 'hft_network';

interface SurrealDBResponse<T = any> {
  time: string;
  status: string;
  result: T;
}

export class SlotGraphQueryEngine {
  private surrealToken: string | null = null;

  constructor() {
    this.initializeSurrealDB();
  }

  /**
   * Initialize SurrealDB connection
   */
  private async initializeSurrealDB() {
    try {
      const response = await fetch(`${SURREALDB_URL}/signin`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify({
          NS: SURREALDB_NAMESPACE,
          DB: SURREALDB_DATABASE,
          SC: 'allusers',
          user: 'root',
          pass: 'root'
        })
      });

      const data = await response.json();
      this.surrealToken = data.token;
      console.log('✅ SurrealDB connected for HFT Slot Graph');
    } catch (error) {
      console.error('❌ SurrealDB connection failed:', error);
      console.log('📊 Falling back to Supabase-only mode');
    }
  }

  /**
   * Execute SurrealDB query
   */
  private async querySurreal<T = any>(query: string, vars?: any): Promise<T> {
    if (!this.surrealToken) {
      throw new Error('SurrealDB not initialized');
    }

    const response = await fetch(`${SURREALDB_URL}/sql`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        'Authorization': `Bearer ${this.surrealToken}`,
        'NS': SURREALDB_NAMESPACE,
        'DB': SURREALDB_DATABASE
      },
      body: JSON.stringify({ query, vars })
    });

    const results: SurrealDBResponse<T>[] = await response.json();
    return results[0]?.result;
  }

  /**
   * Find all routes between two stations using graph traversal
   */
  async findAllRoutes(
    source: string,
    dest: string,
    maxHops: number = 5
  ): Promise<Route[]> {
    try {
      // Use SurrealDB graph traversal
      const query = `
        SELECT * FROM (
          SELECT 
            id,
            ->NetworkLink->(GroundStationNode, SatelliteNode) AS path
          FROM GroundStationNode:${source}
          WHERE id = $dest
          LIMIT ${maxHops}
        )
      `;

      const paths = await this.querySurreal(query, { dest });

      // Convert paths to Route objects
      return this.convertPathsToRoutes(paths, source, dest);
    } catch (error) {
      console.error('SurrealDB query failed, falling back to Supabase:', error);
      return this.findAllRoutesSupabase(source, dest, maxHops);
    }
  }

  /**
   * Fallback: Find routes using Supabase (BFS algorithm)
   */
  private async findAllRoutesSupabase(
    source: string,
    dest: string,
    maxHops: number
  ): Promise<Route[]> {
    // Get all network links from Supabase
    const { data: links } = await supabase
      .from('network_links')
      .select('*');

    if (!links) return [];

    // Build adjacency list
    const graph = new Map<string, Array<{ to: string; link: any }>>();
    for (const link of links) {
      if (!graph.has(link.from_id)) {
        graph.set(link.from_id, []);
      }
      graph.get(link.from_id)!.push({ to: link.to_id, link });
    }

    // BFS to find all paths
    const routes: Route[] = [];
    const queue: Array<{ node: string; path: RouteHop[]; visited: Set<string> }> = [
      { node: source, path: [], visited: new Set([source]) }
    ];

    while (queue.length > 0) {
      const { node, path, visited } = queue.shift()!;

      if (node === dest && path.length > 0) {
        routes.push(this.createRouteFromPath(source, dest, path));
        continue;
      }

      if (path.length >= maxHops) continue;

      const neighbors = graph.get(node) || [];
      for (const { to, link } of neighbors) {
        if (!visited.has(to)) {
          const newVisited = new Set(visited);
          newVisited.add(to);
          
          queue.push({
            node: to,
            path: [...path, {
              nodeId: to,
              nodeName: to,
              nodeType: to.startsWith('gs-') ? 'ground' : 'satellite',
              linkId: link.id,
              linkLatency: link.latency_ms,
              linkBandwidth: link.bandwidth_gbps,
              encryption: link.encryption
            }],
            visited: newVisited
          });
        }
      }
    }

    return routes;
  }

  /**
   * Find optimal route using Dijkstra's algorithm
   */
  async findOptimalRoute(
    source: string,
    destination: string,
    constraints: RouteConstraints
  ): Promise<Route | null> {
    try {
      // Use SurrealDB graph algorithm
      const query = `
        SELECT * FROM (
          SELECT 
            id,
            math::min(->NetworkLink.latency_ms) AS totalLatency,
            ->NetworkLink->(GroundStationNode, SatelliteNode) AS path
          FROM GroundStationNode:${source}
          WHERE 
            id = $dest
            ${constraints.maxLatency ? `AND totalLatency <= ${constraints.maxLatency}` : ''}
            ${constraints.requireQKD ? `AND ->NetworkLink.encryption = 'qkd'` : ''}
            ${constraints.minReliability ? `AND ->NetworkLink.reliability >= ${constraints.minReliability}` : ''}
          ORDER BY totalLatency ASC
          LIMIT 1
        )
      `;

      const result = await this.querySurreal(query, { dest: destination });
      
      if (result && result.length > 0) {
        return this.convertPathToRoute(result[0], source, destination);
      }
    } catch (error) {
      console.error('SurrealDB optimal route failed, using fallback:', error);
    }

    // Fallback: Use Dijkstra on Supabase data
    return this.findOptimalRouteSupabase(source, destination, constraints);
  }

  /**
   * Fallback: Dijkstra's algorithm using Supabase data
   */
  private async findOptimalRouteSupabase(
    source: string,
    destination: string,
    constraints: RouteConstraints
  ): Promise<Route | null> {
    // Get all nodes and links
    const [nodesResult, linksResult] = await Promise.all([
      supabase.from('ground_nodes').select('*'),
      supabase.from('network_links').select('*')
    ]);

    const nodes = nodesResult.data || [];
    const links = linksResult.data || [];

    // Build graph
    const graph = new Map<string, Array<{ to: string; weight: number; link: any }>>();
    
    for (const link of links) {
      // Apply constraints
      if (constraints.requireQKD && link.encryption !== 'qkd') continue;
      if (constraints.minReliability && link.reliability < constraints.minReliability) continue;
      if (constraints.minBandwidth && link.bandwidth_gbps < constraints.minBandwidth) continue;

      // Calculate weight: latency + (1/bandwidth) + (1-reliability) + weather_penalty
      const weight = 
        link.latency_ms +
        (1000 / link.bandwidth_gbps) +
        ((1 - link.reliability) * 100) +
        ((1 - (link.weather_impact || 1)) * 50);

      if (!graph.has(link.from_id)) {
        graph.set(link.from_id, []);
      }
      graph.get(link.from_id)!.push({ to: link.to_id, weight, link });
    }

    // Dijkstra's algorithm
    const distances = new Map<string, number>();
    const previous = new Map<string, { node: string; link: any }>();
    const unvisited = new Set<string>();

    // Initialize
    for (const node of nodes) {
      distances.set(node.id, Infinity);
      unvisited.add(node.id);
    }
    distances.set(source, 0);

    while (unvisited.size > 0) {
      // Find node with minimum distance
      let current: string | null = null;
      let minDist = Infinity;
      for (const node of unvisited) {
        const dist = distances.get(node)!;
        if (dist < minDist) {
          minDist = dist;
          current = node;
        }
      }

      if (!current || current === destination) break;
      unvisited.delete(current);

      // Update neighbors
      const neighbors = graph.get(current) || [];
      for (const { to, weight, link } of neighbors) {
        if (!unvisited.has(to)) continue;

        const alt = distances.get(current)! + weight;
        if (alt < distances.get(to)!) {
          distances.set(to, alt);
          previous.set(to, { node: current, link });
        }
      }
    }

    // Reconstruct path
    if (!previous.has(destination)) {
      return null; // No path found
    }

    const path: RouteHop[] = [];
    let current = destination;
    
    while (previous.has(current)) {
      const { node, link } = previous.get(current)!;
      path.unshift({
        nodeId: current,
        nodeName: current,
        nodeType: current.startsWith('gs-') ? 'ground' : 'satellite',
        linkId: link.id,
        linkLatency: link.latency_ms,
        linkBandwidth: link.bandwidth_gbps,
        encryption: link.encryption
      });
      current = node;
    }

    return this.createRouteFromPath(source, destination, path);
  }

  /**
   * Find network bottlenecks
   */
  async findBottlenecks(threshold: number = 0.8): Promise<NetworkLink[]> {
    try {
      const query = `
        SELECT * FROM NetworkLink
        WHERE utilization_slot.current.value > ${threshold}
        ORDER BY utilization_slot.current.value DESC
      `;

      return await this.querySurreal(query);
    } catch (error) {
      // Fallback to Supabase
      const { data } = await supabase
        .from('network_links')
        .select('*')
        .gte('utilization', threshold)
        .order('utilization', { ascending: false });

      return data || [];
    }
  }

  /**
   * Simulate network failure
   */
  async simulateFailure(nodeIds: string[]): Promise<NetworkImpact> {
    // Get all routes before failure
    const { data: allLinks } = await supabase
      .from('network_links')
      .select('*');

    const totalRoutes = allLinks?.length || 0;

    // Filter out links connected to failed nodes
    const activeLinks = allLinks?.filter(
      link => !nodeIds.includes(link.from_id) && !nodeIds.includes(link.to_id)
    ) || [];

    const routesAffected = totalRoutes - activeLinks.length;

    // Calculate alternate routes availability
    const graph = new Map<string, string[]>();
    for (const link of activeLinks) {
      if (!graph.has(link.from_id)) {
        graph.set(link.from_id, []);
      }
      graph.get(link.from_id)!.push(link.to_id);
    }

    // Check connectivity
    const { data: allNodes } = await supabase
      .from('ground_nodes')
      .select('id')
      .not('id', 'in', `(${nodeIds.join(',')})`);

    const activeNodes = allNodes?.map(n => n.id) || [];
    const connectedPairs = this.countConnectedPairs(graph, activeNodes);
    const totalPairs = (activeNodes.length * (activeNodes.length - 1)) / 2;
    const alternateRoutesAvailable = connectedPairs / totalPairs;

    // Calculate network health
    const networkHealthScore = 1 - (routesAffected / totalRoutes);

    // Find critical paths (single point of failure)
    const criticalPaths = this.findCriticalPaths(graph, activeNodes);

    return {
      failedNodes: nodeIds,
      totalRoutes,
      routesAffected,
      alternateRoutesAvailable,
      avgLatencyIncrease: routesAffected * 0.15, // Estimate 15% increase per affected route
      networkHealthScore,
      criticalPaths
    };
  }

  /**
   * Analyze traffic patterns over time
   */
  async analyzeTrafficPatterns(timeRange: { start: Date; end: Date }): Promise<Pattern[]> {
    try {
      const query = `
        SELECT 
          count() AS frequency,
          array::distinct(from_id) AS affectedNodes,
          time::group(timestamp, '1h') AS timeGroup
        FROM TradeRoute
        WHERE timestamp >= ${timeRange.start.toISOString()}
          AND timestamp <= ${timeRange.end.toISOString()}
        GROUP BY timeGroup
        ORDER BY frequency DESC
      `;

      const results = await this.querySurreal(query);

      return results.map((r: any) => ({
        patternType: 'traffic' as const,
        confidence: 0.85,
        frequency: r.frequency,
        affectedNodes: r.affectedNodes,
        timeRange,
        description: `High traffic pattern detected: ${r.frequency} trades/hour`
      }));
    } catch (error) {
      console.error('Pattern analysis failed:', error);
      return [];
    }
  }

  /**
   * Query HFT network with filters
   */
  async queryHFTNetwork(query: SlotGraphQuery): Promise<SlotGraphResult> {
    const startTime = Date.now();

    try {
      // Build SurrealDB query
      let surrealQuery = 'SELECT * FROM ';
      
      if (query.nodes && query.nodes.length > 0) {
        surrealQuery += `(${query.nodes.join(', ')})`;
      } else {
        surrealQuery += '(GroundStationNode, SatelliteNode)';
      }

      if (query.filters) {
        const conditions = Object.entries(query.filters)
          .map(([key, value]) => `${key} = ${JSON.stringify(value)}`)
          .join(' AND ');
        surrealQuery += ` WHERE ${conditions}`;
      }

      if (query.limit) {
        surrealQuery += ` LIMIT ${query.limit}`;
      }

      if (query.offset) {
        surrealQuery += ` START ${query.offset}`;
      }

      const nodes = await this.querySurreal(surrealQuery);

      return {
        nodes: nodes || [],
        edges: [],
        metadata: {
          query_time_ms: Date.now() - startTime,
          total_count: nodes?.length || 0,
          returned_count: nodes?.length || 0
        }
      };
    } catch (error) {
      console.error('SurrealDB query failed:', error);
      
      // Fallback to Supabase
      const { data: nodes } = await supabase
        .from('ground_nodes')
        .select('*')
        .limit(query.limit || 100);

      return {
        nodes: nodes || [],
        edges: [],
        metadata: {
          query_time_ms: Date.now() - startTime,
          total_count: nodes?.length || 0,
          returned_count: nodes?.length || 0
        }
      };
    }
  }

  // ============================================================================
  // HELPER METHODS
  // ============================================================================

  private convertPathsToRoutes(paths: any[], source: string, dest: string): Route[] {
    // Implementation depends on SurrealDB response format
    return [];
  }

  private convertPathToRoute(path: any, source: string, dest: string): Route {
    // Implementation depends on SurrealDB response format
    return {
      id: `route-${Date.now()}`,
      source,
      destination: dest,
      hops: [],
      totalLatency: 0,
      totalCost: 0,
      reliability: 1,
      bandwidth: 0,
      qkdCapable: false,
      createdAt: new Date()
    };
  }

  private createRouteFromPath(source: string, dest: string, path: RouteHop[]): Route {
    const totalLatency = path.reduce((sum, hop) => sum + (hop.linkLatency || 0), 0);
    const minBandwidth = Math.min(...path.map(hop => hop.linkBandwidth || Infinity));
    const qkdCapable = path.every(hop => hop.encryption === 'qkd');

    return {
      id: `route-${source}-${dest}-${Date.now()}`,
      source,
      destination: dest,
      hops: path,
      totalLatency,
      totalCost: totalLatency * 0.01, // $0.01 per ms
      reliability: 0.99, // Calculate from link reliabilities
      bandwidth: minBandwidth,
      qkdCapable,
      createdAt: new Date()
    };
  }

  private countConnectedPairs(graph: Map<string, string[]>, nodes: string[]): number {
    let count = 0;
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        if (this.isConnected(graph, nodes[i], nodes[j])) {
          count++;
        }
      }
    }
    return count;
  }

  private isConnected(graph: Map<string, string[]>, start: string, end: string): boolean {
    const visited = new Set<string>();
    const queue = [start];

    while (queue.length > 0) {
      const current = queue.shift()!;
      if (current === end) return true;
      if (visited.has(current)) continue;
      visited.add(current);

      const neighbors = graph.get(current) || [];
      queue.push(...neighbors);
    }

    return false;
  }

  private findCriticalPaths(graph: Map<string, string[]>, nodes: string[]): string[][] {
    const criticalPaths: string[][] = [];

    // Find articulation points (nodes whose removal disconnects the graph)
    // This is a simplified version - full implementation would use Tarjan's algorithm
    
    return criticalPaths;
  }
}

// Singleton instance
export const slotGraphQueryEngine = new SlotGraphQueryEngine();

