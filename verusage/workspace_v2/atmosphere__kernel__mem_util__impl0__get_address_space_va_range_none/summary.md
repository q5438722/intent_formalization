# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__mem_util__impl0__get_address_space_va_range_none.rs`
**Date:** 2026-03-24T06:51:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives. The `page_entry_to_map_entry_drops_present` PHI identifies a genuine abstraction leak: the conversion from PageEntry to MapEntry discards the `present` bit, meaning stale non-present entries with non-zero addresses produce MapEntries indistinguishable from valid mappings. The `container_map_4k_allows_double_ownership` PHI identifies that the Ghost field `container_map_4k` lacks disjointness constraints, permitting two containers to claim the same physical page — a real spec gap for exclusive ownership in a page allocator.

## True Positives (Spec Issues)

### page_entry_to_map_entry_drops_present
- **Confidence:** medium
- **Reasoning:** `spec_page_entry_to_map_entry` constructs a `MapEntry` with `addr`, `write`, and `execute_disable` but drops `present`, `ps`, and `user` fields. When a non-present entry has a stale non-zero address, the resulting MapEntry carries that address with no indication the entry isn't present. Consumers of MapEntry cannot distinguish a valid mapping from a stale one, which is a real information loss in the spec abstraction.

### container_map_4k_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The ensures clause correctly demonstrates that two distinct containers can simultaneously contain the same page, which is satisfiable given the preconditions. A sound page allocator should enforce exclusive page ownership across containers.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field as trusted length

### φ3: page_entry_to_map_entry_drops_present
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** page_entry_to_map_entry copies addr even when present==false — a non-present entry with a stale address produces a MapEntry with that address, losing the "not present" information
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `spec_page_entry_to_map_entry` constructs a `MapEntry` with `addr`, `write`, and `execute_disable` but drops `present`, `ps`, and `user` fields. When a non-present entry has a stale non-zero address, the resulting MapEntry carries that address with no indication the entry isn't present. Consumers of MapEntry cannot distinguish a valid mapping from a stale one, which is a real information loss in the spec abstraction.

### φ4: container_map_4k_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no disjointness predicate — two distinct containers can simultaneously claim the same physical page
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The ensures clause correctly demonstrates that two distinct containers can simultaneously contain the same page, which is satisfiable given the preconditions. A sound page allocator should enforce exclusive page ownership across containers.

### φ5: map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** spec_page_entry_to_map_entry discards the user permission bit — MapEntry has no user field, so user/supervisor distinction is lost when converting page entries to map entries used in mapping specs

