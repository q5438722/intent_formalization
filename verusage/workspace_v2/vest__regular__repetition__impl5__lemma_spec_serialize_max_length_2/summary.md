# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__repetition__impl5__lemma_spec_serialize_max_length_2.rs`
**Date:** 2026-03-24T15:47:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `lemma_spec_serialize_max_length` is now `external_body` (unlike the previously-seen verified version), making both its ensures clauses (individual element bound and prefix bound) unverified trust assumptions; and `spec_serialize` continues to ignore the count field entirely. Two false positives: the full-take identity is a trivial tautology restating the precondition, and empty serialization follows from fold_left's base case.

## True Positives (Spec Issues)

### serialize_max_length_external_body
- **Confidence:** high
- **Reasoning:** `lemma_spec_serialize_max_length` is `external_body` with `unimplemented!()`. The individual element serialization length bound is entirely trusted without verification. Note this version replaces the previously-seen verified version of the same lemma.

### serialize_prefix_max_length_external_body
- **Confidence:** high
- **Reasoning:** Same `external_body` `lemma_spec_serialize_max_length` — the prefix serialization length bound is a second ensures clause of the same unverified lemma. This is an independent trust assumption (prefix lengths bounded) from the same unverified stub.

### serialize_ignores_count_field
- **Confidence:** high
- **Reasoning:** `spec_serialize` uses `vs.fold_left(...)` which iterates over the provided sequence regardless of `self.1`. The count field is completely ignored during serialization — a persistent spec design gap.

## All Candidates

### φ1: serialize_max_length_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_spec_serialize_max_length` is `external_body` with `unimplemented!()` — the individual element length bound is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_spec_serialize_max_length` is `external_body` with `unimplemented!()`. The individual element serialization length bound is entirely trusted without verification. Note this version replaces the previously-seen verified version of the same lemma.

### φ2: serialize_prefix_max_length_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_spec_serialize_max_length` is `external_body` — the prefix serialization length bound is also trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same `external_body` `lemma_spec_serialize_max_length` — the prefix serialization length bound is a second ensures clause of the same unverified lemma. This is an independent trust assumption (prefix lengths bounded) from the same unverified stub.

### φ3: serialize_ignores_count_field
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `spec_serialize` uses `fold_left` over `vs` without referencing `self.1` — the count field is completely ignored during serialization
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `spec_serialize` uses `vs.fold_left(...)` which iterates over the provided sequence regardless of `self.1`. The count field is completely ignored during serialization — a persistent spec design gap.

### φ4: max_length_2_full_take_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The `_2` variant extends the range to `i <= vs.len()` — but `vs.take(vs.len())` is just `vs`, so this is the precondition restated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `vs.take(vs.len())` equals `vs`, so the ensures clause reduces to the precondition `c.spec_serialize(vs).len() <= usize::MAX`. This is a trivial tautology, not a spec gap.

### φ5: serialize_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Serializing an empty sequence should produce empty bytes — verifying fold_left base case behavior
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `fold_left(Seq::empty(), f)` on an empty sequence returns `Seq::empty()` by the vstd base case. Correct by definition.

