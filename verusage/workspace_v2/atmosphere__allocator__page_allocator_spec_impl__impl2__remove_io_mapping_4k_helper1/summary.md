# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__remove_io_mapping_4k_helper1.rs`
**Date:** 2026-03-24T05:12:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives: two copy-paste bugs in `container_wf` (2m/1g domains checked against `allocated_pages_4k`), and one unverified external_body lemma. One false positive: mapped pages having nonzero refcount is an explicitly intended invariant.

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

### φ4: free_4k_not_in_allocated_4k
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** spec should enforce disjointness between free and allocated sets but this relies on page state being unique per page — if the invariant is incomplete, a page could appear in both sets

### φ5: mapped_page_has_nonzero_refcount
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** mapped pages must have nonzero ref_count — if the spec allows ref_count == 0 for mapped pages, unmapping logic could silently lose track of mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `mapped_pages_have_reference_counter` explicitly specifies `ref_count != 0 <==> Mapped*`, so nonzero refcount for mapped pages is a correct, intended property.

