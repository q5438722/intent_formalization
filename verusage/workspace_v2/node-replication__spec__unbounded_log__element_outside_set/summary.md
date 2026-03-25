# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__element_outside_set.rs`
**Date:** 2026-03-24T12:00:37Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: max_of_set_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `max_of_set` is `external_body` — the existence of an upper bound for any finite nat set is trusted without proof

### φ2: element_outside_any_finite_set
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Any finite set of nats has an element outside it — this depends entirely on the unverified `max_of_set` external_body

### φ3: max_of_empty_set_unconstrained
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For the empty set, `max_of_set` returns an arbitrary nat since the ensures is vacuously true — the "maximum" of an empty set is undefined but the spec produces a concrete value

### φ4: max_not_tight_upper_bound
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `max_of_set` guarantees an upper bound but not the least upper bound — the returned value could be arbitrarily larger than the actual maximum, and this is trusted without proof

### φ5: element_outside_set_is_beyond_max
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The witness for "element outside the set" is strictly greater than every element — this strong ordering property is a consequence of the unverified `max_of_set` axiom

