# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/properties/properties__SecureSpecBombinator__corollary_serialize_injective_contraposition.rs`
**Date:** 2026-03-24T15:29:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`, making the core serialize-parse roundtrip an unverified trust assumption. The other two candidates (injectivity and parse-succeeds) are downstream consequences of this same axiom and thus false positives as duplicates.

## True Positives (Spec Issues)

### roundtrip_external_body_trusted
- **Confidence:** high
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental correctness property of the combinator framework — that parsing a serialized value recovers the original — is entirely trusted without verification. Any concrete combinator inherits this axiom for free.

## All Candidates

### φ1: roundtrip_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()` — the fundamental serialize-parse roundtrip is an unverified trust assumption that any concrete combinator inherits for free
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental correctness property of the combinator framework — that parsing a serialized value recovers the original — is entirely trusted without verification. Any concrete combinator inherits this axiom for free.

### φ2: serialize_injective_from_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Serialize injectivity is derived from the unverified `external_body` roundtrip — a buggy combinator could have non-injective serialization accepted as sound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct downstream consequence of the already-flagged `external_body` roundtrip theorem. The injectivity proof itself is verified (two calls to the roundtrip lemma). Duplicate of φ1.

### φ3: parse_some_for_all_serialized
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The roundtrip guarantees parse always succeeds on serialized output — but this is entirely from the `external_body` axiom; a combinator whose parse rejects its own serialize output would still be "verified"
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Parse succeeding on serialized output is a direct instantiation of the roundtrip theorem. This is another downstream consequence of the same `external_body` axiom already captured by φ1, not an independent spec gap.

### φ4: wf_vacuously_true_default
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `wf` returns `true` for all values — any combinator that forgets to override `wf` would have the roundtrip theorem's precondition satisfied for ALL values including malformed ones

### φ5: requires_vacuously_true_default
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `requires` returns `true` unconditionally — any combinator needing initialization constraints but forgetting to override `requires` has all proof obligations trivially dischargeable

