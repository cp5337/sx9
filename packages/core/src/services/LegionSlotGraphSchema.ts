/**
 * Legion Slot Graph Schema for HFT Network
 * Defines node types, edge types, and slot properties for the 289-station network
 */

export interface SlotProperty<T = any> {
  value: T;
  timestamp: Date;
  quality: number; // 0-1 data quality score
}

export interface TimeSeriesSlot<T = number> {
  current: SlotProperty<T>;
  history: SlotProperty<T>[];
  forecast?: SlotProperty<T>[];
}

// ============================================================================
// NODE TYPES
// ============================================================================

export interface GroundStationNode {
  id: string;
  type: 'GroundStationNode';
  properties: {
    name: string;
    location: [number, number, number]; // [lat, lon, altitude]
    tier: 1 | 2 | 3;
    capacity_gbps: number;
    antennas: number;
    optical_capable: boolean;
    qkd_capable: boolean;
    status: 'active' | 'standby' | 'maintenance' | 'offline';
    region: string;
    clear_sky_days: number;
    uptime_sla: number;
  };
  slots: {
    bandwidth_slot: TimeSeriesSlot<number>;
    latency_slot: TimeSeriesSlot<number>;
    weather_slot: TimeSeriesSlot<WeatherConditions>;
    utilization_slot: TimeSeriesSlot<number>;
    qkd_key_budget_slot: TimeSeriesSlot<number>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export interface SatelliteNode {
  id: string;
  type: 'SatelliteNode';
  properties: {
    name: string;
    norad_id?: string;
    orbital_plane: string;
    altitude_km: number;
    inclination_deg: number;
    period_min: number;
    laser_power_w: number;
    qrng_capable: boolean;
    status: 'active' | 'commissioning' | 'degraded' | 'offline';
    uplink_stations: string[];
  };
  slots: {
    position_slot: TimeSeriesSlot<[number, number, number]>; // [lat, lon, alt]
    velocity_slot: TimeSeriesSlot<[number, number, number]>; // [vx, vy, vz]
    link_budget_slot: TimeSeriesSlot<number>;
    qkd_rate_slot: TimeSeriesSlot<number>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export interface TradeOrderNode {
  id: string;
  type: 'TradeOrderNode';
  properties: {
    order_id: string;
    timestamp: Date;
    from_exchange: string; // Maps to ground_station_id
    to_exchange: string;
    instrument: string;
    quantity: number;
    price: number;
    urgency: 'low' | 'medium' | 'high' | 'critical';
    require_qkd: boolean;
    max_latency_ms: number;
    status: 'pending' | 'routing' | 'transmitted' | 'completed' | 'failed';
  };
  slots: {
    route_slot: TimeSeriesSlot<string[]>; // Array of node IDs in route
    actual_latency_slot: TimeSeriesSlot<number>;
    cost_slot: TimeSeriesSlot<number>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export type SlotGraphNode = GroundStationNode | SatelliteNode | TradeOrderNode;

// ============================================================================
// EDGE TYPES
// ============================================================================

export interface NetworkLink {
  id: string;
  type: 'NetworkLink';
  from_id: string;
  to_id: string;
  properties: {
    link_type: 'ground-to-sat' | 'sat-to-sat' | 'ground-to-ground';
    bandwidth_gbps: number;
    base_latency_ms: number;
    reliability: number; // 0-1
    encryption: 'qkd' | 'classical' | 'hybrid';
    status: 'active' | 'congested' | 'degraded' | 'offline';
  };
  slots: {
    current_latency_slot: TimeSeriesSlot<number>;
    utilization_slot: TimeSeriesSlot<number>;
    packet_loss_slot: TimeSeriesSlot<number>;
    weather_impact_slot: TimeSeriesSlot<number>; // 0-1 degradation factor
    qkd_consumption_slot: TimeSeriesSlot<number>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export interface TradeRoute {
  id: string;
  type: 'TradeRoute';
  from_id: string; // TradeOrderNode
  to_id: string; // GroundStationNode (final destination)
  properties: {
    hops: string[]; // Array of node IDs in route
    total_latency_ms: number;
    total_cost: number;
    route_quality: number; // 0-1
    selected_at: Date;
    completed_at?: Date;
  };
  slots: {
    performance_slot: TimeSeriesSlot<RoutePerformance>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export interface WeatherImpact {
  id: string;
  type: 'WeatherImpact';
  from_id: string; // WeatherConditions source
  to_id: string; // GroundStationNode or NetworkLink
  properties: {
    impact_type: 'cloud_cover' | 'precipitation' | 'visibility' | 'wind';
    severity: number; // 0-1
    degradation_factor: number; // 0-1 (1 = no impact, 0 = complete blockage)
    forecast_confidence: number; // 0-1
  };
  slots: {
    current_impact_slot: TimeSeriesSlot<number>;
    forecast_slot: TimeSeriesSlot<number>;
  };
  metadata: {
    created_at: Date;
    updated_at: Date;
    version: number;
  };
}

export type SlotGraphEdge = NetworkLink | TradeRoute | WeatherImpact;

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

export interface WeatherConditions {
  temperature: number;
  cloudCover: number; // 0-100%
  precipitation: number; // mm/hour
  visibility: number; // km
  windSpeed: number; // km/h
  humidity: number; // 0-100%
  pressure: number; // hPa
  conditions: string;
}

export interface RoutePerformance {
  actual_latency_ms: number;
  expected_latency_ms: number;
  packet_loss: number;
  jitter_ms: number;
  throughput_gbps: number;
  reliability_score: number; // 0-1
}

// ============================================================================
// SLOT GRAPH SCHEMA
// ============================================================================

export interface SlotGraphSchema {
  nodes: {
    GroundStationNode: GroundStationNode;
    SatelliteNode: SatelliteNode;
    TradeOrderNode: TradeOrderNode;
  };
  edges: {
    NetworkLink: NetworkLink;
    TradeRoute: TradeRoute;
    WeatherImpact: WeatherImpact;
  };
}

// ============================================================================
// QUERY TYPES
// ============================================================================

export interface SlotGraphQuery {
  nodes?: string[]; // Node types to query
  edges?: string[]; // Edge types to query
  timeWindow?: {
    start: Date;
    end: Date;
  };
  filters?: {
    [key: string]: any;
  };
  limit?: number;
  offset?: number;
}

export interface SlotGraphResult<T = any> {
  nodes: SlotGraphNode[];
  edges: SlotGraphEdge[];
  metadata: {
    query_time_ms: number;
    total_count: number;
    returned_count: number;
  };
  data?: T;
}

export interface RouteConstraints {
  maxLatency?: number; // ms
  minBandwidth?: number; // Gbps
  minReliability?: number; // 0-1
  requireQKD?: boolean;
  avoidNodes?: string[]; // Node IDs to avoid
  preferNodes?: string[]; // Node IDs to prefer
  maxHops?: number;
}

export interface Route {
  id: string;
  source: string;
  destination: string;
  hops: RouteHop[];
  totalLatency: number;
  totalCost: number;
  reliability: number;
  bandwidth: number;
  qkdCapable: boolean;
  createdAt: Date;
}

export interface RouteHop {
  nodeId: string;
  nodeName: string;
  nodeType: 'ground' | 'satellite';
  linkId?: string;
  linkLatency?: number;
  linkBandwidth?: number;
  encryption?: string;
}

export interface NetworkImpact {
  failedNodes: string[];
  totalRoutes: number;
  routesAffected: number;
  alternateRoutesAvailable: number;
  avgLatencyIncrease: number;
  networkHealthScore: number; // 0-1
  criticalPaths: string[][]; // Routes that have no alternatives
}

export interface Pattern {
  patternType: 'traffic' | 'failure' | 'weather' | 'performance';
  confidence: number; // 0-1
  frequency: number; // occurrences per hour
  affectedNodes: string[];
  timeRange: {
    start: Date;
    end: Date;
  };
  description: string;
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

export function createTimeSeriesSlot<T>(initialValue: T): TimeSeriesSlot<T> {
  return {
    current: {
      value: initialValue,
      timestamp: new Date(),
      quality: 1.0
    },
    history: []
  };
}

export function createDynamicSlot<T>(
  fetchFn: () => Promise<T>
): TimeSeriesSlot<T> {
  return {
    current: {
      value: null as any,
      timestamp: new Date(),
      quality: 0
    },
    history: []
  };
}

export function updateSlot<T>(
  slot: TimeSeriesSlot<T>,
  newValue: T,
  quality: number = 1.0
): TimeSeriesSlot<T> {
  // Move current to history
  slot.history.push(slot.current);
  
  // Keep only last 1000 entries
  if (slot.history.length > 1000) {
    slot.history = slot.history.slice(-1000);
  }
  
  // Update current
  slot.current = {
    value: newValue,
    timestamp: new Date(),
    quality
  };
  
  return slot;
}

