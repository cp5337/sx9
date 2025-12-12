/**
 * CTAS System Configuration - Real Port Architecture
 * Based on ctas7-real-port-manager and ctasctl configurations
 */

export const SYSTEM_PORTS = {
  MAIN_OPS: {
    UI: 5173,              // Main UI port (this application)
    BRIDGE_SERVICE: 15001,  // gRPC-Web bridge for Neural Mux (confirmed running)
  },

  // CTAS Port Manager Range: 18100-18199
  ORBITAL_BLOCK: {        // 18120-18139
    GROUNDSTATIONS_HFT: 18120,
    ORBITAL_MECHANICS: 18121,
    ENHANCED_GEOLOCATION: 18122,
    ORBITAL_INGEST: 18123,
    LASERLIGHT_CONSTELLATION: 18124,
    MCP_LASER_LIGHT: 18125,
    SPACE_WORLD_FOUNDATION_BRIDGE: 18126,
  },

  CDN_BLOCK: {            // 18140-18159
    PRIMARY: 18140,
    MIRROR_1: 18141,
    MIRROR_2: 18142,
  },

  NEURAL_MESH_BLOCK: {    // 18160-18179
    PRIMARY: 18160,
    MIRROR_1: 18161,
    MIRROR_2: 18162,
  },

  // Memory Mesh Services (ctasctl): 19011-19016
  MEMORY_MESH: {
    CONTEXT_MESH: 19011,
    ATOMIC_CLIPBOARD: 19012,
    THALMIC_FILTER: 19013,
    SLEDIS_CACHE: 19014,      // The real sledis database!
    VOICE_GATEWAY: 19015,
    SHUTTLE_SYNC: 19016,
  },

  DATABASES: {
    SURREALDB: 8000,           // SurrealDB (confirmed running)
    SLEDIS: 19014,             // Sledis cache database
  },

  NEURAL_MUX: {
    GRPC: 50051,               // Neural mux gRPC service
    BRIDGE: 15001,             // gRPC-Web bridge (confirmed running)
  },

  // Legacy/External Services
  SERVICES: {
    GROUND_STATION_ORCHESTRATOR: 18400,  // Ground station WebSocket
    CESIUM_DATA: 21575,                  // Cesium development center
    WEATHER_MCP: 19840,                  // Weather MCP server
  }
};

// Re-export hooks
export { useDatabase } from './useDatabase';
export { useSimulatedData } from './useSimulatedData';