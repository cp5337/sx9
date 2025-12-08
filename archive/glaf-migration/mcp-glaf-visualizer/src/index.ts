#!/usr/bin/env node
/**
 * MCP Server for GLAF Universal Data Visualizer
 * 
 * Provides tools for Cursor/Claude to interact with all CTAS databases:
 * - SurrealDB (GLAF Core, Analytics, Main)
 * - PostgreSQL (Supabase)
 * - SlotGraph ECS
 * - GeoJSON layers
 * - Neo4j (visualization)
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

const CDN_URL = process.env.CDN_URL || "http://localhost:18100";

// ============================================================================
// Server Setup
// ============================================================================

const server = new Server(
  {
    name: "mcp-glaf-visualizer",
    version: "0.1.0",
  },
  {
    capabilities: {
      tools: {},
      resources: {},
    },
  }
);

// ============================================================================
// Tools
// ============================================================================

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "list_databases",
        description: "List all registered databases and their status in the CTAS CDN fabric",
        inputSchema: {
          type: "object",
          properties: {},
          required: [],
        },
      },
      {
        name: "query_graph",
        description: "Execute a graph query (SurrealQL, Cypher, or SQL) and return results as graph data",
        inputSchema: {
          type: "object",
          properties: {
            database: {
              type: "string",
              description: "Database ID (e.g., 'glaf-core', 'surrealdb-main', 'postgres-supabase')",
            },
            query: {
              type: "string",
              description: "The query to execute (SurrealQL, Cypher, or SQL)",
            },
            format: {
              type: "string",
              enum: ["graph", "table", "json", "geojson"],
              description: "Output format (default: graph)",
            },
          },
          required: ["database", "query"],
        },
      },
      {
        name: "get_schema",
        description: "Get the schema (tables, columns, relationships) for a database",
        inputSchema: {
          type: "object",
          properties: {
            database: {
              type: "string",
              description: "Database ID",
            },
          },
          required: ["database"],
        },
      },
      {
        name: "get_geojson_layer",
        description: "Get a GeoJSON layer (ground-stations, submarine-cables, cable-landings, landing-points)",
        inputSchema: {
          type: "object",
          properties: {
            layer: {
              type: "string",
              description: "Layer name",
            },
          },
          required: ["layer"],
        },
      },
      {
        name: "transform_format",
        description: "Transform data between formats (graph, table, geojson, cypher, surql)",
        inputSchema: {
          type: "object",
          properties: {
            data: {
              type: "object",
              description: "Input data to transform",
            },
            from_format: {
              type: "string",
              description: "Source format",
            },
            to_format: {
              type: "string",
              description: "Target format",
            },
          },
          required: ["data", "from_format", "to_format"],
        },
      },
      {
        name: "query_threat_intel",
        description: "Query PTCC-TETH threat intelligence data with HD4 phase filtering",
        inputSchema: {
          type: "object",
          properties: {
            hd4_phase: {
              type: "string",
              enum: ["Hunt", "Detect", "Disrupt", "Disable", "Dominate"],
              description: "Filter by HD4 phase",
            },
            threat_level: {
              type: "string",
              enum: ["low", "medium", "high", "critical"],
              description: "Filter by threat level",
            },
            limit: {
              type: "number",
              description: "Maximum results (default: 100)",
            },
          },
          required: [],
        },
      },
      {
        name: "get_convergence",
        description: "Calculate GLAF convergence score for a set of fragments",
        inputSchema: {
          type: "object",
          properties: {
            fragment_ids: {
              type: "array",
              items: { type: "string" },
              description: "Fragment IDs to analyze",
            },
            h1_input: {
              type: "number",
              description: "H1 operational input value (0-1)",
            },
          },
          required: ["fragment_ids"],
        },
      },
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    switch (name) {
      case "list_databases": {
        const response = await fetch(`${CDN_URL}/api/v1/databases`);
        const databases = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(databases, null, 2),
            },
          ],
        };
      }

      case "query_graph": {
        const { database, query, format = "graph" } = args as {
          database: string;
          query: string;
          format?: string;
        };

        const response = await fetch(`${CDN_URL}/api/v1/query`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ database, query, format }),
        });

        const result = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(result, null, 2),
            },
          ],
        };
      }

      case "get_schema": {
        const { database } = args as { database: string };
        const response = await fetch(`${CDN_URL}/api/v1/schema/${database}`);
        const schema = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(schema, null, 2),
            },
          ],
        };
      }

      case "get_geojson_layer": {
        const { layer } = args as { layer: string };
        const response = await fetch(`${CDN_URL}/api/v1/geojson/${layer}`);
        const geojson = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(geojson, null, 2),
            },
          ],
        };
      }

      case "transform_format": {
        const { data, from_format, to_format } = args as {
          data: object;
          from_format: string;
          to_format: string;
        };
        // Transform logic would go here
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify({ transformed: data, from: from_format, to: to_format }, null, 2),
            },
          ],
        };
      }

      case "query_threat_intel": {
        const { hd4_phase, threat_level, limit = 100 } = args as {
          hd4_phase?: string;
          threat_level?: string;
          limit?: number;
        };

        let query = `SELECT * FROM ptcc_configurations`;
        const conditions: string[] = [];
        
        if (hd4_phase) {
          conditions.push(`recommended_hd4_phase = '${hd4_phase}'`);
        }
        if (threat_level) {
          conditions.push(`threat_level = '${threat_level}'`);
        }
        
        if (conditions.length > 0) {
          query += ` WHERE ${conditions.join(" AND ")}`;
        }
        query += ` LIMIT ${limit}`;

        const response = await fetch(`${CDN_URL}/api/v1/query`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ database: "glaf-core", query, format: "table" }),
        });

        const result = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(result, null, 2),
            },
          ],
        };
      }

      case "get_convergence": {
        const { fragment_ids, h1_input = 0.5 } = args as {
          fragment_ids: string[];
          h1_input?: number;
        };

        // Call GLAF math endpoint
        const response = await fetch(`${CDN_URL}/api/glaf/convergence`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ fragment_indices: fragment_ids.map((_, i) => i), h1_input }),
        });

        const result = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(result, null, 2),
            },
          ],
        };
      }

      default:
        throw new Error(`Unknown tool: ${name}`);
    }
  } catch (error) {
    return {
      content: [
        {
          type: "text",
          text: `Error: ${error instanceof Error ? error.message : String(error)}`,
        },
      ],
      isError: true,
    };
  }
});

// ============================================================================
// Resources
// ============================================================================

server.setRequestHandler(ListResourcesRequestSchema, async () => {
  return {
    resources: [
      {
        uri: "glaf://databases",
        name: "CTAS Database Registry",
        description: "All registered databases in the CTAS CDN fabric",
        mimeType: "application/json",
      },
      {
        uri: "glaf://geojson/ground-stations",
        name: "Ground Stations",
        description: "Global ground station locations",
        mimeType: "application/geo+json",
      },
      {
        uri: "glaf://geojson/submarine-cables",
        name: "Submarine Cables",
        description: "Global submarine cable network",
        mimeType: "application/geo+json",
      },
      {
        uri: "glaf://geojson/cable-landings",
        name: "Cable Landings",
        description: "Submarine cable landing points",
        mimeType: "application/geo+json",
      },
      {
        uri: "glaf://schema/glaf-core",
        name: "GLAF Core Schema",
        description: "Schema for GLAF Core database",
        mimeType: "application/json",
      },
    ],
  };
});

server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
  const { uri } = request.params;

  try {
    if (uri === "glaf://databases") {
      const response = await fetch(`${CDN_URL}/api/v1/databases`);
      const data = await response.json();
      return {
        contents: [
          {
            uri,
            mimeType: "application/json",
            text: JSON.stringify(data, null, 2),
          },
        ],
      };
    }

    if (uri.startsWith("glaf://geojson/")) {
      const layer = uri.replace("glaf://geojson/", "");
      const response = await fetch(`${CDN_URL}/api/v1/geojson/${layer}`);
      const data = await response.json();
      return {
        contents: [
          {
            uri,
            mimeType: "application/geo+json",
            text: JSON.stringify(data, null, 2),
          },
        ],
      };
    }

    if (uri.startsWith("glaf://schema/")) {
      const database = uri.replace("glaf://schema/", "");
      const response = await fetch(`${CDN_URL}/api/v1/schema/${database}`);
      const data = await response.json();
      return {
        contents: [
          {
            uri,
            mimeType: "application/json",
            text: JSON.stringify(data, null, 2),
          },
        ],
      };
    }

    throw new Error(`Unknown resource: ${uri}`);
  } catch (error) {
    throw new Error(`Failed to read resource: ${error instanceof Error ? error.message : String(error)}`);
  }
});

// ============================================================================
// Main
// ============================================================================

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("🧬 MCP GLAF Visualizer server running on stdio");
}

main().catch(console.error);


