# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__syscall_io_mmap__impl0__syscall_io_mmap.rs`
**Date:** 2026-03-24T06:53:33Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives. The `container_map_2m` and `container_map_1g` Ghost fields mirror the same spec gap previously identified for `container_map_4k`: all three container map fields lack disjointness constraints, allowing multiple containers to claim the same physical page simultaneously. This is a systematic gap across all page sizes in the PageAllocator spec.

## True Positives (Spec Issues)

### container_map_2m_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The `container_map_2m` Ghost field has no disjointness constraint in the visible spec. The ensures clause demonstrates that two distinct containers can both contain the same page pointer, which Verus confirmed is satisfiable. A sound page allocator should enforce exclusive 2M page ownership across containers.

### container_map_1g_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** Same pattern as container_map_2m — the `container_map_1g` Ghost field lacks any disjointness predicate. Two distinct containers can simultaneously claim the same 1G physical page, violating exclusive ownership that a page allocator should enforce.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field value as trusted length

### φ3: container_map_2m_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_2m is a Ghost field with no disjointness constraint — two distinct containers can simultaneously claim the same 2M physical page
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `container_map_2m` Ghost field has no disjointness constraint in the visible spec. The ensures clause demonstrates that two distinct containers can both contain the same page pointer, which Verus confirmed is satisfiable. A sound page allocator should enforce exclusive 2M page ownership across containers.

### φ4: container_map_1g_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_1g is a Ghost field with no disjointness constraint — two distinct containers can simultaneously claim the same 1G physical page
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern as container_map_2m — the `container_map_1g` Ghost field lacks any disjointness predicate. Two distinct containers can simultaneously claim the same 1G physical page, violating exclusive ownership that a page allocator should enforce.

### φ5: page_entry_to_map_entry_drops_present_and_user
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** spec_page_entry_to_map_entry copies addr regardless of present/user bits — a non-present user-accessible entry with stale address produces a MapEntry indistinguishable from a valid kernel mapping

