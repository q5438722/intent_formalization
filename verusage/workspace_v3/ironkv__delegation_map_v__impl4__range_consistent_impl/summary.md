# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl4__range_consistent_impl/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: is_lt_true_for_greater → `is_lt`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If Greater satisfies lt(), is_lt would conflate Greater with Less, breaking ordering logic everywhere.

### φ2: end_lt_self → `end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If end is strictly less than itself, lt_spec is not irreflexive and range queries using end as a bound would be unsound.

### φ3: above_self_key → `above`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If an iterator is above its own key, the gap/between predicates used by range_consistent would include keys they should exclude.

### φ4: new_spec_is_end → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a Some-carrying iterator reports is_end, get's precondition becomes unsatisfiable and every range collapses to empty.

### φ5: range_consistent_always_true → `range_consistent_impl`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If range_consistent is trivially true for any dst, the implementation cannot detect delegation mismatches, voiding its purpose.

