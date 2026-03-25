# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_submap_preserves_no_overlap.rs`
**Date:** 2026-03-24T14:50:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Vacuous truth for empty core states is correct for initialization. Submap preservation with empty submap is a valid (if degenerate) application of the lemma. Overlap monotonicity in sizes is a correct arithmetic property of the overlap definition that the lemma legitimately relies on.

## All Candidates

### φ1: submap_shrinks_pte_size
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** When a key is in pt but not sub_pt, UnmapWaiting's pte_size drops to 0 — the lemma exploits this monotonicity (smaller regions overlap less), but the zero-size region means the core's operation becomes invisible to overlap checks on the submap

### φ2: no_overlap_vacuous_on_empty_values
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `no_overlap_vmem_values` is vacuously true for empty core_states — tests the base case; if this didn't hold, no initial state could satisfy the invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty map has no values, so the universal quantifier in `no_overlap_vmem_values` is vacuously satisfied. This is correct and expected — the invariant must hold for the initial empty state.

### φ3: submap_preserves_with_empty_submap
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Using empty submap makes all UnmapWaiting/UnmapExecuting pte_sizes 0, effectively hiding all unmap operations from overlap detection — the invariant degenerates but the lemma still claims no-overlap holds
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With an empty submap, all `pte_size` calls for UnmapWaiting/UnmapExecuting return 0, producing zero-size regions. The overlap check with smaller-or-equal regions is a correct consequence of the monotonicity argument — no-overlap on the supermap implies no-overlap on any submap. This is the intended behavior of the lemma.

### φ4: unique_corestates_not_preserved_by_submap
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two cores with identical MapWaiting states violate uniqueness — tests that `unique_CoreStates` correctly rejects duplicate non-Idle entries; if it accepted them, the no-overlap argument would break

### φ5: overlap_monotone_in_size
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The lemma's proof implicitly relies on overlap being monotone in region sizes — if overlap weren't monotone, the submap-to-supermap reasoning would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Overlap monotonicity in sizes follows from the definition: if `base2 < base1 + size1_small` then `base2 < base1 + size1_large` (since `size1_small <= size1_large`), and similarly for the other branch. This is a correct arithmetic property of the overlap definition.

