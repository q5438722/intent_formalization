# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__get_fresh_nat_not_in.rs`
**Date:** 2026-03-24T12:01:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives identified: `combiner_request_ids_not_contains` and `combiner_request_ids_finite` are both `external_body` axioms trusted without proof, forming the unverified foundation for the fresh-nat generation mechanism. The other two candidates are false positives — `get_fresh_nat_always_exists` is a wrapper around a verified lemma, and the set-inclusion disjunction holds trivially from the freshness guarantees.

## True Positives (Spec Issues)

### combiner_request_ids_not_contains_external_body
- **Confidence:** medium
- **Reasoning:** `combiner_request_ids_not_contains` is `external_body` with `unimplemented!()` body — the biconditional between pointwise freshness and aggregate set non-containment is trusted without proof. While mathematically correct, this is an unverified foundation that the entire fresh-nat generation chain depends on.

### combiner_request_ids_finite_external_body
- **Confidence:** medium
- **Reasoning:** `combiner_request_ids_finite` is `external_body` — finiteness of the union-fold over `choose()` is asserted without proof. This is required by `get_fresh_nat_not_in` to construct the combined set for `element_outside_set`.

## All Candidates

### φ1: combiner_request_ids_not_contains_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `combiner_request_ids_not_contains` is `external_body` — the biconditional between pointwise freshness and set non-containment is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `combiner_request_ids_not_contains` is `external_body` with `unimplemented!()` body — the biconditional between pointwise freshness and aggregate set non-containment is trusted without proof. While mathematically correct, this is an unverified foundation that the entire fresh-nat generation chain depends on.

### φ2: combiner_request_ids_finite_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `combiner_request_ids_finite` is `external_body` — finiteness of the fold-over-choose result is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `combiner_request_ids_finite` is `external_body` — finiteness of the union-fold over `choose()` is asserted without proof. This is required by `get_fresh_nat_not_in` to construct the combined set for `element_outside_set`.

### φ3: element_outside_set_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `element_outside_set` is `external_body` — the existence of a nat outside any finite set is trusted without proof

### φ4: get_fresh_nat_always_exists
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Fresh nat generation always succeeds — depends on three chained `external_body` axioms (`element_outside_set`, `combiner_request_ids_not_contains`, `combiner_request_ids_finite`)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct call to the fully verified `get_fresh_nat_not_in` lemma. While that lemma depends on external_body axioms, the φ itself is just an instantiation — the trust gaps are in the underlying axioms (φ1 and φ2), not in this wrapper.

### φ5: get_fresh_nat_closed_spec_choose
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `get_fresh_nat` uses `closed spec` with `choose` — the chosen value for a smaller set may or may not avoid a larger set, but the second disjunct always holds; tests whether `get_fresh_nat` is monotonic w.r.t. set inclusion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The disjunction holds trivially because the second disjunct `!reqs.contains(get_fresh_nat(reqs2, combiner))` is always true (since `get_fresh_nat_not_in` guarantees the result avoids `reqs2`, and `reqs ⊆ reqs2`). This is a correct logical consequence, not a spec gap.

