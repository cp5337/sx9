/**
 * Atomic Cliport Integration
 * 
 * Connects Atomic Cliport (robotics/vision) with Thalamic Filter and DistilBERT
 * for cognitive processing of visual/robotic commands.
 */

import { thalamicFilter } from './thalamic-filter';
import type { Threat } from '../../types/plasma';

const ATOMIC_CLIPORT_ENDPOINT = import.meta.env.VITE_ATOMIC_CLIPORT_URL || 'http://localhost:18120';
const DISTILBERT_ENDPOINT = import.meta.env.VITE_LEPTOSE_URL || 'http://localhost:18114';

export interface CliportCommand {
  id: string;
  type: 'pick' | 'place' | 'move' | 'grasp' | 'visual_query' | 'scene_analysis';
  target?: string;
  position?: { x: number; y: number; z: number };
  image_data?: string; // base64 encoded
  description: string;
  metadata?: {
    confidence?: number;
    hd4_phase?: string;
    agent_id?: string;
  };
}

export interface CliportResponse {
  success: boolean;
  result: string;
  confidence: number;
  processing_ms: number;
  thalamic_output?: {
    gate_decision: string;
    pathway: string;
    priority: string;
    activated_domains: string[];
  };
  distilbert_embedding?: number[];
}

/**
 * Process Cliport command through thalamic filter and DistilBERT
 */
export async function processCliportCommand(
  command: CliportCommand
): Promise<CliportResponse> {
  const startTime = performance.now();

  try {
    // Step 1: Convert command to threat-like structure for thalamic filter
    const threat: Threat = {
      id: command.id,
      level: command.metadata?.confidence && command.metadata.confidence > 0.8 ? 'high' : 'medium',
      description: command.description,
      source: 'atomic-cliport',
      confidence: command.metadata?.confidence || 0.7,
      mitre: [],
      indicators: [command.type],
      timestamp: new Date().toISOString(),
    };

    // Step 2: Process through thalamic filter (uses DistilBERT internally)
    const thalamicOutput = await thalamicFilter(threat);

    // Step 3: Get DistilBERT embedding for semantic understanding
    let distilbertEmbedding: number[] | undefined;
    try {
      const embeddingResponse = await fetch(`${DISTILBERT_ENDPOINT}/api/v1/embed`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          text: command.description,
          model: 'distilbert-base-uncased',
        }),
      });

      if (embeddingResponse.ok) {
        const embeddingData = await embeddingResponse.json();
        distilbertEmbedding = embeddingData.embedding;
      }
    } catch (error) {
      console.warn('[Atomic Cliport] DistilBERT embedding failed:', error);
    }

    // Step 4: Execute Cliport command
    const cliportResponse = await fetch(`${ATOMIC_CLIPORT_ENDPOINT}/api/v1/execute`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        command: command.type,
        target: command.target,
        position: command.position,
        image_data: command.image_data,
        description: command.description,
        thalamic_gate: thalamicOutput.gate_decision,
        priority: thalamicOutput.priority,
      }),
    });

    const cliportResult = await cliportResponse.json();

    const processingMs = Math.round(performance.now() - startTime);

    return {
      success: cliportResult.success || false,
      result: cliportResult.result || 'Command processed',
      confidence: cliportResult.confidence || 0.7,
      processing_ms: processingMs,
      thalamic_output: {
        gate_decision: thalamicOutput.gate_decision,
        pathway: thalamicOutput.pathway,
        priority: thalamicOutput.priority,
        activated_domains: thalamicOutput.activated_domains,
      },
      distilbert_embedding: distilbertEmbedding,
    };
  } catch (error) {
    console.error('[Atomic Cliport] Processing error:', error);
    return {
      success: false,
      result: `Error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      confidence: 0,
      processing_ms: Math.round(performance.now() - startTime),
    };
  }
}

/**
 * Batch process multiple Cliport commands
 */
export async function batchProcessCliportCommands(
  commands: CliportCommand[]
): Promise<Map<string, CliportResponse>> {
  const results = new Map<string, CliportResponse>();

  // Process in parallel with concurrency limit
  const batchSize = 5;
  for (let i = 0; i < commands.length; i += batchSize) {
    const batch = commands.slice(i, i + batchSize);
    const batchResults = await Promise.all(
      batch.map((cmd) => processCliportCommand(cmd))
    );
    batch.forEach((cmd, idx) => {
      results.set(cmd.id, batchResults[idx]);
    });
  }

  return results;
}

/**
 * Check if Cliport service is available
 */
export async function checkCliportHealth(): Promise<boolean> {
  try {
    const response = await fetch(`${ATOMIC_CLIPORT_ENDPOINT}/health`, {
      method: 'GET',
      signal: AbortSignal.timeout(2000),
    });
    return response.ok;
  } catch {
    return false;
  }
}



