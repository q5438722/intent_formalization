# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__kernel_kill_thread__impl0__kernel_proc_kill_all_threads.rs`
**Date:** 2026-03-24T06:49:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

All three candidates are true positives identifying the same structural spec gap across the three page size tiers. The `container_map_4k`, `container_map_2m`, and `container_map_1g` Ghost fields in PageAllocator lack any visible disjointness constraint, meaning the spec permits two distinct containers to simultaneously claim ownership of the same physical page. While the closed `wf()` spec may internally enforce disjointness, this invariant is not exposed at the spec surface, allowing reasoning in partial-wf contexts to assume double ownership is possible. This is a meaningful spec weakness for a page allocator where exclusive ownership is a fundamental safety property.

## True Positives (Spec Issues)

### container_map_4k_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness predicate in the visible spec — the ensures clause correctly demonstrates that two distinct containers can simultaneously contain the same page pointer in their respective maps. While the PageAllocator's closed `wf()` might internally enforce disjointness, the fact that this property is provable without requiring `wf()` means callers operating without the full well-formedness assumption can reason about double-owned pages.

### container_map_2m_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** Same issue as `container_map_4k` — the `container_map_2m` Ghost field lacks any visible disjointness constraint. The property is provable without requiring `wf()`, exposing that two containers can claim the same 2M page in partial-wf contexts.

### container_map_1g_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** Same pattern as the 4k and 2m variants. The `container_map_1g` Ghost field has no visible disjointness predicate, allowing two distinct containers to simultaneously claim the same 1G page without contradiction.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field as trusted length

### φ3: container_map_4k_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no disjointness predicate — two distinct containers can simultaneously claim the same physical page
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness predicate in the visible spec — the ensures clause correctly demonstrates that two distinct containers can simultaneously contain the same page pointer in their respective maps. While the PageAllocator's closed `wf()` might internally enforce disjointness, the fact that this property is provable without requiring `wf()` means callers operating without the full well-formedness assumption can reason about double-owned pages.

### φ4: container_map_2m_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_2m has the same unconstrained Ghost field issue as container_map_4k — two containers can both claim a 2M page with no contradiction
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same issue as `container_map_4k` — the `container_map_2m` Ghost field lacks any visible disjointness constraint. The property is provable without requiring `wf()`, exposing that two containers can claim the same 2M page in partial-wf contexts.

### φ5: container_map_1g_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_1g has the same unconstrained Ghost field issue — two containers can both claim a 1G page with no visible disjointness constraint
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern as the 4k and 2m variants. The `container_map_1g` Ghost field has no visible disjointness predicate, allowing two distinct containers to simultaneously claim the same 1G page without contradiction.

