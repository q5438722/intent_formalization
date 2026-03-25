# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlproof/spec_t__hlproof__unmap_start_preserves_inv.rs`
**Date:** 2026-03-24T13:22:32Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: the `insert_non_map_preserves_unique` external_body lemma is an unverified trust assumption. One false positive: the unsound state transition on inflight conflict is an intentional modeling pattern for concurrent safety violations.

## True Positives (Spec Issues)

### insert_non_map_preserves_unique_external_body
- **Confidence:** medium
- **Reasoning:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` body — its postcondition is trusted without proof. While the property is likely correct, this is an unverified trust assumption in the invariant preservation proof chain for `unmap_start_preserves_inv`.

## All Candidates

### φ1: unmap_start_removes_mapping_immediately
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `step_UnmapStart` immediately removes the mapping from `s2.mappings` before the unmap operation completes — in a concurrent TLB system, removing the mapping before shootdown completes could allow stale TLB entries to reference freed physical memory

### φ2: insert_non_map_preserves_unique_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` — trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` body — its postcondition is trusted without proof. While the property is likely correct, this is an unverified trust assumption in the invariant preservation proof chain for `unmap_start_preserves_inv`.

### φ3: unmap_sound_pte_size_zero_for_unmapped
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When unmapping a non-existent vaddr, `pte_size` is 0 and `step_Unmap_sound` checks overlap with a zero-size region — zero-size overlap checks are unreliable due to the `overlap` bug

### φ4: unmap_start_unsound_on_inflight_conflict
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When unmap conflicts with inflight operations, the system transitions to unsound rather than rejecting the operation — permissive failure mode that poisons the entire state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional design. The `unsound_state` transition is the spec's way of modeling that an unsafe concurrent operation has occurred — it marks the state as unsound so that all subsequent invariant preservation obligations become vacuously true (guarded by `s2.sound ==> inv(c, s2)`). This is a standard pattern for modeling undefined behavior in verification, not a spec gap.

### φ5: unmap_stores_pte_in_thread_state
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The mapping is simultaneously removed from `mappings` and stored in the thread's Unmap state — if the thread crashes or is cancelled, the PTE exists only in `thread_state` and may never be restored or properly freed

