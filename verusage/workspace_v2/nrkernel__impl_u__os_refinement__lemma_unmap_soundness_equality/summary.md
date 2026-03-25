# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__lemma_unmap_soundness_equality.rs`
**Date:** 2026-03-24T13:17:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. `step_Unmap_sound` is intentionally scoped to concurrency safety — checking only for conflicts with in-flight thread operations. Mapping existence validation is handled separately in the transition logic. The empty-inflights case is trivially correct by design: no concurrent threads means no race conditions to guard against.

## All Candidates

### φ1: ptmem_view_external_body_write_invisible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — writes to page table memory have no observable effect on the PTE interpretation

### φ2: rl3_interp_external_body_collapses
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec — completely opaque with no postconditions, so any two rl3 states map to the same rl2 state

### φ3: step_unmap_sound_empty_inflights
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** With no in-flight threads, any unmap is trivially sound — the negation of an exists over an empty set is always true, so `step_Unmap_sound` provides no real safety check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When there are no in-flight threads, there are no concurrent operations to conflict with. The unmap soundness check is specifically about preventing races with concurrent map/unmap operations — if no threads are active, any unmap is indeed safe from a concurrency perspective. The existence check for the mapping itself is handled elsewhere in the transition logic.

### φ4: unmap_sound_ignores_existing_mappings
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `step_Unmap_sound` only checks inflight vmem overlap — it does not verify that the vaddr being unmapped actually exists in `s.mappings`, allowing unmaps of non-existent addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `step_Unmap_sound` is specifically a concurrency safety check — it verifies that an unmap doesn't conflict with in-flight operations. Whether the mapping exists at the given vaddr is a separate concern handled by other parts of the transition system (the unmap step itself would return an error result if the mapping doesn't exist). Separation of concerns is standard in layered state machine specs.

### φ5: overlap_asymmetric_at_boundary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `overlap` is asymmetric at boundary points — a zero-size region at the exclusive upper bound is detected as overlapping in one direction but not the reverse

