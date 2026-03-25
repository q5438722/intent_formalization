# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/array/array__impl4__init2none.rs`
**Date:** 2026-03-24T06:37:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_external_body_ar_ghost_desync
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** set's external_body only constrains Ghost<Seq<A>> — the concrete ar:[A;N] field has no ensures linking it to the ghost state, so runtime and ghost can desync

### φ2: init2none_vacuous_on_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** init2none on a zero-length array vacuously satisfies the postcondition with no work — the N <= usize::MAX guard is always true, potentially masking edge cases

### φ3: set_no_value_domain_constraint
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** set accepts any value of type A with no domain validation — arbitrary sentinel values can be written without the spec constraining the value space

### φ4: wf_only_checks_length
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** wf() only checks seq@.len() == N — two Arrays with identical ghost seqs are indistinguishable at spec level even if their concrete ar fields differ entirely

### φ5: set_double_write_loses_first
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two consecutive sets to the same index — the first write is silently lost with no spec-level tracking, and the spec provides no mechanism to detect or prevent overwrites

