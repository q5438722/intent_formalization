# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/properties/properties__SecureSpecBombinator__lemma_serialize_productive.rs`
**Date:** 2026-03-24T15:30:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: `theorem_serialize_parse_roundtrip` and `lemma_parse_productive` are independent `external_body` axioms with `unimplemented!()` bodies, each trusting a distinct property (roundtrip correctness and parse productivity). The serialize productivity candidate is a false positive — it's a verified derivation from the two already-flagged axioms.

## True Positives (Spec Issues)

### roundtrip_external_body_trusted
- **Confidence:** high
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental serialize-parse roundtrip property is entirely trusted without verification. Any concrete combinator inherits this axiom for free.

### parse_productive_external_body_trusted
- **Confidence:** high
- **Reasoning:** `lemma_parse_productive` is a separate `external_body` with `unimplemented!()`, independent from the roundtrip theorem. It trusts that productive combinators always consume >0 bytes on successful parse — this is a distinct unverified assumption.

## All Candidates

### φ1: roundtrip_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()` — the fundamental serialize-parse roundtrip correctness is entirely trusted without verification
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental serialize-parse roundtrip property is entirely trusted without verification. Any concrete combinator inherits this axiom for free.

### φ2: parse_productive_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_parse_productive` is `external_body` with `unimplemented!()` — the productivity guarantee (parse consumes >0 bytes) is trusted without proof, allowing a non-productive parser to be accepted as productive
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_parse_productive` is a separate `external_body` with `unimplemented!()`, independent from the roundtrip theorem. It trusts that productive combinators always consume >0 bytes on successful parse — this is a distinct unverified assumption.

### φ3: serialize_productive_from_two_externals
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Serialize productivity chains two `external_body` axioms (roundtrip + parse_productive) — if either trusted axiom is wrong, a zero-length serialization would be accepted for a supposedly productive combinator
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a downstream consequence of the two already-flagged `external_body` axioms. The `lemma_serialize_productive` proof itself is verified — it correctly chains the roundtrip and parse-productive lemmas. Duplicate of φ1 + φ2.

### φ4: wf_default_true_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `wf` returns `true` for all values — any combinator that forgets to override `wf` has the roundtrip and productivity theorems' preconditions trivially satisfied for ALL values including malformed ones

### φ5: requires_default_true_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `requires` returns `true` unconditionally — any combinator needing initialization constraints but forgetting to override `requires` has all proof obligations trivially dischargeable

