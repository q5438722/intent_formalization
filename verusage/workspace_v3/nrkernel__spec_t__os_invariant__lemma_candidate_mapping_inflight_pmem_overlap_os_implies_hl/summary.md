# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__lemma_candidate_mapping_inflight_pmem_overlap_os_implies_hl/original.rs`
**Date:** 2026-03-25 04:16:01
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `inv_allows_empty_arch` identifies that `Arch::inv()` is vacuously satisfied by a zero-layer architecture. While this is a valid observation about the predicate's boundary behavior, it does not constitute a real spec gap for executable code: `Arch` and `inv()` are purely ghost/spec constructs, the concrete architecture `x86_arch_spec` always has 4 layers, and no executable function's contract is affected. The finding is a false positive.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) is always true, any address is vacuously "aligned" to size zero, which is meaningless and could mask alignment-check bugs.

### φ2: inv_allows_empty_arch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If an architecture with zero layers satisfies inv(), the invariant is too weak—an empty page table hierarchy is nonsensical and would let downstream proofs vacuously hold.
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` is a pure `spec` function, not an executable function's specification. The entire file consists of ghost/spec-level definitions and proof functions with no executable (`fn`/`exec`) code. While an empty-layer `Arch` satisfying `inv()` is technically vacuous, this is constrained in practice by `x86_arch_spec` being the only `Arch` value used, and the property targets spec-level predicates rather than any executable function's requires/ensures.

### φ3: upper_vaddr_equals_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals the base, the mapped address range has zero size, meaning the layer maps nothing—defeating the purpose of the page table level.

### φ4: entry_base_ignores_base
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** entry_base at index 0 must equal the base address; if it doesn't, the arithmetic is broken and all derived virtual addresses would be shifted incorrectly.

### φ5: child_layer_entry_not_smaller
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** In a well-formed page table hierarchy, inner layers must map strictly smaller regions than outer layers; if a child layer's entry_size is >= the parent's, the hierarchical decomposition is violated.

