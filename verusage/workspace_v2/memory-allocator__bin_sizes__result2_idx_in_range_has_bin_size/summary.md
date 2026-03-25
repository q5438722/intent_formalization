# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__result2_idx_in_range_has_bin_size.rs`
**Date:** 2026-03-24T10:40:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

One false positive: the `size_of_bin` candidate is a trivial tautology that proves nothing about the external_body gap. Two true positives: `pow2` silently returning 1 for negative exponents is a non-standard design choice that could mask errors, and `result_idx_in_range_has_bin_size` being external_body means the core inductive lemma connecting bounded checking to universal quantification is entirely trusted without verification.

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). Callers passing negative exponents by mistake silently get 1 with no precondition guard, and this non-standard behavior could mask off-by-one errors near e=0 boundaries.

### result_idx_external_body_trusted
- **Confidence:** medium
- **Reasoning:** `result_idx_in_range_has_bin_size` is `external_body` with `unimplemented!()` — the inductive lemma connecting `check_idx_in_range_has_bin_size` to the universally quantified `property_idx_in_range_has_bin_size` is trusted without proof. This is a real soundness concern since the entire bin-fitting correctness argument depends on this unverified induction step.

## All Candidates

### φ1: size_of_bin_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `size_of_bin` is `external_body` with no axioms — its return value is completely uninterpreted, so `pfd_lower`, `pfd_upper`, and the entire bin-fitting pipeline have no concrete meaning
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `size_of_bin(1) == size_of_bin(1)` is a trivial tautology (x == x) that holds for any expression. It demonstrates nothing about the external_body gap — the ensures would be true even if `size_of_bin` were fully specified.

### φ2: smallest_bin_wsize1_is_bin1
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `smallest_bin_fitting_size` maps all sizes 0–8 to bin 1 because `wsize = (size + 7) / 8` yields wsize ≤ 1 for size ≤ 8 — bins 2–8 (wsize 2–8 range) are unreachable for byte sizes ≤ 8, potentially misallocating small objects

### φ3: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers passing negative exponents by mistake silently get 1
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). Callers passing negative exponents by mistake silently get 1 with no precondition guard, and this non-standard behavior could mask off-by-one errors near e=0 boundaries.

### φ4: result_idx_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `result_idx_in_range_has_bin_size` is `external_body` — the inductive unrolling lemma that connects `check_idx_in_range_has_bin_size` to the universally quantified property is trusted without proof, so the entire bin-fitting correctness chain rests on an unverified assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `result_idx_in_range_has_bin_size` is `external_body` with `unimplemented!()` — the inductive lemma connecting `check_idx_in_range_has_bin_size` to the universally quantified `property_idx_in_range_has_bin_size` is trusted without proof. This is a real soundness concern since the entire bin-fitting correctness argument depends on this unverified induction step.

### φ5: smallest_bin_huge_boundary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The boundary between the computed bin path and BIN_HUGE is at wsize > 524288 (strict), so wsize == 524288 (size == 4194304) takes the complex computation path rather than mapping to BIN_HUGE — off-by-one could cause the largest non-huge size to get an incorrect bin index

