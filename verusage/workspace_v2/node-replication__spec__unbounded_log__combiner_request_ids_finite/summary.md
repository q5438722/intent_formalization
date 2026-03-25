# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__combiner_request_ids_finite.rs`
**Date:** 2026-03-24T11:57:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `seq_to_set_is_finite` is an `external_body` axiom trusted without proof, forming the unverified foundation for the `combiner_request_ids_finite` proof. The other two candidates are false positives — Ready contributing empty ops and the empty-map base case are both correct by design.

## True Positives (Spec Issues)

### seq_to_set_is_finite_external_body
- **Confidence:** medium
- **Reasoning:** `seq_to_set_is_finite` is `#[verifier::external_body]` with an `unimplemented!()` body — the finiteness of `Set::new(|a| seq.contains(a))` is asserted without proof. While mathematically true (a sequence has finitely many distinct elements), this is an unverified trust assumption that the entire `combiner_request_ids_finite` proof chain depends on.

## All Candidates

### φ1: seq_to_set_is_finite_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `seq_to_set_is_finite` is `external_body` — finiteness of `Set::new(|a| seq.contains(a))` is trusted without proof, and the set comprehension could in principle capture elements outside the sequence indices
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `seq_to_set_is_finite` is `#[verifier::external_body]` with an `unimplemented!()` body — the finiteness of `Set::new(|a| seq.contains(a))` is asserted without proof. While mathematically true (a sequence has finitely many distinct elements), this is an unverified trust assumption that the entire `combiner_request_ids_finite` proof chain depends on.

### φ2: combiner_request_ids_infinite_arbitrary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the domain is infinite, `combiner_request_ids` returns `arbitrary()` which can equal any set — an infinite combiner map yields unconstrained request IDs

### φ3: combiner_ready_contributes_empty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A `Ready` combiner contributes empty queued_ops — but it still participates in the `combiner_request_ids` fold, potentially masking that Ready combiners should not be in the map at all
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `queued_ops()` spec function explicitly returns `Seq::empty()` for `CombinerState::Ready`. This is correct by design — a Ready combiner has no pending operations, and contributing an empty set to the fold is harmless.

### φ4: combiner_request_ids_empty_map
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty combiner map yields empty request IDs — tests that the base case of the fold-over-choose produces exactly the empty set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case of `combiner_request_ids` explicitly returns `Set::empty()` when `combiners.dom().len() == 0`. This is the correct and intended base case for the fold.

### φ5: combiner_request_ids_superset_of_single
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Each node's queued_ops is a subset of the aggregate — tests that the nondeterministic `choose()`-based fold correctly accumulates all nodes' contributions

