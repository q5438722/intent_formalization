# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__merged_4k_to_2m.rs`
**Date:** 2026-03-24T06:33:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: container_map_4k_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k tracks per-container page ownership but wf() may not require these pages to actually be in allocated_pages_4k — containers could claim unallocated pages

### φ2: push_remove_roundtrip_len_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Verifies push+remove roundtrip preserves length — trivially true by arithmetic but validates the len ensures are consistent

### φ3: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — the raw field value is trusted even if the linked list is corrupted

### φ4: mapped_page_no_mappings_tracked
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A page can be in Mapped4k state yet have an empty mappings set — the spec may not require mapped pages to have at least one mapping recorded

### φ5: remove_preserves_get_node_ref_for_removed
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** remove's ensures preserves get_node_ref for remaining elements — validates the node reference stability invariant across removal

