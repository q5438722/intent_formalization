# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__pigeonhole_too_many_elements_implies_double.rs`
**Date:** 2026-03-24T11:41:55Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pigeonhole_missing_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `pigeonhole_missing_idx_implies_double` is `external_body` — the entire pigeonhole principle is trusted without proof

### φ2: pigeonhole_len_0
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** With `len == 0`, we need `m[0] < 0` for `nat` which is impossible — the precondition is vacuously false, so the lemma proves anything

### φ3: pigeonhole_dup_in_range
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Both duplicate indices are in `[0, len+1)` — follows from `m.dom().contains(i/j)` and the domain biconditional, but the ensures doesn't state this explicitly

### φ4: pigeonhole_dup_value_range
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The duplicated value is within `[0, len)` — follows from the precondition on `m`'s values, confirming the collision is among valid target values

### φ5: pigeonhole_missing_returns_not_missing
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The duplicate value avoids the missing index — follows from the precondition but confirms the external_body-trusted pigeonhole correctly respects the exclusion

