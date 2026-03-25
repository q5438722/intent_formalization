# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__kernel_kill_thread__impl0__kernel_kill_thread.rs`
**Date:** 2026-03-24T06:50:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One of three candidates is a true positive. The `container_map_4k_allows_double_ownership` PHI meaningfully demonstrates that the Ghost field lacks disjointness constraints, allowing two distinct containers to claim the same page — a real spec gap for a page allocator. The other two are false positives: `is_empty_kernel_entries_unchecked` has a tautological ensures clause and describes intentional kernel mapping design, while `container_map_not_subset_allocated` trivially restates its precondition without demonstrating any actual subset violation.

## True Positives (Spec Issues)

### container_map_4k_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The PHI correctly demonstrates that two distinct containers can both contain the same page pointer in their respective maps — the ensures clause is not a tautology since it conjoins both containment facts under the `c1 != c2` precondition, and Verus confirmed this is satisfiable. A sound page allocator should enforce exclusive page ownership.

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
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The PHI correctly demonstrates that two distinct containers can both contain the same page pointer in their respective maps — the ensures clause is not a tautology since it conjoins both containment facts under the `c1 != c2` precondition, and Verus confirmed this is satisfiable. A sound page allocator should enforce exclusive page ownership.

### φ4: is_empty_kernel_entries_unchecked
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** PageTable::is_empty only constrains L4 entries from kernel_l4_end..512 — kernel-range entries (0..kernel_l4_end) are completely unchecked, allowing arbitrary non-empty entries in the "empty" page table's kernel region
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a trivial tautology (`x == x`). It proves nothing about kernel entries being unchecked. Moreover, kernel-range L4 entries being unconstrained in `is_empty` is intentional design — kernel mappings are shared across address spaces and persist regardless of whether a process's user-space page table is "empty."

### φ5: container_map_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k entries have no visible constraint requiring membership in allocated_pages_4k — a container can claim an unallocated page
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause trivially restates the precondition (`container_map_4k@[c].contains(p)` implies itself). This proves nothing about the relationship between container maps and allocated pages. While the observation about missing subset constraints may be valid, this PHI doesn't demonstrate it.

