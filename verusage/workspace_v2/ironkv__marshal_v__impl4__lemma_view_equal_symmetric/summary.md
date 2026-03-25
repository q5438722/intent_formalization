# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl4__lemma_view_equal_symmetric.rs`
**Date:** 2026-03-24T09:38:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives identify the same pattern: `usize` and `Vec<u8>` implementations of `lemma_view_equal_symmetric` are unnecessarily `external_body` when symmetry of `===` is trivially provable by SMT. The remaining two are false positives — `view_equal` collapsing to equality for primitives is the expected base case, and the contrapositive of symmetry holding unconditionally is correct behavior, not a separate issue from the external_body concern.

## True Positives (Spec Issues)

### view_equal_symmetric_usize_external_body
- **Confidence:** medium
- **Reasoning:** `usize::lemma_view_equal_symmetric` is `external_body` when `self@

### view_equal_symmetric_vec_u8_external_body
- **Confidence:** medium
- **Reasoning:** Same issue — `Vec<u8>::lemma_view_equal_symmetric` is `external_body` when `self@

## All Candidates

### φ1: view_equal_symmetric_usize_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::lemma_view_equal_symmetric` is `external_body` despite being trivially provable from `===` symmetry — unnecessarily trusted assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `usize::lemma_view_equal_symmetric` is `external_body` when `self@

### φ2: view_equal_symmetric_vec_u8_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<u8>::lemma_view_equal_symmetric` is `external_body` despite being trivially provable from `===` symmetry — unnecessarily trusted assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same issue — `Vec<u8>::lemma_view_equal_symmetric` is `external_body` when `self@

### φ3: view_equal_implies_equal_usize
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** For usize, `view_equal` (defined as `self@ === other@`) collapses to full equality — the abstraction barrier provides no separation from `==` for primitive types
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For primitive types like usize, `view_equal` collapsing to `==` is expected and correct. The `view_equal` abstraction exists for compound types (like `Vec<T>`) where structural equality may differ from view equality; primitives are the base case.

### φ4: trait_symmetry_no_preconditions
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The trait-level `lemma_view_equal_symmetric` has no preconditions at all — any implementation's `external_body` version is trusted to maintain symmetry with zero verification obligation, even for complex types where symmetry might genuinely fail
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just the contrapositive of φ1 (symmetry). The trait-level lemma having no preconditions is correct — symmetry of an equivalence relation should hold unconditionally. The real concern (external_body trust) is already captured by φ1/φ2.

### φ5: vec_t_view_equal_not_transitive
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `view_equal` for `Vec<T>` is defined via element-wise `view_equal` — transitivity holds by accident for usize (where view_equal is `===`) but the trait never requires or proves transitivity, so implementations could provide a non-transitive `view_equal` without violating any trait-level specification

