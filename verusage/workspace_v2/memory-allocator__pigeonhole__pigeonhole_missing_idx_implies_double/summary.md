# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__pigeonhole_missing_idx_implies_double.rs`
**Date:** 2026-03-24T11:40:23Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pigeonhole_helper_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `pigeonhole_missing_idx_implies_double_helper` is `external_body` — the core pigeonhole argument (finding a duplicate) is entirely trusted without proof

### φ2: pigeonhole_len_2_trivial
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** With `len == 2` and `missing == 0`, both `m[0]` and `m[1]` must equal 1 — the simplest pigeonhole case relies on the external_body helper

### φ3: helper_prev_vals_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The helper's `prev_vals` precondition only requires `exists |j| 0 <= j < k && m[j] == elt` for contained elements — a set containing value 42 (outside `[0, len)`) satisfies this vacuously if no such j exists, allowing the external_body to be called with semantically bogus accumulator state

### φ4: pigeonhole_returns_in_range
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Both returned indices are in `[0, len)` — the ensures of `pigeonhole_missing_idx_implies_double` guarantees `m.dom().contains(i)` and `m.dom().contains(j)` but doesn't explicitly state `0 <= i < len` and `0 <= j < len`; this is derived from the domain biconditional

### φ5: pigeonhole_shared_value_not_missing
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The duplicated value is never the missing index — follows from the precondition, but confirms the pigeonhole maps `len` keys into `len - 1` values (excluding `missing`)

