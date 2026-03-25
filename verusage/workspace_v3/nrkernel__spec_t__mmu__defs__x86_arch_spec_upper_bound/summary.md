# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__defs__x86_arch_spec_upper_bound/original.rs`
**Date:** 2026-03-25 04:15:04
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

Of the five candidates, only **inv_allows_empty_arch** is a genuine true positive: the `inv()` predicate should require at least one layer (`self.layers.len() > 0`) since an empty architecture is meaningless. The remaining four are false positives. The non-page-aligned and single-entry observations reflect deliberate generality in the `Arch` abstraction (the concrete `x86_arch_spec` is correctly constrained). The `upper_vaddr_exceeds_max_base` conflates a named constant with a universal bound — `MAX_BASE` is just the zero-base upper address. The vacuous `entry_size_is_next_layer_size` for a single-layer arch is logically correct and doesn't constitute a separate spec gap beyond the empty-arch issue.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** An `Arch` with zero layers vacuously satisfies the forall in `inv()` and `0 <= X86_NUM_LAYERS`. This is a genuine spec weakness — an empty architecture represents no address translation and should be excluded. The invariant should require `self.layers.len() > 0`.

## All Candidates

### φ1: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers should not satisfy the invariant, as it represents no address translation at all
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** An `Arch` with zero layers vacuously satisfies the forall in `inv()` and `0 <= X86_NUM_LAYERS`. This is a genuine spec weakness — an empty architecture represents no address translation and should be excluded. The invariant should require `self.layers.len() > 0`.

### φ2: inv_allows_non_page_aligned_entry_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An entry_size of 3 bytes is nonsensical for page tables; inv should enforce entry sizes are multiples of PAGE_SIZE
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** While entry_size=3 looks nonsensical, `inv()` is intentionally a generic architectural invariant parameterized over `Arch`, not hardcoded to x86 page sizes. The concrete x86 arch is `x86_arch_spec` which uses correct sizes. The invariant's role is structural (positive sizes, bounded, hierarchical decomposition), not enforcing page alignment — that's a design choice, not a spec gap.

### φ3: upper_vaddr_exceeds_max_base
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** upper_vaddr with any nonzero base at layer 0 exceeds MAX_BASE, suggesting the spec lacks a base-range guard
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `upper_vaddr` computes `base + num_entries * entry_size`, so with base=1 it naturally exceeds `MAX_BASE` (which equals `upper_vaddr(0,0)`). The function has only `recommends` (not `requires`) preconditions, and `MAX_BASE` is just a named constant for the zero-base case. There's no spec gap — callers are expected to use valid bases; the `recommends` clause signals this.

### φ4: inv_allows_single_entry_layer
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A layer with num_entries=1 is degenerate and provides no fan-out; inv should enforce num_entries > 1
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** A layer with `num_entries=1` is architecturally valid in the general `Arch` abstraction — it could represent a single-entry root level. The invariant requires `0 < num_entries <= 512`, which is a deliberate bound. Requiring `> 1` would be an unnecessary restriction on the generic architecture type; the concrete `x86_arch_spec` uses 512 everywhere.

### φ5: entry_size_is_next_layer_vacuously_true
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** For a single-layer arch the implication is vacuously true, allowing arbitrary entry_size values that bypass the hierarchical size constraint
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the same underlying observation as `inv_allows_empty_arch` and `inv_allows_non_page_aligned_entry_size`. For a single-layer arch, the last layer's `entry_size_is_next_layer_size` is vacuously true by design — there's no next layer to constrain against. This is correct behavior for an implication over a non-existent successor; the real issue (if any) is the empty-arch case already captured by φ1.

