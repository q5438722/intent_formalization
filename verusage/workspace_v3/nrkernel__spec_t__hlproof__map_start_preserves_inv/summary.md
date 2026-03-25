# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__hlproof__map_start_preserves_inv/original.rs`
**Date:** 2026-03-25 04:14:02
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property identifies a real but low-severity spec weakness: `Arch::inv()` is vacuously satisfied by a zero-layer architecture because the universal quantifier over layers is trivially true when there are none. However, since `inv()` is a pure spec function (not an exec function's pre/postcondition), this does not meet the TRUE_POSITIVE criteria. In practice, this is mitigated by the fact that the only `Arch` value actually used (`x86_arch_spec`) has 4 layers, so the degenerate case never arises in real verification paths.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned with size 0 should be rejected or undefined, not vacuously true via 0%0==0

### φ2: empty_arch_satisfies_inv
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers is degenerate and should not satisfy the well-formedness invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` is a `pub open spec` function, not an executable function. While the observation that an empty-layer architecture vacuously satisfies `inv()` is a genuine spec weakness, the critic criteria require TRUE_POSITIVE to target an executable function's specification (requires/ensures). This targets a pure spec-level predicate.

### φ3: upper_vaddr_not_above_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** upper_vaddr should always exceed base since num_entries>0 and entry_size>0 under inv; equality or less would mean zero or negative address span

### φ4: entry_base_collapses_distinct_indices
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Different indices must yield different entry bases; collision would mean overlapping page table entries

### φ5: entry_size_next_layer_inconsistent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Under inv the layer size decomposition must hold; if this verifies the invariant is internally contradictory

