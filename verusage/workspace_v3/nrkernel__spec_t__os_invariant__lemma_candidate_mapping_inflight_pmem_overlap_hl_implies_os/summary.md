# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__lemma_candidate_mapping_inflight_pmem_overlap_hl_implies_os/original.rs`
**Date:** 2026-03-25 04:15:26
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate φ (`inv_allows_empty_arch`) is a true positive. It exposes a real gap in `Arch::inv()`: the invariant is satisfiable by a zero-layer architecture because all its conjuncts are either vacuously true or trivially satisfied when `layers.len() == 0`. Since the entire MMU specification—address translation, entry sizes, upper virtual address bounds—depends on `Arch` having at least one layer, any proof that assumes `arch.inv()` implies meaningful translation structure is working on an under-constrained invariant. In practice this is mitigated because `x86_arch_spec` is hardcoded with 4 layers, but any generic lemma quantifying over arbitrary `Arch` values satisfying `inv()` could be silently vacuous. Adding a `self.layers.len() > 0` conjunct to `inv()` would close the gap.

## True Positives (Spec Issues)

### inv_allows_empty_arch
- **Confidence:** high
- **Reasoning:** `Arch::inv()` only constrains layers via a universal quantifier over `i < self.layers.len()`, which is vacuously true when `layers.len() == 0`. The only other conjunct is `self.layers.len() <= X86_NUM_LAYERS`, which also holds. This means an empty architecture satisfies `inv()` yet has no layers, making `upper_vaddr`, `entry_size`, and all translation logic degenerate. Any proof relying on `arch.inv()` to guarantee meaningful page table structure is unsound for the empty case. The fix would be to add `self.layers.len() > 0` (or `== X86_NUM_LAYERS`) to `inv()`.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment to size 0 is trivially true for all addresses, the spec fails to guard against meaningless zero-size alignment checks

### φ2: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If an architecture with zero layers satisfies inv(), then page table reasoning becomes vacuous and no translations are possible
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` only constrains layers via a universal quantifier over `i < self.layers.len()`, which is vacuously true when `layers.len() == 0`. The only other conjunct is `self.layers.len() <= X86_NUM_LAYERS`, which also holds. This means an empty architecture satisfies `inv()` yet has no layers, making `upper_vaddr`, `entry_size`, and all translation logic degenerate. Any proof relying on `arch.inv()` to guarantee meaningful page table structure is unsound for the empty case. The fix would be to add `self.layers.len() > 0` (or `== X86_NUM_LAYERS`) to `inv()`.

### φ3: upper_vaddr_no_progress
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals base, the layer maps zero address space, making the entire page table level useless

### φ4: entry_size_uniform_across_layers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If adjacent layers have identical entry sizes, the hierarchical decomposition is broken and multi-level translation provides no granularity benefit

### φ5: entry_base_non_monotonic
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a higher index yields a lower-or-equal entry base address, the virtual address space ordering is violated and page table walks cannot correctly partition memory

