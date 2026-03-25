# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__api_actions__lemma_list_pods_request_returns_ok_list_resp_containing_matching_pods/original.rs`
**Date:** 2026-03-24T05:24:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives — they expose reliance on `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter`, an `#[verifier::external_body]` axiom whose biconditional is entirely unverified. The first uses it directly (empty filter → universal non-satisfaction), and the second derives filter completeness through its contrapositive. Both are real soundness gaps rooted in trusted axioms.

## True Positives (Spec Issues)

### filter_empty_implies_no_elements_false_direction
- **Confidence:** medium
- **Reasoning:** `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter` is `#[verifier::external_body]` — the biconditional between empty filter and universal non-satisfaction is unverified. If the filter implementation or its axiomatization has a bug, this direction (empty filter → no satisfying elements) could be unsound.

### filter_contains_then_original_and_pred
- **Confidence:** medium
- **Reasoning:** This derives filter completeness (element in seq + satisfies pred → in filtered seq) from the biconditional external_body axiom via contrapositive. The proof chain relies entirely on the trusted `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter` — a real soundness dependency on an unverified axiom.

## All Candidates

### φ1: filter_empty_implies_no_elements_false_direction
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the external_body lemma asserts a biconditional between empty filter and universal non-satisfaction — if unsound, it could let you prove no elements satisfy a predicate just because filter length is 0, even if the filter implementation has bugs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter` is `#[verifier::external_body]` — the biconditional between empty filter and universal non-satisfaction is unverified. If the filter implementation or its axiomatization has a bug, this direction (empty filter → no satisfying elements) could be unsound.

### φ2: set_to_seq_roundtrip_inflates
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** combining the external_body contains-biconditional with no_duplicates implicitly forces to_seq().len() == s.len() — if the two trusted axioms are inconsistent with the actual to_seq implementation this could be unsound

### φ3: filter_contains_then_original_and_pred
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the external_body equivalence lemma only relates empty filter to universal non-satisfaction — deriving that a specific satisfying element must appear in the filter output relies on the trusted axiom being strong enough, which could mask a missing completeness axiom
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This derives filter completeness (element in seq + satisfies pred → in filtered seq) from the biconditional external_body axiom via contrapositive. The proof chain relies entirely on the trusted `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter` — a real soundness dependency on an unverified axiom.

### φ4: marshal_preserves_integrity_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_preserves_integrity is external_body and only guarantees unmarshal(marshal(o)) == o — the reverse direction (marshal(unmarshal(obj)) == obj) is NOT axiomatized, so if entailed it reveals the uninterpreted functions accidentally give a stronger-than-intended bijection

### φ5: finite_filter_on_infinite_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** finite_set_to_finite_filtered_set requires s.finite() but Set::full() is infinite — if Verus accepts this call it would mean the precondition check is bypassed via the external_body trust, allowing infinite sets to be treated as finite

