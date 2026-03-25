# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__repetition__impl2__lemma_parse_productive_helper.rs`
**Date:** 2026-03-24T15:36:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: `RepeatN::lemma_parse_productive` is `external_body` despite a verified helper existing that could replace it, and the trait-level `SecureSpecCombinator::lemma_parse_productive` is an unverified foundational axiom. One false positive: the result length equaling the repetition count follows by straightforward induction on the closed spec's structure.

## True Positives (Spec Issues)

### repeatn_productive_external_body
- **Confidence:** medium
- **Reasoning:** `RepeatN::lemma_parse_productive` is `external_body` with `unimplemented!()`. While `lemma_parse_productive_helper` exists as a verified helper that proves the same property, the actual trait implementation bypasses it entirely. This is a real unverified trust assumption — the developer wrote the proof but forgot to wire it into the `external_body` override.

### inner_productive_external_body
- **Confidence:** high
- **Reasoning:** `SecureSpecCombinator::lemma_parse_productive` is `external_body` with `unimplemented!()` at the trait level. This is the foundational productivity axiom — every combinator's productivity guarantee ultimately rests on this unverified assumption.

## All Candidates

### φ1: repeatn_productive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `RepeatN::lemma_parse_productive` is `external_body` with `unimplemented!()` — the productivity property for the composed repetition combinator is trusted without proof, even though `lemma_parse_productive_helper` exists as a verified helper that could have been used
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `RepeatN::lemma_parse_productive` is `external_body` with `unimplemented!()`. While `lemma_parse_productive_helper` exists as a verified helper that proves the same property, the actual trait implementation bypasses it entirely. This is a real unverified trust assumption — the developer wrote the proof but forgot to wire it into the `external_body` override.

### φ2: inner_productive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `SecureSpecCombinator::lemma_parse_productive` is `external_body` on the trait — the base productivity guarantee for any combinator is an unverified trust assumption
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `SecureSpecCombinator::lemma_parse_productive` is `external_body` with `unimplemented!()` at the trait level. This is the foundational productivity axiom — every combinator's productivity guarantee ultimately rests on this unverified assumption.

### φ3: parse_zero_always_succeeds
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Parsing zero repetitions always succeeds regardless of input — the combinator produces a result without examining any bytes, which could mask malformed input

### φ4: parse_helper_monotone_consumed
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** With a productive inner combinator, each additional repetition must consume strictly more bytes — if this weren't true, the parser could loop without advancing through the input

### φ5: parse_result_length_equals_count
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The parsed sequence length should always equal the requested repetition count — if the result contained fewer or more elements than requested, the combinator would violate its contract
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `spec_parse_helper` base case returns `seq![]` (length 0) for `n == 0`, and each recursive step calls `vs.push(v)` adding exactly one element. By induction on `n`, the result length always equals `n`. Correct by the closed spec definition.

