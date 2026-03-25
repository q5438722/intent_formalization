# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__aligned_transitive_auto/original.rs`
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
- **Why flagged:** aligned with size 0 should be meaningless/false; if true, the spec fails to guard against division by zero

### φ2: odd_aligned_to_even → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** 1 is odd and cannot be 2-aligned; if provable the alignment definition is unsound

### φ3: aligned_implies_double_size → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** alignment to b does not imply alignment to 2*b (e.g. 4 is 4-aligned but not 8-aligned); would signal the spec is too weak

### φ4: adjacent_preserves_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** if a is b-aligned then a+1 must not be (for b>1); provability would mean alignment is trivially true

### φ5: all_addresses_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** not every address is aligned to every nonzero size; if provable the spec is vacuously true and useless

