# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__hlspec__next_step_preserves_inv/original.rs`
**Date:** 2026-03-25 04:14:33
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate property identifies a genuine spec weakness: `Arch::inv()` admits a zero-layer architecture, which is a degenerate/meaningless configuration for a page table. The invariant's universal quantifier over layers is vacuously true when `layers` is empty, so any downstream reasoning that assumes `inv()` guarantees at least one usable layer (and thus a nonzero address space) is potentially unsound. The fix would be to add `self.layers.len() > 0` (or `self.layers.len() >= 1`) to `inv()`. The practical impact is limited since the system uses the concrete `x86_arch_spec` with 4 layers, but for the general `Arch` abstraction this is a missing precondition.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** medium
- **Reasoning:** `Arch::inv()` is vacuously true for an empty layers sequence because both the length bound (`0 <= 4`) and the universal quantifier over layers are trivially satisfied. Any proof or function that assumes `arch.inv()` implies at least one layer exists (e.g., that `upper_vaddr` produces a meaningful bound, or that valid page sizes exist) could be unsound for the degenerate case. While the concrete `x86_arch_spec` has 4 layers, the invariant is used generically and should rule out the empty architecture.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) is provable for all addr, then any code relying on aligned as a meaningful divisibility check is unsound when size is zero

### φ2: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-layer architecture is degenerate and meaningless; if inv() accepts it, downstream code assuming at least one layer may be unsound
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Arch::inv()` is vacuously true for an empty layers sequence because both the length bound (`0 <= 4`) and the universal quantifier over layers are trivially satisfied. Any proof or function that assumes `arch.inv()` implies at least one layer exists (e.g., that `upper_vaddr` produces a meaningful bound, or that valid page sizes exist) could be unsound for the degenerate case. While the concrete `x86_arch_spec` has 4 layers, the invariant is used generically and should rule out the empty architecture.

### φ3: upper_vaddr_collapses_to_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals base, the address range for a layer is empty, breaking all bounds checks that rely on upper_vaddr as an exclusive upper bound

### φ4: entry_base_not_strictly_monotone
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a higher index yields a lower-or-equal entry base, page table entries would overlap or go backwards, corrupting the address space layout

### φ5: entry_size_increases_at_deeper_layer
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a parent layer's entry size is smaller than its child's, the hierarchical decomposition is inverted and page table walks would be inconsistent

