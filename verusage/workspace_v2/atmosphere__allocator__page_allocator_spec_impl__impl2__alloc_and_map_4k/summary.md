# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__alloc_and_map_4k.rs`
**Date:** 2026-03-24T05:08:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

Four true positives. Two copy-paste bugs in `container_wf` where 2m/1g container map domains are checked against `allocated_pages_4k` instead of their respective size-class sets. Two unverified external_body lemmas (`wf_to_no_duplicates` and `seq_skip_lemma`) that form trust assumptions in the allocator's proof chain.

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
- **Reasoning:** `seq_skip_lemma` is external_body — unverified axiom about `Seq::skip` semantics. The claimed properties are likely true but remain unproved trust assumptions in the verification chain.

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
- **Why flagged:** wf_to_no_duplicates is external_body — unverified claim that wf() implies no_duplicates on ghost sequence
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified trust assumption. Both `wf()` and this lemma are opaque and unproved.

### φ4: pop_preserves_node_ref_external_body
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** pop's external_body postcondition claims node indices are stable across structural linked list mutations — unverified

### φ5: seq_skip_lemma_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_skip_lemma is external_body — unverified axiom about Seq::skip behavior used throughout the allocator proofs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `seq_skip_lemma` is external_body — unverified axiom about `Seq::skip` semantics. The claimed properties are likely true but remain unproved trust assumptions in the verification chain.

