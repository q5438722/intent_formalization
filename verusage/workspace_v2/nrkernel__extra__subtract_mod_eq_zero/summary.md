# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__subtract_mod_eq_zero.rs`
**Date:** 2026-03-24T12:33:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains a fully verified lemma proving alignment closure under subtraction, using the verified vstd library lemma `lemma_sub_mod_noop`. No external_body or trust gaps exist.

## All Candidates

### φ1: subtract_aligned_preserves
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Difference of two aligned values is aligned — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the fully verified `subtract_mod_eq_zero` lemma. Closure of alignment under subtraction is a correct mathematical property.

### φ2: subtract_self_aligned
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Subtracting a value from itself yields 0, which is trivially aligned — tests the `a == b` boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % c == 0` is trivially true. The `a == b` instantiation is a degenerate case.

### φ3: subtract_zero_aligned
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Subtracting 0 preserves alignment — tests the `a == 0` boundary (tautological)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Subtracting 0 from `b` yields `b` — tautological restatement of the precondition.

### φ4: subtract_consecutive_multiples
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Difference of any two multiples of `c` is aligned to `c` — tests closure of multiples under subtraction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(k2 - k1) * c` is a multiple of `c`, so it's aligned. Correct mathematical closure property proved via the verified lemma.

### φ5: vstd_lemma_sub_mod_noop_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The proof depends on vstd's `lemma_sub_mod_noop` — if that library lemma were unsound the subtraction-alignment property would be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `vstd::arithmetic::div_mod::lemma_sub_mod_noop` is a verified vstd library lemma, not an `external_body`. It is part of the trusted verified standard library.

