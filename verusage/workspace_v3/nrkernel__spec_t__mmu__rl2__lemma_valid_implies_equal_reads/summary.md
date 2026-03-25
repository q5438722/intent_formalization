# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_valid_implies_equal_reads/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: zero_size_alignment → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment with size 0 should be undefined/invalid, not trivially true via 0 % 0 == 0 in SMT

### φ2: nonzero_zero_size_alignment → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Any nonzero address being aligned to size 0 would indicate the spec fails to guard against zero divisor

### φ3: upward_alignment_implication → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** 4-alignment does not imply 8-alignment (e.g. addr=4), so this being provable would mean the spec is too weak

### φ4: off_by_one_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Shifting an aligned address by 1 must break alignment for any size > 1; provability implies the spec is vacuous

### φ5: product_sum_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Divisibility by a product does not imply divisibility by the sum (e.g. 6 % 6 == 0 but 6 % 5 != 0)

