# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_base__impl0__block_running_thread/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: blocked_thread_owning_proc_changes → `block_running_thread`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Blocking a thread should not change its owning process; the spec does not explicitly preserve owning_proc for the target thread

### φ2: endpoint_queue_empty_after_push → `block_running_thread`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** After pushing a thread onto the endpoint queue, queue length must be at least 1; an empty queue would indicate broken push semantics

### φ3: running_cpu_still_some_after_block → `block_running_thread`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A BLOCKED thread should not have a running CPU; if wf() allows this, the cpu-thread invariant (threads_cpu_wf) is too weak

### φ4: endpoint_rf_counter_changes → `block_running_thread`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Blocking a thread on an endpoint should not change the endpoint's reference counter; the spec omits rf_counter preservation for the target endpoint

### φ5: endpoint_owning_container_changes → `block_running_thread`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Blocking a thread should not migrate the target endpoint to a different container; the spec preserves queue_state and owning_threads but omits owning_container

