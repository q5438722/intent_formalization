# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/properties/properties__SecureSpecBombinator__corollary_serialize_injective.rs`
**Date:** 2026-03-24T15:27:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`, meaning the fundamental correctness property of the combinator framework is an unverified trust assumption. The other two candidates are downstream consequences of this same axiom — injectivity is a verified corollary and the length non-negativity follows from sequence length semantics — so they are false positives as duplicates.

## True Positives (Spec Issues)

### serialize_parse_roundtrip_external_body
- **Confidence:** high
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental serialize-parse roundtrip property — the cornerstone of combinator correctness — is entirely trusted without verification. Each concrete combinator inherits this axiom for free without proving it.

## All Candidates

### φ1: serialize_parse_roundtrip_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()` — the fundamental correctness property linking serialize and parse is entirely trusted without verification, so any buggy combinator implementation would be accepted as sound
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `theorem_serialize_parse_roundtrip` is `external_body` with `unimplemented!()`. The fundamental serialize-parse roundtrip property — the cornerstone of combinator correctness — is entirely trusted without verification. Each concrete combinator inherits this axiom for free without proving it.

### φ2: serialize_injective_from_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Serialize injectivity is derived entirely from the unverified `external_body` roundtrip theorem — if the roundtrip axiom were wrong for some combinator, injectivity would be unsoundly assumed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct downstream consequence of the already-flagged `external_body` roundtrip theorem. The injectivity corollary's proof is verified (it calls the roundtrip lemma twice and the conclusion follows). The root issue is the roundtrip axiom, not this derived property. Duplicate of φ1.

### φ3: parse_result_length_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The roundtrip theorem guarantees parse returns a non-negative consumed length equal to serialized length, but there is no upper bound constraint on `spec_parse` for arbitrary inputs — parse could claim to consume more bytes than exist without violating any axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures that parse is `Some` and consumed length ≥ 0 both follow directly from instantiating the roundtrip theorem (consumed length equals `spec_serialize(v).len()` which is a `nat` cast to `int`, hence ≥ 0). This is a downstream consequence of the already-flagged external_body roundtrip, not an independent spec gap.

### φ4: wf_default_true_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `wf` implementation returns `true` for all values — any combinator that forgets to override `wf` would vacuously satisfy the roundtrip theorem's precondition for ALL values including malformed ones

### φ5: requires_default_true_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The default `requires` implementation returns `true` unconditionally — any combinator that needs initialization constraints but forgets to override `requires` would have all its proof obligations trivially satisfiable

