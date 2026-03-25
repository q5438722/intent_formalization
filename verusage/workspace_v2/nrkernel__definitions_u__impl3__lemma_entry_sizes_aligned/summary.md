# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__impl3__lemma_entry_sizes_aligned.rs`
**Date:** 2026-03-24T12:21:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `aligned_transitive` is an `external_body` axiom trusting divisibility transitivity without proof, which the `lemma_entry_sizes_aligned` inductive step depends on. The other four candidates are false positives — they are either redundant instantiations of the same axiom, trivially correct properties (self-alignment), or properties derivable without the external_body (adjacent divisibility).

## True Positives (Spec Issues)

### aligned_transitive_external_body
- **Confidence:** medium
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — transitivity of divisibility is a correct mathematical fact but trusted without proof. The `lemma_entry_sizes_aligned` proof depends on this axiom for its inductive step.

## All Candidates

### φ1: aligned_transitive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `aligned_transitive` is `external_body` — transitivity of alignment (divisibility) is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — transitivity of divisibility is a correct mathematical fact but trusted without proof. The `lemma_entry_sizes_aligned` proof depends on this axiom for its inductive step.

### φ2: entry_sizes_aligned_uses_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The verified `lemma_entry_sizes_aligned` depends on the unverified `aligned_transitive` external_body — if transitivity were wrong the entire alignment chain would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct call to the verified `lemma_entry_sizes_aligned`. The trust gap is in the underlying `aligned_transitive` axiom already captured by φ1; this wrapper adds no new information.

### φ3: aligned_transitive_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two applications of the external_body axiom yield a 3-step transitivity chain — the unverified axiom composes freely to derive alignment across arbitrarily many intermediaries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just two applications of the same external_body axiom already flagged by φ1. The composability of transitivity is a correct mathematical property and the trust gap is already identified.

### φ4: self_aligned_from_lemma
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Self-alignment is trivially true but the lemma proves it via nonlinear_arith rather than from `x % x == 0` — tests the base case path
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Self-alignment (`x % x == 0` for `x > 0`) is trivially correct. The base case in `lemma_entry_sizes_aligned` proves this via `nonlinear_arith` without using the external_body.

### φ5: adjacent_alignment_from_inv
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Adjacent layer divisibility follows from `entry_size(i) == entry_size(i+1) * num_entries(i+1)` — but the proof path goes through the lemma that uses the external_body `aligned_transitive`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Adjacent layer divisibility follows directly from `entry_size(i) == entry_size(i+1) * num_entries(i+1)` in the invariant. For the `i+1` case, the lemma doesn't actually need `aligned_transitive` — the base case and one step suffice.

