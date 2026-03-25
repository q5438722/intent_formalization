# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__alloc_and_map_2m.rs`
**Date:** 2026-03-24T05:07:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives. Two copy-paste bugs in `container_wf` where 2m/1g container map domains are checked against `allocated_pages_4k` instead of their respective size-class sets, and one unverified external_body lemma. The copy-paste bugs are confirmed by the developer's own TODO comment.

## True Positives (Spec Issues)

### container_map_2m_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** `container_wf` checks `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms incomplete spec. Copy-paste bug.

### container_map_1g_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should reference `allocated_pages_1g@`.

### wf_to_no_duplicates_external_body
- **Confidence:** high
- **Reasoning:** `wf_to_no_duplicates` is external_body with `unimplemented!()` body — unverified trust assumption. Both `wf()` itself and this lemma are opaque and unproved.

## All Candidates

### φ1: container_map_2m_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_wf checks container_map_2m domain against allocated_pages_4k instead of allocated_pages_2m — likely copy-paste bug
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_wf` checks `container_map_2m@.dom().subset_of(allocated_pages_4k@)` instead of `allocated_pages_2m@`. The `@Xiangdong Come back for this` comment confirms incomplete spec. Copy-paste bug.

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
- **Reasoning:** `wf_to_no_duplicates` is external_body with `unimplemented!()` body — unverified trust assumption. Both `wf()` itself and this lemma are opaque and unproved.

### φ4: pop_external_body_node_ref_preserved
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** pop's external_body postcondition claims node refs are preserved after mutation — unverified that internal linked list indices remain stable across structural changes

### φ5: sll_wf_opaque_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::wf() is external_body closed spec — completely opaque, properties gated on it are unverified trust assumptions

