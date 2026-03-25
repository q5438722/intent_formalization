# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__set_mismatch.rs`
**Date:** 2026-03-24T11:42:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `singleton_set_unique_elt` is `external_body` trusting the uniqueness property of singleton sets without proof, forming the unverified base case of the `set_mismatch` induction. Three false positives: `set_mismatch` itself is fully verified (just depends on the flagged axiom), the concrete singleton test is trivially correct, and the zero-length boundary is contradictory by vstd's own axioms.

## True Positives (Spec Issues)

### singleton_set_unique_elt_external_body
- **Confidence:** medium
- **Reasoning:** `singleton_set_unique_elt` is `external_body` trusting that a set of length 1 containing both `a` and `b` implies `a == b`. While mathematically correct, this is unverified and serves as the base case for `set_mismatch`'s inductive proof — any error would propagate unsoundness.

## All Candidates

### φ1: singleton_set_unique_elt_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `singleton_set_unique_elt` is `external_body` — the fundamental uniqueness property of singleton sets is trusted without proof; if wrong, `set_mismatch` inherits unsoundness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `singleton_set_unique_elt` is `external_body` trusting that a set of length 1 containing both `a` and `b` implies `a == b`. While mathematically correct, this is unverified and serves as the base case for `set_mismatch`'s inductive proof — any error would propagate unsoundness.

### φ2: set_mismatch_derives_false
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `set_mismatch` derives `false` — a contradiction lemma whose correctness depends on the external_body `singleton_set_unique_elt`; the chain of trust bottoms out at an unverified axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `set_mismatch` is fully verified (not external_body) — its proof is a correct induction on set size using `remove` and `choose`. The only trust gap is the base case's reliance on `singleton_set_unique_elt`, which is already flagged separately. The contradiction itself is a correct mathematical fact.

### φ3: singleton_any_two_elements_equal
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The external_body axiom allows deriving that any concrete singleton set has exactly one element — testing the axiom on a concrete set to confirm it doesn't over-constrain
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A concrete singleton set `{5}` having exactly one element is a trivially correct set-theoretic property. The φ merely instantiates the external_body axiom on a concrete case where the result is obviously true.

### φ4: set_mismatch_len_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A set of length 0 containing an element is contradictory — tests whether `set_mismatch` can be invoked at the degenerate boundary where `s2` is empty
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A finite set with `len() == 0` containing an element is contradictory by vstd's own set axioms — this derives `false` without any external_body involvement. The precondition is simply unsatisfiable.

### φ5: set_mismatch_strict_subset_equal_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A strict subset of a finite set cannot have equal cardinality — tests the consistency of `set_mismatch`'s preconditions against basic set theory

