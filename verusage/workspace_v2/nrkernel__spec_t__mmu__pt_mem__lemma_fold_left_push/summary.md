# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__lemma_fold_left_push.rs`
**Date:** 2026-03-24T13:32:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The `lemma_fold_left_push` is a correctly proven (non-external_body) lemma about `fold_left` behavior. The tested properties — empty base case, double push decomposition, and degenerate closure handling — are all correct and expected mathematical properties of fold_left.

## All Candidates

### φ1: fold_left_push_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Tests fold_left_push with empty sequence — base case where `s.fold_left(b, f) == b` so result should be `f(b, a)`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and expected property. Pushing 42 onto an empty sequence and folding with addition from 0 yields 42. This is standard fold_left behavior, not a spec gap.

### φ2: fold_left_push_non_commutative
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Tests with non-commutative subtraction — fold_left is left-associative, so push should apply `f` to the accumulated result

### φ3: fold_left_push_ignores_order
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subtraction is non-commutative, so `[a, b]` and `[b, a]` should give different results — if this verifies, it confirms fold_left respects order, but `(0 - a) - b == (0 - b) - a` is actually `-(a+b)` which IS equal, making this vacuously false

### φ4: fold_left_push_concat_equivalence
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Double push should decompose into accumulated sum plus both elements — tests that repeated application of the lemma composes correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct mathematical property — folding addition over a double push decomposes into the sum of the original fold plus both pushed elements. This is standard fold_left + addition homomorphism, not a spec issue.

### φ5: fold_left_push_closure_mismatch
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** With `f = |acc, x| x` (ignores accumulator), fold_left always returns the last element — the lemma should confirm `f(f(10, 1), 2) == 2`, testing a degenerate closure that discards history
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma correctly handles degenerate closures. With `f = |acc, x| x`, `fold_left` always returns the last element, and the push lemma correctly states the result equals `f(prior_fold, 2) == 2`. This is expected behavior for any well-defined fold_left.

