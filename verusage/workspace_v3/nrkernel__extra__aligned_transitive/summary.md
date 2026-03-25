# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__aligned_transitive/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: everything_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean every address is aligned to every size, making the spec vacuously permissive

### φ2: only_zero_aligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean only zero can ever be aligned, collapsing all non-trivial alignment to false

### φ3: odd_aligned_to_even → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 1 is not a multiple of 2, so claiming 1 is 2-aligned indicates a broken modulus check

### φ4: bad_product_composition → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** aligned(12,4) && aligned(12,6) does not imply aligned(12,24); conflates LCM with product

### φ5: aligned_symmetric → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned(4,2) is true but aligned(2,4) is false; symmetry would indicate modulus is broken

