# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_nonempty_implies_interp_contains/original.rs`
**Date:** 2026-03-25 04:12:04
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One verified property was analyzed. `inv_allows_zero_layers` is a true positive: `Arch::inv()` admits zero-layer architectures due to a vacuously true quantifier, even though such architectures are meaningless for page table translation. While no `Directory` can be `well_formed` under a zero-layer `Arch` (providing some indirect protection), the invariant itself is weaker than intended and should explicitly require at least one layer.

## True Positives (Spec Issues)

### inv_allows_zero_layers
- **Confidence:** high
- **Reasoning:** `Arch::inv()` is the central invariant gating all directory operations. It permits a zero-layer architecture because the universal quantifier over layers is vacuously true and `0 <= X86_NUM_LAYERS`. A zero-layer `Arch` is degenerate—no `Directory` can satisfy `well_formed()` (which requires `self.layer < self.arch.layers.len()`), making such an architecture unusable for translation. The spec should include `self.layers.len() > 0` to rule out this vacuous case.

## All Candidates

### φ1: inv_allows_zero_layers
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Arch::inv should require at least one layer; a zero-layer architecture is degenerate and unusable for address translation
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` is the central invariant gating all directory operations. It permits a zero-layer architecture because the universal quantifier over layers is vacuously true and `0 <= X86_NUM_LAYERS`. A zero-layer `Arch` is degenerate—no `Directory` can satisfy `well_formed()` (which requires `self.layer < self.arch.layers.len()`), making such an architecture unusable for translation. The spec should include `self.layers.len() > 0` to rule out this vacuous case.

### φ2: entry_base_overlap
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Distinct entry indices at the same layer must map to distinct base addresses; overlap would corrupt address translation

### φ3: interp_of_entry_key_overlap
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two different directory entries should never map the same virtual address; overlapping keys would cause ambiguous translations

### φ4: well_formed_allows_zero_entries
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A well-formed directory with zero entries is degenerate; well_formed should guarantee at least one slot via arch.inv

### φ5: contains_zero_entry_size
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Under a valid arch invariant all entry sizes are positive, so finding an entry_size of 0 would indicate an inconsistent spec

