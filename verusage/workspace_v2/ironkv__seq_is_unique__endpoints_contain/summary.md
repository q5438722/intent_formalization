# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/seq_is_unique_v/seq_is_unique__endpoints_contain.rs`
**Date:** 2026-03-24T09:49:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `do_end_points_match` is `external_body` with `unimplemented!()`, meaning the exec function panics at runtime while its ensures clause is trusted without verification. The remaining three candidates are false positives — they are direct definitional consequences of `Seq::map` preserving length, `AbstractEndPoint` being a single-field struct where equality reduces to `id` equality, and the view function faithfully mapping the `id` field.

## True Positives (Spec Issues)

### do_end_points_match_external_body_unimplemented
- **Confidence:** medium
- **Reasoning:** `do_end_points_match` is `external_body` with `unimplemented!()` — the exec body panics at runtime. The spec property itself (extensional equality of id implies view equality) is correct, but the trusted exec function has no real implementation backing its ensures clause.

## All Candidates

### φ1: do_end_points_match_external_body_unimplemented
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `do_end_points_match` is `external_body` with `unimplemented!()` — the exec body panics at runtime while the ensures clause (`eq == (e1@ == e2@)`) is trusted without verification
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `do_end_points_match` is `external_body` with `unimplemented!()` — the exec body panics at runtime. The spec property itself (extensional equality of id implies view equality) is correct, but the trusted exec function has no real implementation backing its ensures clause.

### φ2: abstractify_empty_is_empty
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If an empty vector of endpoints doesn't abstractify to an empty sequence, the mapping function would be unsound for the base case

### φ3: abstractify_preserves_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If abstractification changed the length, it would violate the bijection between concrete and abstract endpoint sequences, breaking the containment check in `endpoints_contain`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `abstractify_end_points` is defined as `end_points@.map(...)` and `Seq::map` preserves length by definition. This is a direct definitional consequence, not a spec gap.

### φ4: endpoints_contain_false_for_absent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If distinct concrete IDs could abstractify to equal abstract endpoints, the containment check would have false positives — indicating the view function or AbstractEndPoint equality is too coarse
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property — if all concrete endpoints have different IDs from the query, the abstract sequence should not contain the query's abstract view. It follows from `AbstractEndPoint` being a single-field struct where equality is determined by `id`, so distinct IDs yield distinct abstract endpoints.

### φ5: view_preserves_id_identity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If two endpoints with extensionally equal id sequences had different abstract views, the `do_end_points_match` ensures clause would be inconsistent with structural equality of `AbstractEndPoint`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `AbstractEndPoint` has a single field `id`, so extensional equality of `id` implies structural equality of `AbstractEndPoint`. This is a trivial definitional consequence of the struct layout, not a spec gap.

