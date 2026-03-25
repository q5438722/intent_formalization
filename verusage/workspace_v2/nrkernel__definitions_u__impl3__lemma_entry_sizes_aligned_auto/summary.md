# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__impl3__lemma_entry_sizes_aligned_auto.rs`
**Date:** 2026-03-24T12:19:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_entry_sizes_aligned` is an `external_body` axiom that trusts the inductive alignment proof between arbitrary layer pairs without verification. The other four candidates are false positives — self-alignment is a tautology, adjacent-layer divisibility follows directly from the `inv()` invariant, and the remaining two are redundant instantiations of the same external_body.

## True Positives (Spec Issues)

### lemma_entry_sizes_aligned_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_entry_sizes_aligned` is `external_body` with `unimplemented!()` body — the alignment of coarser layer entry sizes to finer ones is trusted without proof. While mathematically correct (follows from `entry_size(i) = entry_size(i+1) * num_entries(i+1)`), the inductive proof is missing.

## All Candidates

### φ1: lemma_entry_sizes_aligned_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_entry_sizes_aligned` is `external_body` — the alignment relationship between layer entry sizes is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_entry_sizes_aligned` is `external_body` with `unimplemented!()` body — the alignment of coarser layer entry sizes to finer ones is trusted without proof. While mathematically correct (follows from `entry_size(i) = entry_size(i+1) * num_entries(i+1)`), the inductive proof is missing.

### φ2: self_aligned
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Every entry size is self-aligned — trivially true for any nonzero nat, but derived from the external_body axiom rather than from first principles
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Self-alignment (`x % x == 0`) is trivially true for any nonzero nat, and `inv()` ensures `entry_size > 0`. While it happens to go through the external_body, this is a correct mathematical tautology.

### φ3: aligned_auto_instantiation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Layer 0's entry size is aligned to layer 1's — follows from auto lemma which itself depends on the unverified external_body base lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just an instantiation of `lemma_entry_sizes_aligned_auto` at specific indices. The trust gap is already captured by φ1; this adds no new information.

### φ4: adjacent_layers_divisible
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Adjacent layer entry sizes are divisible — the `entry_size_is_next_layer_size` invariant makes `entry_size(i) == entry_size(i+1) * num_entries(i+1)`, so divisibility is a verified consequence, but the alignment lemma's external_body makes even this path unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For adjacent layers, `entry_size(i) == entry_size(i+1) * num_entries(i+1)` is directly stated in `inv()` via `entry_size_is_next_layer_size`. Divisibility follows trivially from `(a*b) % a == 0`. The external_body is not needed for this case.

### φ5: aligned_transitive_via_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Alignment is transitive across non-adjacent layers — this skips intermediate layers entirely, relying solely on the external_body axiom without verifying the inductive chain
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct call to `lemma_entry_sizes_aligned(i, k)` — the same external_body axiom already flagged by φ1. It adds no additional trust gap beyond what φ1 captures.

