# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl3__values_agree.rs`
**Date:** 2026-03-24T08:35:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. It tests that two `EndPoint` values with equal id sequences have equal abstract views, which is a trivial consequence of the `EndPoint::view` definition constructing `AbstractEndPoint` from the id field. No spec issues were identified.

## All Candidates

### φ1: values_agree_lo_equals_hi
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When lo == hi, `values_agree` checks a single element — the forall quantifier `lo <= i <= hi` degenerates to checking one index, which could mask off-by-one errors in callers that expect a range was validated

### φ2: values_agree_false_branch_inconsistent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The second return value `ret.1` represents "almost agrees" (all match except the last) — but when `ret.0` is false and `ret.1` is true, the hi element disagrees while all prior agree; when `ret.1` is false, some element before hi also disagrees, but the spec doesn't tell you which one

### φ3: do_end_points_match_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `do_end_points_match` is external_body and trusts that byte-level comparison matches spec-level `AbstractEndPoint` equality — if the implementation uses a different comparison (e.g., pointer equality or partial comparison), the ensures would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property simply unfolds the definition of `EndPoint::view`: `e1@ == e2@` expands to `AbstractEndPoint{id: e1.id@} == AbstractEndPoint{id: e2.id@}`, which follows directly from `e1.id@ == e2.id@` by structural equality. This has nothing to do with `do_end_points_match`'s external_body — it's a tautological consequence of the view definition.

### φ4: map_valid_ghost_desync_possible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `map_valid` asserts `m@.dom() == keys@.to_set()` — but `m` is a Ghost field with no runtime enforcement, so the ghost map domain could theoretically diverge from the actual keys if any external_body function mutates keys without updating the ghost

### φ5: values_agree_ret1_true_when_ret0_true
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When `ret.0` is true, the spec says nothing about `ret.1` — the second return value is only specified under `!ret.0`, leaving `ret.1` unconstrained when all values agree, which could cause callers to read garbage from the second field on the success path

