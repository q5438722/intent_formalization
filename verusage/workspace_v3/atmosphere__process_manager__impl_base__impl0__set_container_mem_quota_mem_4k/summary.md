# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_base__impl0__set_container_mem_quota_mem_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: quota_noop → `set_container_mem_quota_mem_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, spec_set_mem_4k is an identity function and the quota is never actually modified by the call.

### φ2: quota_ignores_input → `set_container_mem_quota_mem_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If verified, spec_set_mem_4k is a constant function and the new_quota parameter is completely ignored.

### φ3: domain_inconsistency → `set_container_mem_quota_mem_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, the spec is internally inconsistent since container_ptr should remain in an extensionally-equal domain.

### φ4: owned_procs_cleared → `set_container_mem_quota_mem_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, extensional equality on StaticLinkedList does not preserve length, breaking the frame condition on owned_procs.

### φ5: depth_changes → `set_container_mem_quota_mem_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, extensional equality on usize does not imply value equality, exposing an inconsistency in the depth preservation postcondition.

