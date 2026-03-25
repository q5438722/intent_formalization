# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__os_refinement__lemma_unmap_soundness_equality/original.rs`
**Date:** 2026-03-25 04:13:40
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate φ demonstrates that `Arch { layers: seq![] }` vacuously satisfies the `inv()` predicate due to the universal quantifier over layers being trivially true for an empty sequence. While this is a valid logical observation, it targets a spec-level definition (`open spec fn inv`) rather than any executable function's requires/ensures clause, and the only concrete `Arch` instance used in the system (`x86_arch_spec`) has exactly 4 layers. The gap has no practical impact on the soundness of the verified system.

## All Candidates

### φ1: aligned_zero_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment with size 0 is satisfiable, it masks division-by-zero in any spec that relies on aligned without guarding size > 0

### φ2: inv_empty_arch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers is vacuously valid under inv, which would allow meaningless architectures to satisfy all downstream specs
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** The property targets `Arch::inv`, which is a pure spec function, not an executable function's specification. While the observation that an empty `Arch` vacuously satisfies `inv()` is technically correct, this is benign in practice: `x86_arch_spec` is the only `Arch` instance used downstream and it has 4 layers. The empty-arch case never arises in any reachable state or executable code path.

### φ3: upper_vaddr_no_progress
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr never exceeds base, the layer covers zero address space, making all entry-size and mapping reasoning unsound

### φ4: entry_base_collision
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Distinct entry indices mapping to the same base address would collapse the entire page table structure, making separate pages indistinguishable

### φ5: aligned_implies_double_aligned
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment to size does not imply alignment to 2*size; if provable it would mean every 4K-aligned address is also 8K-aligned, breaking page-size distinctions

