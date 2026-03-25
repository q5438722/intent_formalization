# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_base__impl0__schedule_blocked_thread/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: push_thread_becomes_head → `scheduler_push_thread`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Push appends to the tail; the head of a non-empty scheduler must remain the old head element

### φ2: push_exceeds_max_capacity → `scheduler_push_thread`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Old length < MAX so new length <= MAX; exceeding MAX would indicate an overflow or off-by-one in the capacity bound

### φ3: push_removes_existing_element → `scheduler_push_thread`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Seq::push only appends; all previously present elements must still be contained in the new scheduler

### φ4: schedule_empties_multi_element_queue → `schedule_blocked_thread`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Skipping one element from a queue with >1 elements must leave a non-empty queue; emptying it would lose blocked threads

### φ5: schedule_modifies_other_endpoint → `schedule_blocked_thread`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Scheduling from one endpoint must not modify any other endpoint's queue; violating this frame condition would corrupt unrelated IPC channels

