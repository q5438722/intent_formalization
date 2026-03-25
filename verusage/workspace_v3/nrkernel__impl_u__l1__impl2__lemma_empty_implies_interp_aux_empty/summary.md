# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_empty_implies_interp_aux_empty/original.rs`
**Date:** 2026-03-25 04:08:33
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One candidate property was evaluated. The `inv_allows_zero_layers` property is a true positive: `Arch::inv()` is too weak because it permits a zero-layer architecture (the forall constraint is vacuously satisfied). While `Directory::well_formed` independently prevents constructing directories with such an architecture (requiring `layer < layers.len()`), the `Arch` invariant itself should exclude this degenerate case. The practical severity is low since downstream consumers add their own guards, but it represents a genuine gap in the specification of `Arch::inv`.

## True Positives (Spec Issues)

### inv_allows_zero_layers
- **Confidence:** medium
- **Reasoning:** `Arch::inv()` uses `self.layers.len() <= X86_NUM_LAYERS` which admits length 0, and the universal quantifier over layers is vacuously true when `layers` is empty. A zero-layer architecture is architecturally meaningless for x86 page tables (there must be at least one level of translation), yet `inv()` accepts it. This is a spec weakness—`inv` should require `self.layers.len() > 0` to prevent degenerate instances. However, `Directory::well_formed` requires `self.layer < self.arch.layers.len()`, which means no `Directory` can actually be constructed with a zero-layer `Arch`, so the practical impact is limited to `Arch` being used standalone without a `Directory`.

## All Candidates

### φ1: inv_allows_zero_layers
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-layer architecture satisfying inv() would allow degenerate page table structures that map no addresses
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Arch::inv()` uses `self.layers.len() <= X86_NUM_LAYERS` which admits length 0, and the universal quantifier over layers is vacuously true when `layers` is empty. A zero-layer architecture is architecturally meaningless for x86 page tables (there must be at least one level of translation), yet `inv()` accepts it. This is a spec weakness—`inv` should require `self.layers.len() > 0` to prevent degenerate instances. However, `Directory::well_formed` requires `self.layer < self.arch.layers.len()`, which means no `Directory` can actually be constructed with a zero-layer `Arch`, so the practical impact is limited to `Arch` being used standalone without a `Directory`.

### φ2: entry_base_non_injective
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Non-injective entry bases would allow distinct page table entries to map overlapping virtual address regions

### φ3: consecutive_entries_not_contiguous
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Non-contiguous entry bases would leave unmapped gaps in the virtual address space between consecutive entries

### φ4: page_frame_size_mismatch
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Page frames with sizes mismatching the layer entry size would cause incorrect address translation boundaries

### φ5: subdir_bases_overlap
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subdirectories sharing the same base virtual address would cause ambiguous overlapping translations in the page table walk

