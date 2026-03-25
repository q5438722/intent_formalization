# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__os_refinement__lemma_inflight_vaddr_implies_hl_unmap_or_map/original.rs`
**Date:** 2026-03-25 04:13:09
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property identifies a vacuous-truth edge case in `Arch::inv` where an empty layer sequence passes the invariant. While mathematically accurate, this targets a ghost spec function rather than any executable function's contract, and the only concrete `Arch` instance (`x86_arch_spec`) always has exactly 4 layers, making the degenerate case unreachable in practice. No true positives found.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment to size 0 is meaningless; if entailed, any address is trivially "aligned" to nothing, breaking alignment guarantees downstream.

### φ2: inv_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A degenerate architecture with zero layers should not satisfy the invariant, as it cannot describe any valid address translation.
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Arch::inv` is a pure spec function (`open spec(checked) fn`), not an executable function. While the observation that an empty architecture vacuously satisfies `inv()` is technically valid, it targets a ghost specification rather than an executable function's requires/ensures. Additionally, `x86_arch_spec` is the only `Arch` instance used in the codebase and it has 4 layers, so this degenerate case is never reachable.

### φ3: upper_vaddr_equals_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the upper virtual address equals the base for a valid architecture layer, the mapped region has zero size, meaning no addresses can be translated.

### φ4: entry_base_non_monotone
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Entry base addresses must be strictly increasing with index; if a later index maps to a lower-or-equal address, entries overlap or go backwards.

### φ5: entry_size_is_next_layer_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A single-layer valid architecture having entry_size of 1 byte would be far too small for any real page size, suggesting the invariant fails to lower-bound leaf entry sizes.

