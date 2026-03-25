# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/array/array__impl3__init2zero.rs`
**Date:** 2026-03-24T06:36:12Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

Four of the five candidates are false positives reflecting standard properties of vstd sequences, Rust type semantics, and vacuous truth over empty collections. The one true positive (`set_external_body_ar_ghost_desync`) identifies a real architectural gap: `Array::set`'s external_body ensures only constrain the `Ghost<Seq<A>>` field with no specification linking the concrete `ar: [A;N]` runtime field to the ghost state, meaning all verified properties hold over ghost state only with no formal guarantee of runtime correctness.

## True Positives (Spec Issues)

### set_external_body_ar_ghost_desync
- **Confidence:** medium
- **Reasoning:** The `set` external_body ensures only constrain `self.seq@` (the Ghost field). The concrete `ar: [A;N]` is never mentioned in any ensures, so there is no formal guarantee the runtime array is updated. This is a real ghost-runtime desync gap inherent to the Array abstraction.

## All Candidates

### φ1: set_external_body_ar_ghost_desync
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** set's external_body only constrains Ghost<Seq<A>> — the concrete ar:[A;N] field has no ensures linking it to the ghost state, so runtime and ghost can desync
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `set` external_body ensures only constrain `self.seq@` (the Ghost field). The concrete `ar: [A;N]` is never mentioned in any ensures, so there is no formal guarantee the runtime array is updated. This is a real ghost-runtime desync gap inherent to the Array abstraction.

### φ2: init2zero_usize_only_not_generic
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** init2zero is only implemented for Array<usize, N> — Array<u8, N> or other integer types have no zero-initialization, and empty arrays vacuously satisfy the postcondition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This PHI merely shows an empty array vacuously satisfies `wf()` — that's standard behavior. The fact that `init2zero` is only for `usize` is a design choice, not a spec gap. The property proven is trivially true and reveals no issue.

### φ3: set_overwrites_without_old_value_constraint
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two consecutive sets to the same index — the first write is silently lost with no spec-level tracking of write history or dirty flags
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just `Seq::update` semantics — updating index `i` with `v2` yields `v2` at index `i`. That two writes to the same index lose the first value is standard array behavior, not a spec gap. No system requires write-history tracking for a simple array.

### φ4: wf_only_checks_length_not_content
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** wf() only checks seq@.len() == N — two Arrays with identical ghost seqs are indistinguishable at spec level even if their concrete ar fields differ
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two arrays with `arr1.seq@ =~= arr2.seq@` have extensionally equal ghost sequences, so element-wise equality follows from `Seq` axioms. The concrete `ar` fields differing is the same ghost-runtime desync already captured by PHI 1 — this PHI itself just proves a tautology about extensional equality.

### φ5: set_no_bounds_on_value
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** set accepts any value of type A with no domain validation — usize::MAX or any sentinel value can be written without the spec constraining the value space
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Rust arrays can store any value of type `A` — for `usize`, all values including `usize::MAX` are valid. There is no domain restriction needed. This is standard `Seq::update` semantics, not a spec gap.

