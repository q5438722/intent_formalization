# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__add_io_mapping_4k.rs`
**Date:** 2026-03-24T05:05:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives. The `wf_to_no_duplicates` external_body lemma is an unverified trust assumption. More significantly, `container_wf` contains a copy-paste bug where `container_map_2m` and `container_map_1g` domains are both checked against `allocated_pages_4k` instead of their respective `allocated_pages_2m` and `allocated_pages_1g` — confirmed by the developer's own `@Xiangdong Come back for this` comment indicating the spec was still under development.

## True Positives (Spec Issues)

### wf_to_no_duplicates_external_body
- **Confidence:** high
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified claim that `wf()` implies `no_duplicates()` on the ghost sequence. Both `wf()` itself and this lemma are unproved.

### container_map_2m_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The commented-out code and the `@Xiangdong Come back for this` note confirm this area was incomplete. This is a copy-paste bug where 2m container map domain should track 2m allocations.

### container_map_1g_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`. The spec fails to enforce that 1g container mappings reference 1g-allocated pages.

## All Candidates

### φ1: usize_u64_roundtrip_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** lemma_usize_u64 is external_body with no requires — claims ALL u64 values roundtrip through usize without proof

### φ2: sll_wf_external_body_opaque
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::wf() is external_body closed spec — completely opaque, any property could be vacuously true if wf() is unsatisfiable

### φ3: wf_to_no_duplicates_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** wf_to_no_duplicates is external_body — unverified claim that wf() implies no_duplicates on the ghost sequence
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `wf_to_no_duplicates` is external_body — unverified claim that `wf()` implies `no_duplicates()` on the ghost sequence. Both `wf()` itself and this lemma are unproved.

### φ4: container_map_2m_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf asserts container_map_2m domain is subset of allocated_pages_4k instead of allocated_pages_2m — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The commented-out code and the `@Xiangdong Come back for this` note confirm this area was incomplete. This is a copy-paste bug where 2m container map domain should track 2m allocations.

### φ5: container_map_1g_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf asserts container_map_1g domain is subset of allocated_pages_4k instead of allocated_pages_1g — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`. The spec fails to enforce that 1g container mappings reference 1g-allocated pages.

