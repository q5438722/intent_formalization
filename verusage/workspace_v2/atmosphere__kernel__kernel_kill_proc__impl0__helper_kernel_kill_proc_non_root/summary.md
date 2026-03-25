# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_non_root.rs`
**Date:** 2026-03-24T06:46:59Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Two (`page_is_mapped_no_subset_allocated` and `container_map_4k_unconstrained_ghost`) have trivially true ensures clauses — one restates the definition of `page_is_mapped` and the other is a logical tautology (`!x || x`). The third (`is_empty_allows_nonempty_page_closure`) correctly shows the cr3 page remains in page_closure when is_empty holds, but this is intentional: the L4 root page directory is structural infrastructure that persists regardless of whether user-space mappings exist. While some of the underlying observations (mapped-not-subset-allocated, unconstrained Ghost fields) may point to real spec gaps, the PHI candidates as written fail to meaningfully demonstrate them.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values are treated as having valid physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field as trusted length

### φ3: page_is_mapped_no_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_is_mapped is definitionally the disjunction of mapped_pages_4k/2m/1g but there is no visible constraint requiring mapped pages to be a subset of allocated pages — a page can be mapped without being allocated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is literally the definition of `page_is_mapped` — it's a tautology. The PHI restates the open spec definition without demonstrating any actual gap. The observation about mapped-not-subset-of-allocated may be valid but this property doesn't prove it.

### φ4: is_empty_allows_nonempty_page_closure
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An "empty" page table still has cr3 in its page_closure since l4_table dom is never emptied by is_empty — the page_closure of an empty page table is non-empty, breaking the intuition that empty means no resources held
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is expected design. The L4 table (containing the cr3 root page) is always present — it's the top-level page directory that must exist for any address space. An "empty" page table means no user-space mappings exist, not that the page table infrastructure itself is deallocated. The cr3 page is a structural requirement, not a mapping resource.

### φ5: container_map_4k_unconstrained_ghost
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no disjointness or subset-of-allocated constraint — two different containers can claim the same page in their container maps simultaneously
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a trivial tautology (`!x || x` is always true). It proves nothing about whether two containers can share pages. The observation about missing disjointness constraints on `container_map_4k` may be valid, but this PHI doesn't demonstrate it — it would verify regardless of any constraints.

