# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__aligned_transitive_auto.rs`
**Date:** 2026-03-24T12:24:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `aligned_transitive` is an `external_body` axiom trusting divisibility transitivity without proof. The other four are false positives — the auto wrapper and chain are redundant instantiations of the same axiom, and zero/self-alignment are mathematical tautologies.

## True Positives (Spec Issues)

### aligned_transitive_external_body
- **Confidence:** medium
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — divisibility transitivity is trusted without proof. The `aligned_transitive_auto` lemma and all downstream alignment proofs depend on this unverified axiom.

## All Candidates

### φ1: aligned_transitive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `aligned_transitive` is `external_body` — divisibility transitivity is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — divisibility transitivity is trusted without proof. The `aligned_transitive_auto` lemma and all downstream alignment proofs depend on this unverified axiom.

### φ2: aligned_transitive_auto_universal
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The auto lemma provides universal transitivity to the SMT solver — depends on the unverified `aligned_transitive` external_body
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct instantiation of `aligned_transitive_auto`, which is a verified wrapper around the external_body. The trust gap is already captured by φ1.

### φ3: aligned_chain_three
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two applications of the external_body axiom yield a 3-step transitivity chain — composes freely without verification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two applications of the same external_body axiom already flagged by φ1. No new trust gap.

### φ4: aligned_zero_any
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero is aligned to everything — tests the trivial base case of `aligned`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % b == 0` is trivially true for any `b > 0`. No external_body dependency.

### φ5: aligned_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Every nonzero value is self-aligned — tests the `a % a == 0` tautology
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `a % a == 0` is a tautology for `a > 0`. No external_body dependency.

