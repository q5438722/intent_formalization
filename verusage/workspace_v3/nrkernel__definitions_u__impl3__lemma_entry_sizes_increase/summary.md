# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__definitions_u__impl3__lemma_entry_sizes_increase/original.rs`
**Date:** 2026-03-25 04:08:14
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One genuine spec weakness was found: the `inv()` predicate admits an empty `Arch` (zero layers), which is not a meaningful MMU architecture. Adding a `self.layers.len() > 0` conjunct would close this gap. The other candidate (vacuous truth of `entry_size_is_next_layer_size` beyond bounds) is benign—it's only ever used under a bounds guard within `inv()` and poses no real specification risk.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** The `inv()` predicate uses `layers.len() <= X86_NUM_LAYERS` and a universally quantified constraint over layers—both vacuously satisfied when `layers` is empty. A valid x86 MMU page table architecture must have at least one layer, so the invariant is too weak; it should require `self.layers.len() > 0`.

## All Candidates

### φ1: inv_allows_empty_arch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An empty architecture with zero layers should not satisfy the invariant, as a valid MMU arch needs at least one layer
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `inv()` predicate uses `layers.len() <= X86_NUM_LAYERS` and a universally quantified constraint over layers—both vacuously satisfied when `layers` is empty. A valid x86 MMU page table architecture must have at least one layer, so the invariant is too weak; it should require `self.layers.len() > 0`.

### φ2: inv_allows_unit_num_entries
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A multi-layer arch where every layer has num_entries==1 is degenerate — page tables provide no fan-out and the hierarchy is pointless

### φ3: inv_allows_equal_consecutive_entry_sizes
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Consecutive layers with identical entry_size means the higher layer doesn't actually subdivide address space, violating the purpose of hierarchical paging

### φ4: entry_size_is_next_layer_size_vacuous_beyond_bounds
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** entry_size_is_next_layer_size is vacuously true for out-of-range indices, which could mask specification errors if callers forget bounds checks
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `entry_size_is_next_layer_size` is a helper spec used only under the guard `i < self.layers.len()` inside `inv()`. Its vacuous truth for out-of-bounds indices is standard implication behavior and has no impact on soundness—callers already enforce bounds via `inv()`.

### φ5: inv_allows_top_entry_size_one
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A full 4-layer arch whose top-level entry_size is 1 byte maps only 1 byte total — far too small for any real x86 page table hierarchy

