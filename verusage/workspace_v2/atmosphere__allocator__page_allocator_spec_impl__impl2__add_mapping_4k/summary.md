# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__add_mapping_4k.rs`
**Date:** 2026-03-24T05:06:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives. Two copy-paste bugs in `container_wf` where 2m/1g container map domains are checked against `allocated_pages_4k` instead of their respective size classes, and one unverified external_body lemma (`wf_to_no_duplicates`).

## True Positives (Spec Issues)

### container_map_2m_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms this area was incomplete. Copy-paste bug.

### container_map_1g_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`.

### wf_to_no_duplicates_external_body
- **Confidence:** high
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified trust assumption. Both `wf()` and this lemma are unproved.

## All Candidates

### φ1: container_map_2m_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf checks container_map_2m domain against allocated_pages_4k instead of allocated_pages_2m — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms this area was incomplete. Copy-paste bug.

### φ2: container_map_1g_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf checks container_map_1g domain against allocated_pages_4k instead of allocated_pages_1g — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`.

### φ3: wf_to_no_duplicates_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** wf_to_no_duplicates is external_body — unverified claim that wf() implies no_duplicates on ghost sequence
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified trust assumption. Both `wf()` and this lemma are unproved.

### φ4: sll_wf_opaque_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::wf() is external_body closed spec — completely opaque, properties depending on it are unverified

### φ5: spec_len_usize_truncation
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** spec_len casts Seq::len() (nat) to usize — if the ghost sequence length exceeds usize::MAX, the cast silently truncates, making wf() vacuously false or creating unsound arithmetic

