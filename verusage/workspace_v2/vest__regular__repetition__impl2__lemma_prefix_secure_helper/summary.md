# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__repetition__impl2__lemma_prefix_secure_helper.rs`
**Date:** 2026-03-24T15:39:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives: the trait-level `lemma_prefix_secure` is an unverified foundational axiom, `RepeatN::lemma_prefix_secure` is `external_body` despite a verified helper existing, and `lemma_parse_length_helper` is an independent unverified assumption used as a dependency. One false positive: the prefix security helper property is correct and its dependency on `external_body` is already captured by other candidates.

## True Positives (Spec Issues)

### prefix_secure_external_body_trait
- **Confidence:** high
- **Reasoning:** `SecureSpecCombinator::lemma_prefix_secure` is `external_body` with `unimplemented!()` at the trait level. The foundational prefix security property for all combinators is entirely trusted without verification.

### repeatn_prefix_secure_external_body
- **Confidence:** medium
- **Reasoning:** `RepeatN::lemma_prefix_secure` is `external_body` with `unimplemented!()` despite a verified `lemma_prefix_secure_helper` existing that proves the same property. The trait obligation bypasses the actual proof — though this is a separate `external_body` from the trait-level one.

### parse_length_helper_external_body
- **Confidence:** high
- **Reasoning:** `lemma_parse_length_helper` is `external_body` with `unimplemented!()`. This is an independent unverified assumption — the parse length bound `0 <= m <= s.len()` for `RepeatN` is trusted and used as a critical dependency in the verified `lemma_prefix_secure_helper`.

## All Candidates

### φ1: prefix_secure_external_body_trait
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `SecureSpecCombinator::lemma_prefix_secure` is `external_body` with `unimplemented!()` at the trait level — the foundational prefix security property is an unverified trust assumption
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `SecureSpecCombinator::lemma_prefix_secure` is `external_body` with `unimplemented!()` at the trait level. The foundational prefix security property for all combinators is entirely trusted without verification.

### φ2: repeatn_prefix_secure_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `RepeatN::lemma_prefix_secure` is `external_body` despite a verified `lemma_prefix_secure_helper` existing — the trait obligation bypasses the actual proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `RepeatN::lemma_prefix_secure` is `external_body` with `unimplemented!()` despite a verified `lemma_prefix_secure_helper` existing that proves the same property. The trait obligation bypasses the actual proof — though this is a separate `external_body` from the trait-level one.

### φ3: parse_length_helper_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_parse_length_helper` is `external_body` with `unimplemented!()` — the parse length bound used as a dependency in `lemma_prefix_secure_helper` is itself unverified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_parse_length_helper` is `external_body` with `unimplemented!()`. This is an independent unverified assumption — the parse length bound `0 <= m <= s.len()` for `RepeatN` is trusted and used as a critical dependency in the verified `lemma_prefix_secure_helper`.

### φ4: parse_zero_always_succeeds
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Parsing zero repetitions always succeeds on any input — the combinator produces a result without examining any bytes

### φ5: prefix_secure_helper_verified_but_unused
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The verified `lemma_prefix_secure_helper` proves prefix security for `spec_parse_helper` but depends on the `external_body` `lemma_parse_length_helper` — if the length bound were wrong, this "verified" proof would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property itself (prefix security of `spec_parse_helper`) is correct and expected. While the proof depends on the `external_body` `lemma_parse_length_helper`, that dependency is already captured by φ3. This candidate flags a downstream consequence, not an independent spec gap.

