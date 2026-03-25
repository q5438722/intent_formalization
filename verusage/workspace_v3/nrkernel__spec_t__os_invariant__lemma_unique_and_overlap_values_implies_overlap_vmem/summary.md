# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__lemma_unique_and_overlap_values_implies_overlap_vmem/original.rs`
**Date:** 2026-03-25 04:16:22
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate property identifies a genuine spec weakness in `Arch::inv()`: the invariant lacks a lower bound on the number of layers, allowing a zero-layer architecture to trivially satisfy it. Since `Arch` and its invariant are used throughout the MMU specification to reason about page table structure and address translation, admitting a degenerate empty architecture could silently weaken downstream proofs that assume at least one translation layer exists. Adding `0 < self.layers.len()` (or even `self.layers.len() == X86_NUM_LAYERS` for the concrete x86 case) would close this gap.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** The `Arch::inv()` spec function only requires `self.layers.len() <= X86_NUM_LAYERS` and a universally quantified condition over layers that is vacuously true when there are zero layers. This means a degenerate zero-layer architecture satisfies the invariant despite being unable to describe any address translation. The spec is too weak — it should require `0 < self.layers.len()`.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) is trivially true for all addr, zero-size alignment checks would silently pass, masking invalid frame sizes

### φ2: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers is degenerate and should not satisfy the invariant, as it cannot describe any address translation
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `Arch::inv()` spec function only requires `self.layers.len() <= X86_NUM_LAYERS` and a universally quantified condition over layers that is vacuously true when there are zero layers. This means a degenerate zero-layer architecture satisfies the invariant despite being unable to describe any address translation. The spec is too weak — it should require `0 < self.layers.len()`.

### φ3: upper_vaddr_leq_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** upper_vaddr should always exceed base for a valid layer, since the mapped region has positive size; equality or less implies an empty or wrapping address space

### φ4: entry_base_zero_idx_neq_base
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** entry_base at index 0 should equal the base itself (base + 0 * entry_size == base); if this proves, the arithmetic identity is broken

### φ5: entry_size_next_layer_equal
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Consecutive layers having equal entry sizes would mean num_entries is 1, collapsing the page table hierarchy and defeating multi-level translation

