# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_insert/original.rs`
**Date:** 2026-03-25 04:08:56
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One candidate property was evaluated. The `inv_allows_empty_arch` property is a true positive: `Arch::inv()` is vacuously satisfied by an architecture with zero layers because its body is a bounded universal quantifier that holds trivially on an empty sequence. Since the x86 page table walk requires at least one layer to be meaningful, and downstream code (e.g., `Directory::well_formed` requires `self.layer < self.arch.layers.len()`) assumes non-empty architectures, the invariant should explicitly require `self.layers.len() > 0`.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers `i < self.layers.len()`, which is vacuously true when `layers.len() == 0`. This means an empty architecture satisfies `inv()`, yet an empty arch is degenerate—it has no layers, no entry sizes, and no meaningful page table structure. The invariant should require `self.layers.len() > 0` to exclude this case. This is a real spec weakness: any code relying on `arch.inv()` to guarantee a usable architecture could receive an empty one.

## All Candidates

### φ1: inv_allows_empty_arch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers is degenerate and should not satisfy the well-formedness invariant
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers `i < self.layers.len()`, which is vacuously true when `layers.len() == 0`. This means an empty architecture satisfies `inv()`, yet an empty arch is degenerate—it has no layers, no entry sizes, and no meaningful page table structure. The invariant should require `self.layers.len() > 0` to exclude this case. This is a real spec weakness: any code relying on `arch.inv()` to guarantee a usable architecture could receive an empty one.

### φ2: entry_base_ignores_index
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entry_base returns the same value regardless of index, all entries would map to the same address causing total overlap

### φ3: inv_page_size_mismatch
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If page frame sizes are allowed to differ from the layer's entry size under inv, address translation would map incorrect regions

### φ4: inv_entry_size_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero-sized entries would collapse the entire address space and break entry_base monotonicity and page mapping

### φ5: subdir_base_equals_parent_base
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If non-zero-index subdirectories share the parent's base_vaddr, their mapped regions would overlap with index-0's region

