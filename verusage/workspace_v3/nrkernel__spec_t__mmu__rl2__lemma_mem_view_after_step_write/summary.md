# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_mem_view_after_step_write/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size_vacuous → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment with size 0 is vacuously true, any guard using `aligned(x, 0)` silently passes, admitting invalid states.

### φ2: aligned_off_by_one → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If incrementing an 8-aligned address by 1 preserves alignment, the alignment predicate is trivially true and all ptmem write guards are vacuous.

### φ3: page_align_not_word_align → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Page-alignment (4096) must imply 8-byte alignment since 4096 is a multiple of 8; if it doesn't, pml4 alignment invariants fail to guarantee write alignment.

### φ4: aligned_cross_divisor → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** 8-alignment should not entail 3-alignment (gcd(8,3)=1); if it does the modular semantics of `aligned` are broken and non-power-of-two checks become unsound.

### φ5: aligned_below_size_wrong_value → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The only nat below 4096 that is 4096-aligned is 0; if the solver derives addr==1, the modular-arithmetic definition of `aligned` is inconsistent.

