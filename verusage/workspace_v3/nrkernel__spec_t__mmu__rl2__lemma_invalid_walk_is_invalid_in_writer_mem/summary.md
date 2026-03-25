# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_invalid_walk_is_invalid_in_writer_mem/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Downward alignment must never return a value exceeding the original input

### φ2: align_to_one_changes_value → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 should be a no-op since every integer is 1-aligned; a changed value would indicate a broken spec

### φ3: nonzero_align_stays_nonzero → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A nonzero address aligned down can legitimately become zero (e.g., a=1 b=4096); if the spec claims otherwise it is too strong

### φ4: aligned_additive_divisors → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to two sizes does not imply alignment to their sum (e.g., 12 is aligned to 4 and 6 but not to 10)

### φ5: align_result_double_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Aligning down to b does not guarantee the result is also aligned to 2b (e.g., align_to_usize(5,4)==4 but 4 is not 8-aligned)

