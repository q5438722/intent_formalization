# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__subtract_mod_eq_zero/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: trivially_aligned_all → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every address is aligned to every nonzero size, the spec is vacuously trivial and useless

### φ2: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Nonzero addresses should not be considered aligned to size 0, which is a degenerate division-by-zero case

### φ3: alignment_survives_increment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Incrementing an aligned address by 1 should break alignment for any size > 1

### φ4: aligned_odd_to_even → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 7 is not a multiple of 4; if the spec says it is aligned, modular arithmetic is broken

### φ5: alignment_implies_double_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Being aligned to size does not imply alignment to 2*size (e.g., 4 is aligned to 4 but not to 8)

