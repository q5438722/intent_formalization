# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__os_refinement__interp_vmem_update_range/original.rs`
**Date:** 2026-03-25 04:12:23
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property targets `Arch::inv()`, which is a pure spec function on a ghost struct, not an executable function's specification. While it's technically true that an empty-layer architecture vacuously satisfies `inv()`, the codebase only ever instantiates `Arch` as `x86_arch_spec` with 4 layers. This is a specification design choice (keeping `inv` general) rather than a real spec gap, and the function is not executable code, making this a false positive.

## All Candidates

### φ1: upper_vaddr_no_progress
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals the base, the layer maps a zero-sized address range, making page table traversal useless

### φ2: entry_base_collapses_indices
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Distinct entry indices mapping to the same base address would mean page table entries alias, breaking address space partitioning

### φ3: inv_allows_empty_arch
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers satisfies inv vacuously, yet cannot perform any translation—the invariant may be too weak
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` is a spec-level invariant on a ghost struct — it's not an executable function's specification. Moreover, the concrete `x86_arch_spec` always has 4 layers, so the vacuous satisfaction for empty architectures is irrelevant in practice; the invariant is only used in contexts where the architecture is instantiated to `x86_arch_spec`.

### φ4: entry_size_non_decreasing_across_layers
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Parent layers must map strictly larger regions than child layers; equal-or-smaller sizes would break the hierarchical decomposition of the address space

### φ5: num_entries_allows_degenerate
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A layer with at most one entry makes the directory level pointless and would collapse the page table hierarchy, indicating the invariant permits degenerate configurations

