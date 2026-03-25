# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__alloc_page_4k.rs`
**Date:** 2026-03-24T05:09:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

Five true positives. Two copy-paste bugs in `container_wf` (2m/1g domains checked against `allocated_pages_4k`). Three external_body lemmas forming unverified trust assumptions — `wf_to_no_duplicates` (opaque invariant), `seq_skip_lemma` (sequence properties), and `page_ptr_lemma1` (pointer arithmetic injectivity, likely provable natively).

## True Positives (Spec Issues)

### container_map_2m_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** `container_wf` checks `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms incomplete spec.

### container_map_1g_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should reference `allocated_pages_1g@`.

### wf_to_no_duplicates_external_body
- **Confidence:** high
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified trust assumption. Both `wf()` and this lemma are opaque and unproved.

### seq_skip_lemma_external_body
- **Confidence:** medium
- **Reasoning:** `seq_skip_lemma` is external_body — unverified axiom about `Seq::skip`. Likely true but remains an unproved trust assumption.

### page_ptr_lemma1_external_body
- **Confidence:** low
- **Reasoning:** `page_ptr_lemma1` is external_body, but the claimed injectivity of `page_index2page_ptr` follows directly from its definition `(i * 4096)` — this is provable from basic arithmetic. Trust assumption rather than spec bug, but still unverified.

## All Candidates

### φ1: container_map_2m_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf checks container_map_2m domain against allocated_pages_4k instead of allocated_pages_2m — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_wf` checks `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms incomplete spec.

### φ2: container_map_1g_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf checks container_map_1g domain against allocated_pages_4k instead of allocated_pages_1g — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should reference `allocated_pages_1g@`.

### φ3: wf_to_no_duplicates_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** wf_to_no_duplicates is external_body — unverified claim that wf() implies no_duplicates
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified trust assumption. Both `wf()` and this lemma are opaque and unproved.

### φ4: seq_skip_lemma_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_skip_lemma is external_body — unverified axiom about Seq::skip used throughout allocator proofs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `seq_skip_lemma` is external_body — unverified axiom about `Seq::skip`. Likely true but remains an unproved trust assumption.

### φ5: page_ptr_lemma1_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_ptr_lemma1 is external_body — unverified axiom about page pointer injectivity that the entire allocator correctness depends on
- **Verdict:** TRUE_POSITIVE (low)
- **Reasoning:** `page_ptr_lemma1` is external_body, but the claimed injectivity of `page_index2page_ptr` follows directly from its definition `(i * 4096)` — this is provable from basic arithmetic. Trust assumption rather than spec bug, but still unverified.

