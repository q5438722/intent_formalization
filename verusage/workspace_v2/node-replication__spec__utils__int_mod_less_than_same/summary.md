# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_utils/spec__utils__int_mod_less_than_same.rs`
**Date:** 2026-03-24T12:08:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. They are all direct instantiations or weaker consequences of the fully verified `int_mod_less_than_same` lemma, which proves a correct mathematical identity using nonlinear arithmetic. No external_body or trust gaps exist in this file.

## All Candidates

### φ1: mod_identity_small
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Modulo of a value smaller than the divisor equals itself — tests the core ensures clause directly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct restatement of the fully verified lemma's ensures clause. `i % len == i` for `0 <= i < len` is a correct mathematical fact proved by nonlinear arithmetic.

### φ2: mod_zero_is_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** 0 mod anything positive is 0 — tests the lower boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiation of the lemma at `i=0`. `0 % len == 0` is a correct and expected boundary case.

### φ3: mod_len_minus_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `(len-1) % len == len-1` — tests the upper boundary just below the divisor
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiation at `i=len-1`. `(len-1) % len == len-1` is correct for `len > 1`.

### φ4: mod_one_is_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `len == 1`, only `i == 0` satisfies the precondition — tests the degenerate divisor case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `len=1`, the only valid `i` is 0, and `0 % 1 == 0` is trivially correct.

### φ5: mod_identity_implies_less
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The modulo result is strictly less than the divisor — a weaker consequence that follows from the identity, but could mask that this only holds under `i < len`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A weaker consequence of `i % len == i` combined with the precondition `i < len`. Correct mathematical fact.

