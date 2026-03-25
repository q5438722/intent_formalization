# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__indexing__lemma_entry_base_from_index_support/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned with size 0 involves division by zero and should never hold; provability here indicates unsound handling of modulo-zero

### φ2: alignment_doubles → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** alignment to b does not imply alignment to 2*b (e.g. aligned(4,4) but not aligned(4,8)); provability means the spec is too strong

### φ3: shift_one_preserves_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** adding 1 to an aligned address must break alignment for any size > 1; provability reveals a broken modular-arithmetic spec

### φ4: product_size_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** aligned to b and c individually does not imply aligned to b*c (e.g. aligned(12,4) and aligned(12,6) but not aligned(12,24)); provability signals a flawed compositionality assumption

### φ5: nonmultiple_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 1 is never a multiple of any b > 1 so aligned(1, b) must be false; provability means the spec incorrectly classifies non-aligned addresses as aligned

