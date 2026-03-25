# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__repetition__impl2__lemma_parse_length_helper.rs`
**Date:** 2026-03-24T15:38:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `RepeatN::lemma_parse_length` is `external_body` despite a verified helper existing, and the trait-level `SecureSpecCombinator::lemma_parse_length` is an unverified foundational axiom. Two false positives: consumed-bytes monotonicity is a correct downstream consequence, and result-length-equals-count follows by straightforward induction on the spec.

## True Positives (Spec Issues)

### repeatn_parse_length_external_body
- **Confidence:** medium
- **Reasoning:** `RepeatN::lemma_parse_length` is `external_body` with `unimplemented!()`. While a verified helper `lemma_parse_length_helper` exists that proves the same property, it is not wired into the trait implementation. The trait obligation is fulfilled by an unverified stub.

### inner_parse_length_external_body
- **Confidence:** high
- **Reasoning:** `SecureSpecCombinator::lemma_parse_length` is `external_body` with `unimplemented!()` at the trait level. This is the foundational parse-length bound axiom — every combinator's length guarantee rests on this unverified assumption.

## All Candidates

### φ1: repeatn_parse_length_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `RepeatN::lemma_parse_length` is `external_body` with `unimplemented!()` — the parse length bound is trusted without proof, even though `lemma_parse_length_helper` exists as a verified helper
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `RepeatN::lemma_parse_length` is `external_body` with `unimplemented!()`. While a verified helper `lemma_parse_length_helper` exists that proves the same property, it is not wired into the trait implementation. The trait obligation is fulfilled by an unverified stub.

### φ2: inner_parse_length_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `SecureSpecCombinator::lemma_parse_length` is `external_body` at the trait level — the foundational parse length bound for any combinator is an unverified trust assumption
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `SecureSpecCombinator::lemma_parse_length` is `external_body` with `unimplemented!()` at the trait level. This is the foundational parse-length bound axiom — every combinator's length guarantee rests on this unverified assumption.

### φ3: parse_zero_always_succeeds
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Parsing zero repetitions always succeeds on any input without examining it — could mask corruption in the byte stream

### φ4: parse_consumed_monotone
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Consumed bytes should be monotonically non-decreasing as repetition count grows — if more repetitions consumed fewer bytes, the parser would regress through input
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The monotonicity proof is verified by induction: each step adds `k >= 0` (from `lemma_parse_length` on the inner combinator ensuring `k >= 0`). While it depends on the `external_body` `lemma_parse_length`, the monotonicity property itself is a correct and expected consequence given the parse-length bound. It's a downstream consequence of already-flagged axioms.

### φ5: parse_result_length_equals_count
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The parsed sequence length must equal the repetition count — if more or fewer elements were returned, the combinator would violate its counting contract
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Base case returns `seq![]` (length 0) for `n == 0`; each recursive step calls `vs.push(v)` adding exactly one element. By induction, result length equals `n`. Correct by the closed spec structure.

