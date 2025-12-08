/**
 * Synaptix9 Core Engine
 * Migrated from CTAS7 Command Center
 * 
 * This module provides the core orchestration, execution, and coordination
 * capabilities for Synaptix9 implementations.
 */

// Core Orchestrator
export { SX9Orchestrator } from './services/SX9Orchestrator';
export type { SX9SystemStatus, ConnectionTarget } from './services/SX9Orchestrator';

// Execution Engines
export { LegionExecutionEngine } from './services/LegionExecutionEngine';
export type { LegionTask, ExecutionContext } from './services/LegionExecutionEngine';

export { ScriptExecutionCoordinator } from './services/ScriptExecutionCoordinator';
export type { ScriptExecution, CoordinationPlan } from './services/ScriptExecutionCoordinator';

// Query Engines
export { SlotGraphQueryEngine, slotGraphQueryEngine } from './services/SlotGraphQueryEngine';

// Connectors
export { HashingEngineConnector } from './services/HashingEngineConnector';
export type { HashRequest, HashResponse, BatchHashRequest, BatchHashResponse } from './services/HashingEngineConnector';

// Hooks
export { useWebSocket } from './hooks/useWebSocket';
export { useVoiceConversation } from './hooks/useVoiceConversation';
export { useSystemDiagnostics } from './hooks/useSystemDiagnostics';

// Types
export * from './types';

// Version
export const SX9_CORE_VERSION = '1.0.0';
