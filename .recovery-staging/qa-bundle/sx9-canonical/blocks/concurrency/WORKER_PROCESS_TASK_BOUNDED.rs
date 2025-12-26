// WORKER_PROCESS_TASK_BOUNDED
// worker process task bounded

/// Canonical Pattern Block (read-only)
/// Constraints: deterministic, no IO, no logging, no config, single responsibility.

pub fn compute_chunk_bounds(total_items: usize, max_workers: usize) -> (usize, usize) {
    let workers = max_workers.max(1);
    let chunk = (total_items + workers - 1) / workers;
    (workers, chunk.max(1))
}
