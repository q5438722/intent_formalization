# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__definitions_u__impl3__lemma_entry_sizes_aligned/original.rs`
**Date:** 2026-03-25 04:07:27
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four of five properties are true positives revealing that `Arch::inv()` is too permissive: it allows empty architectures (no layers), degenerate 1-byte pages, non-hierarchical layers with equal entry sizes, and leaf layers mapping 512 GB. The common theme is missing lower bounds — on layer count, minimum leaf entry size, and `num_entries > 1` for multi-layer hierarchies. The `aligned(0, s)` property is a false positive since zero alignment is mathematically correct and not a spec deficiency.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** The `inv()` predicate uses `<=` for layer count and a vacuously-true universal quantifier over an empty sequence, so an `Arch` with zero layers satisfies it. A real x86 MMU architecture must have at least one layer (typically 4), so `inv` is too weak — it should require `self.layers.len() > 0`.

### inv_allows_unit_entry_size
- **Confidence:** medium
- **Reasoning:** The invariant permits `entry_size: 1` and `num_entries: 1`, which represents a 1-byte page with a single entry — physically impossible on x86 where the minimum page size is 4K. The `inv` should enforce a minimum entry size (e.g., 4096) for the leaf layer to reflect actual hardware constraints.

### inv_allows_equal_consecutive_entry_sizes
- **Confidence:** high
- **Reasoning:** When `num_entries` is 1, the constraint `entry_size(i) == entry_size(i+1) * num_entries(i+1)` degenerates to equal entry sizes across layers. This violates the hierarchical structure where each deeper layer must map a strictly smaller region. The invariant should require `num_entries > 1` or `entry_size` strictly decreasing.

### last_layer_max_entry_size
- **Confidence:** medium
- **Reasoning:** A single-layer architecture where each entry maps 512 GB is accepted by `inv`, but on x86 the leaf-level page size is 4K (or at most 1G for huge pages). The invariant's upper bound `X86_MAX_ENTRY_SIZE` is meant for the top layer, not for leaf entries. Without constraining the last layer's entry size to realistic page sizes, the spec is too permissive.

## All Candidates

### φ1: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers should not satisfy the invariant, as a valid MMU must have at least one page-table layer.
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `inv()` predicate uses `<=` for layer count and a vacuously-true universal quantifier over an empty sequence, so an `Arch` with zero layers satisfies it. A real x86 MMU architecture must have at least one layer (typically 4), so `inv` is too weak — it should require `self.layers.len() > 0`.

### φ2: aligned_zero_to_any
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Address zero being trivially aligned to every size could allow degenerate zero-address mappings to bypass alignment checks throughout the MMU spec.
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % s == 0` for any `s > 0` is a standard mathematical fact. Address zero being aligned is correct and expected — the `aligned` function is a pure math predicate, and any MMU policy restricting address zero would be enforced elsewhere, not in the alignment definition.

### φ3: inv_allows_unit_entry_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A single-layer architecture with 1-byte entry size and 1 entry is degenerate and should be rejected; x86 pages are at least 4K.
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The invariant permits `entry_size: 1` and `num_entries: 1`, which represents a 1-byte page with a single entry — physically impossible on x86 where the minimum page size is 4K. The `inv` should enforce a minimum entry size (e.g., 4096) for the leaf layer to reflect actual hardware constraints.

### φ4: inv_allows_equal_consecutive_entry_sizes
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Consecutive layers having identical entry sizes (via num_entries=1) defeats the hierarchical structure; each deeper layer should map a strictly smaller region.
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** When `num_entries` is 1, the constraint `entry_size(i) == entry_size(i+1) * num_entries(i+1)` degenerates to equal entry sizes across layers. This violates the hierarchical structure where each deeper layer must map a strictly smaller region. The invariant should require `num_entries > 1` or `entry_size` strictly decreasing.

### φ5: last_layer_max_entry_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The finest-granularity (and only) layer mapping 512 GB per entry is unrealistic for x86 paging; leaf-layer entry size should be bounded to 4K or at most 2M/1G.
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** A single-layer architecture where each entry maps 512 GB is accepted by `inv`, but on x86 the leaf-level page size is 4K (or at most 1G for huge pages). The invariant's upper bound `X86_MAX_ENTRY_SIZE` is meant for the top layer, not for leaf entries. Without constraining the last layer's entry size to realistic page sizes, the spec is too permissive.

