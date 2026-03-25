# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/array/array__impl2__init2zero.rs`
**Date:** 2026-03-24T06:34:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

Four of the five candidates are false positives reflecting standard properties of vstd sequences, Rust type semantics, and vacuous truth over empty collections. The one true positive (`set_external_body_ar_ghost_desync`) identifies a real architectural gap: the `Array::set` external_body ensures only constrain the `Ghost<Seq<A>>` field, with no specification linking the concrete `ar: [A;N]` field to the ghost state. This means all verified properties hold over ghost state only, and there is no formal guarantee that the runtime array is actually updated correctly.

## True Positives (Spec Issues)

### set_external_body_ar_ghost_desync
- **Confidence:** medium
- **Reasoning:** The `set` external_body ensures only constrain `self.seq@` (the Ghost field) and `self.wf()` (which only checks `seq@.len() == N`). The concrete runtime field `ar: [A;N]` is never mentioned in any ensures clause, so the spec provides no guarantee that `ar[i]` actually changes. This is a real ghost-runtime desync gap — all verified properties hold over the ghost state only.

## All Candidates

### φ1: set_no_length_preservation_check
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** set's ensures guarantee wf() on the output, but the length preservation is derived from Seq::update semantics — if update had a different axiomatization, wf() alone wouldn't catch length changes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Length preservation after `Seq::update` is a fundamental axiom of vstd's sequence library — `update` is defined to return a sequence of the same length. This is expected behavior, not a spec gap.

### φ2: set_frame_only_from_extensional_eq
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The frame condition (untouched indices preserved) relies entirely on vstd Seq::update axioms — set's external_body provides no independent frame guarantee beyond extensional equality
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The frame condition follows directly from vstd's `Seq::update` axioms: updating index `i` preserves all other indices. The ensures clause `self.seq@ =~= old(self).seq@.update(i as int, out)` is the strongest possible specification — extensional equality IS the frame guarantee.

### φ3: set_external_body_ar_ghost_desync
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** set's external_body ensures only constrain the Ghost<Seq<A>> field — the concrete `ar: [A;N]` array is never mentioned in ensures, so ghost and runtime could desync
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `set` external_body ensures only constrain `self.seq@` (the Ghost field) and `self.wf()` (which only checks `seq@.len() == N`). The concrete runtime field `ar: [A;N]` is never mentioned in any ensures clause, so the spec provides no guarantee that `ar[i]` actually changes. This is a real ghost-runtime desync gap — all verified properties hold over the ghost state only.

### φ4: init2zero_accepts_empty_array
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** init2zero requires N <= usize::MAX which is always true for usize — the vacuous forall on empty arrays means init2zero "succeeds" with no actual work, potentially masking off-by-one issues
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty array trivially satisfies "all elements are zero" via vacuous truth. The `N <= usize::MAX` precondition is redundant but harmless. This is standard behavior for empty collections, not a spec issue.

### φ5: set_arbitrary_value_no_bounds_on_A
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** set's external_body places no domain constraint on the value being written — any A is accepted, meaning the spec trusts that the runtime array can store arbitrary values without validation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Rust arrays can store any value of type `A` — there is no domain restriction needed. For `u8`, all 256 values (0–255) are valid. The spec correctly reflects that any value of the element type can be written.

