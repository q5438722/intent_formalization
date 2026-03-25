# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__free_page_4k.rs`
**Date:** 2026-03-24T05:11:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: two copy-paste bugs in `container_wf` (2m/1g domains checked against `allocated_pages_4k`), and one unverified external_body lemma (`wf_to_no_duplicates`). Two false positives: both are tautologies that test no actual spec property.

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

### φ4: push_preserves_old_node_refs
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** push ensures old node refs are preserved but get_node_ref is external_body/closed — no way to verify the claimed ref stability
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a tautology (`old_ref == old_ref`). This proves nothing about the spec — it's trivially true for any value.

### φ5: alloc_new_container_inserts_into_all_maps
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** alloc_page_4k_for_new_container inserts a 4k page_ptr as container key into container_map_2m and container_map_1g — using a 4k page pointer as a container identifier across all size classes is suspicious
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Map::insert(k, v).dom().contains(k)` is a tautology of vstd's Map — inserting a key always makes it present in the domain. This tests no spec property.

