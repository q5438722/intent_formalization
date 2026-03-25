# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__indexing/impl_u__indexing__lemma_entry_base_from_index_support.rs`
**Date:** 2026-03-24T12:35:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `mod_mult_zero_implies_mod_zero` is an `external_body` axiom trusting that divisibility by a product implies divisibility by a factor, without proof. The other four are false positives — redundant instantiations of the same axiom, verified commutativity, or restatements of the lemma's ensures clauses.

## True Positives (Spec Issues)

### mod_mult_zero_implies_mod_zero_external_body
- **Confidence:** medium
- **Reasoning:** `mod_mult_zero_implies_mod_zero` is `external_body` with `unimplemented!()` body — the property that `(a % (b*c) == 0) ==> (a % b == 0)` is a correct mathematical fact but trusted without proof. The `lemma_entry_base_from_index_support` proof depends on this axiom.

## All Candidates

### φ1: mod_mult_zero_implies_mod_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mod_mult_zero_implies_mod_zero` is `external_body` — divisibility by a product implies divisibility by a factor, trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mod_mult_zero_implies_mod_zero` is `external_body` with `unimplemented!()` body — the property that `(a % (b*c) == 0) ==> (a % b == 0)` is a correct mathematical fact but trusted without proof. The `lemma_entry_base_from_index_support` proof depends on this axiom.

### φ2: aligned_product_implies_factor
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Alignment to `entry_size * a` implies alignment to `entry_size` — depends on the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the same external_body axiom already captured by φ1, just with renamed parameters. No new trust gap.

### φ3: aligned_product_implies_second_factor
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The external_body axiom composes with commutativity to derive alignment to either factor — doubles the unverified trust surface
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Composition of the external_body with multiplication commutativity (verified by nonlinear_arith). The trust gap is already identified by φ1; commutativity adds no new unverified assumption.

### φ4: nat_mul_commutative
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `nat_mul` commutativity is a verified consequence — tests the auxiliary trigger function
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Multiplication commutativity is proved by `nonlinear_arith` in the lemma body. No external_body dependency for this property.

### φ5: entry_base_aligned_support
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The lemma's universal quantifier over `nat_mul` triggers to derive alignment to `entry_size` — the entire chain depends on the external_body
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a redundant instantiation of the verified lemma's third ensures clause, which itself depends on the external_body already flagged by φ1. No new trust gap.

