# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_entries_insert_implies_interp_insert/original.rs`
**Date:** 2026-03-25 04:09:13
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `zero_layer_arch_satisfies_inv` targets `Arch::inv()`, which is a ghost spec function defining a structural invariant — not an executable function's specification. The observation that a zero-layer architecture vacuously satisfies the invariant is mathematically expected and does not represent a spec weakness. No true positives were found.

## All Candidates

### φ1: zero_layer_arch_satisfies_inv
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-layer architecture is degenerate (no translation at all) but would satisfy the invariant since all forall-quantifiers are vacuously true
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` is a pure spec function, not an executable function's specification. The vacuous truth for zero-length layers is mathematically correct and expected — the invariant guards layer properties via a universally quantified formula, and an empty sequence trivially satisfies it. The `layers.len() <= X86_NUM_LAYERS` bound also holds. This is standard behavior for inductive invariants over sequences, not a spec gap.

### φ2: adjacent_entry_bases_collide
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two adjacent entries at the same layer sharing the same base address would mean distinct page table slots map the same virtual address region

### φ3: page_frame_size_zero_under_inv
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A page with a zero-size frame in a valid directory would represent a nonsensical mapping to no physical memory

### φ4: update_invalid_keeps_mapping
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Replacing a Page entry with Invalid should erase the mapping at that slot; a non-empty interp would mean unmapping fails to remove the translation

### φ5: nonzero_subdir_base_equals_parent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A non-first subdirectory sharing the parent's base_vaddr would mean distinct directory entries govern the same virtual address range, breaking isolation

