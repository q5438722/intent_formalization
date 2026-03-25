# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__indexing__lemma_entry_base_from_index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_trivially_true → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If 1 is aligned to 2 the spec vacuously accepts non-multiples, making alignment meaningless.

### φ2: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) holds for nonzero addr the spec fails to guard against division-by-zero undefined behavior.

### φ3: aligned_successor → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a+1 is always aligned whenever a is, alignment collapses to a trivial property for every size greater than 1.

### φ4: aligned_collapses_values → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Many distinct naturals are aligned to the same size; alignment must not force equality of all aligned addresses.

### φ5: aligned_over_transitive → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Transitivity gives aligned(a,c) not aligned(a,b*c); conflating the two would unsoundly strengthen alignment guarantees.

