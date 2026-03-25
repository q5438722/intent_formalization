# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__syscall_receive_empty__impl0__syscall_receive_empty_no_block/original.rs`
**Date:** 2026-03-24T21:42:39Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property claims that wrapping two distinct `RetValueType` enum variants (`Error` vs `Else`) in the same `NoSwitchNew` constructor yields equal values. This is structurally impossible under Verus's equality semantics for enums: different variant discriminants guarantee inequality regardless of the surrounding constructor. The property is not entailed by the spec and is a false positive.

## All Candidates

### φ1: endpoint_excludes_zero → `syscall_receive_empty_no_block`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, endpoint index 0 is never valid, silently excluding the first descriptor slot

### φ2: requires_vacuous_on_index → `syscall_receive_empty_no_block`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, MAX_NUM_ENDPOINT_DESCRIPTORS is 0 and no valid index exists, making the syscall unreachable

### φ3: error_indistinguishable_from_success → `syscall_receive_empty_no_block`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If entailed, the caller cannot distinguish a failed receive from a successful one
- **Verdict:** FALSE_POSITIVE (high)

### φ4: scheduler_capacity_zero → `syscall_receive_empty_no_block`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, the scheduler-full check always triggers and the syscall can never successfully schedule a blocked sender

### φ5: queue_capacity_zero → `syscall_receive_empty_no_block`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, the endpoint queue capacity is zero so the queue-length branch conditions degenerate and no sender can ever be queued

