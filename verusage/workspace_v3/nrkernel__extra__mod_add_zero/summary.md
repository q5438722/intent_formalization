# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__mod_add_zero/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: all_addrs_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every address is trivially aligned to any positive size, the spec is vacuously weak and useless as a constraint.

### φ2: succ_preserves_alignment → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment to a size > 1 must not be preserved by incrementing by 1; otherwise alignment is meaningless.

### φ3: double_size_aligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Being aligned to size should not automatically imply alignment to 2*size (e.g., 4 is aligned to 4 but not to 8).

### φ4: only_zero_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the only aligned address were 0, the spec would be over-constrained; multiples of size must also be aligned.

### φ5: aligned_implies_one_aligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** 1 should never be aligned to any size > 1; if it were, the modular arithmetic in the spec would be broken.

