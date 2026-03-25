# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_utils/spec__utils__seq_to_set_rec_is_finite.rs`
**Date:** 2026-03-24T12:17:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. They test correct properties of the fully verified `seq_to_set_rec` function: base case, singleton, finiteness, last-element membership, and cardinality bound. No external_body or trust gaps exist in this file.

## All Candidates

### φ1: seq_to_set_rec_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty sequence produces empty set — tests the base case directly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Directly follows from the open spec base case `seq.len() == 0 → Set::empty()`. Correct by definition.

### φ2: seq_to_set_rec_singleton_contains
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Singleton sequence's set contains the element — tests one-step recursion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** One recursive step: `Set::empty().insert(x).contains(x)` is trivially true. Correct behavior.

### φ3: seq_to_set_rec_always_finite
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The resulting set is always finite — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Directly proved by the fully verified `seq_to_set_rec_is_finite` lemma via structural induction. Finite set from finite sequence is correct.

### φ4: seq_to_set_rec_last_in_set
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The last element is always in the set — follows from the recursive definition inserting `seq.last()`, but only the last element is guaranteed per step
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The recursive step does `.insert(seq.last())`, so the last element is always in the result. This is correct by the open spec definition.

### φ5: seq_to_set_rec_len_leq_seq_len
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The set can be strictly smaller than the sequence due to deduplication — the set size is bounded by but not equal to the sequence length, so callers cannot rely on a bijection
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Set size ≤ sequence length is a correct mathematical fact — inserting into a set grows its size by at most 1. The proof is fully verified by induction.

