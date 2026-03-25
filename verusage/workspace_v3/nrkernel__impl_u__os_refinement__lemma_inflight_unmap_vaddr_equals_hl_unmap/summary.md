# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__os_refinement__lemma_inflight_unmap_vaddr_equals_hl_unmap/original.rs`
**Date:** 2026-03-25 04:12:44
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate property exposes a genuine weakness in `Arch::inv()`: it admits an empty architecture with no layers. Since `inv()` gates well-formedness reasoning throughout the MMU specification (translation bounds, entry sizes, address space limits), accepting a degenerate zero-layer `Arch` is a meaningful spec gap. A minimum-layers constraint (e.g., `self.layers.len() > 0` or equality with `X86_NUM_LAYERS`) should be added to tighten the invariant.

## True Positives (Spec Issues)

### inv_empty_arch
- **Confidence:** high
- **Reasoning:** The `Arch::inv()` predicate is satisfied by an architecture with zero layers because `layers.len() <= X86_NUM_LAYERS` holds (0 ≤ 4) and the universal quantifier over layer indices is vacuously true. This means any code relying on `inv()` to guarantee a meaningful architecture (e.g., `upper_vaddr` producing a nonzero address space) could be unsound. The invariant should include `self.layers.len() > 0` or a similar lower bound.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment to size 0 is trivially true for all addresses, the spec lacks a guard against degenerate zero-size alignment

### φ2: inv_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers is degenerate and should not satisfy the well-formedness invariant
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `Arch::inv()` predicate is satisfied by an architecture with zero layers because `layers.len() <= X86_NUM_LAYERS` holds (0 ≤ 4) and the universal quantifier over layer indices is vacuously true. This means any code relying on `inv()` to guarantee a meaningful architecture (e.g., `upper_vaddr` producing a nonzero address space) could be unsound. The invariant should include `self.layers.len() > 0` or a similar lower bound.

### φ3: upper_vaddr_no_progress
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals base under a valid invariant, the layer covers zero address space, making the architecture useless

### φ4: entry_base_collapses_indices
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two distinct indices map to the same entry base, page table entries would alias and the address space partitioning is broken

### φ5: entry_size_next_layer_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the first and second layer have equal entry sizes despite the decomposition property requiring entry_size(0) == entry_size(1) * num_entries(1) with num_entries > 1, the invariant would be contradictory or num_entries is forced to 1

