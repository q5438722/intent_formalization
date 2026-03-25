# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_subregion/subregion_new_with_condition.rs`
**Date:** 2026-03-24T15:19:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_condition_sufficient_to_create_wrpm_subregion` is `external_body`, trusting the core permission transfer argument (constructing a crash state for the original view from an alternative view's crash state) without verification. Three false positives confirm correct properties: condition-to-permission chaining by definition, transitivity of equality for memory-differ, and subrange composition for nested subregion views.

## True Positives (Spec Issues)

### lemma_condition_sufficient_external_body
- **Confidence:** high
- **Reasoning:** `lemma_condition_sufficient_to_create_wrpm_subregion` is `external_body` with `unimplemented!()`. It asserts that given a sufficient condition, any alternative region view differing only in writable addresses has all its crash states satisfying the permission. The proof requires constructing a crash state for the original view from the alternative's crash state — this non-trivial reasoning is entirely trusted.

## All Candidates

### φ1: lemma_condition_sufficient_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_condition_sufficient_to_create_wrpm_subregion` is `external_body` — the key permission transfer from condition sufficiency to crash-state permission is trusted without proof; if the crash-state construction in the proof were wrong, unsound writes would be permitted
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_condition_sufficient_to_create_wrpm_subregion` is `external_body` with `unimplemented!()`. It asserts that given a sufficient condition, any alternative region view differing only in writable addresses has all its crash states satisfying the permission. The proof requires constructing a crash state for the original view from the alternative's crash state — this non-trivial reasoning is entirely trusted.

### φ2: condition_closed_under_diff_vacuous_all_writable
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** When the entire region is writable, `memories_differ_only_where_subregion_allows` is vacuously true for any s1/s2, so the condition closure clause forces the condition to hold for ALL sequences of the right length — making it trivially universal

### φ3: condition_implies_perm_for_current_crashes
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The current region's own crash states should satisfy permission via condition — tests that the three-part chain (crash→condition→perm) correctly covers the base case before any subregion writes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `condition_sufficient_to_create_wrpm_subregion` includes `forall |crash_state| region_view.can_crash_as(crash_state) ==> condition(crash_state)` and `forall |crash_state| condition(crash_state) ==> perm.check_permission(crash_state)`. Chaining these two implications directly yields the ensures. Correct by definition.

### φ4: memories_differ_transitive
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Transitivity of memory-differ — if m1 agrees with m2 outside writable and m2 agrees with m3, then m1 agrees with m3; if this failed, chaining subregion mutations would lose the invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For any address satisfying the antecedent (outside writable range), `m1[addr] == m2[addr]` and `m2[addr] == m3[addr]`, so `m1[addr] == m3[addr]` by transitivity of equality. Correct.

### φ5: get_subregion_view_nested
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Nested subregion views should compose correctly — if subrange-of-subrange didn't equal a direct subrange at the composed offset, nested subregion creation would produce wrong views
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Nested subranges compose: `subrange(start1, start1+len1).subrange(start2, start2+len2)` equals `subrange(start1+start2, start1+start2+len2)` by the vstd subrange-of-subrange axiom. The proof uses extensional equality to establish this. Correct.

