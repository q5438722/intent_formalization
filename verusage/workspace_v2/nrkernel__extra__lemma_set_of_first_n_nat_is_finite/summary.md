# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__lemma_set_of_first_n_nat_is_finite.rs`
**Date:** 2026-03-24T12:26:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains a single fully verified lemma proving finiteness of `{0, ..., n-1}` by induction, with no external_body or trust gaps. All tested properties are correct mathematical facts about finite sets of naturals.

## All Candidates

### φ1: first_n_nat_finite
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The set `{0, ..., n-1}` is always finite — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the fully verified `lemma_set_of_first_n_nat_is_finite`. Correct by inductive proof.

### φ2: first_zero_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The set of naturals less than 0 is empty — tests the base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Set::new(|i: nat| i < 0)` has no members, so it equals `Set::empty()`. Correct by definition of `Set::new`.

### φ3: first_n_contains_pred
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The set `{0, ..., n-1}` contains `n-1` — tests membership at the upper boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `n-1 < n` is trivially true, so `Set::new(|i: nat| i < n).contains(n-1)` holds by definition. Correct.

### φ4: first_n_not_contains_n
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The set `{0, ..., n-1}` does not contain `n` — tests the strict upper bound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `n < n` is false, so `n` is not in the set. Correct by definition.

### φ5: first_n_insert_equals_first_n_plus_one
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Inserting `n` into `{0,...,n-1}` equals `{0,...,n}` — tests the recursive construction used in the proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The extensional equality follows from `i < n || i == n <==> i < n + 1` for naturals. Correct set identity.

