# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__create_and_share_pages__impl0__range_create_and_share_mapping.rs`
**Date:** 2026-03-24T06:45:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives due to trivial tautological ensures clauses (`x == x`). Neither PHI actually demonstrates the spec gaps described in their reasoning. The `is_empty_kernel_entries_unconstrained` property merely asserts a kernel L4 entry equals itself, and the `container_map_4k_not_subset_allocated` property asserts a map value equals itself. To be meaningful, these would need ensures clauses that actually expose the claimed gaps — e.g., showing a concrete inconsistency or deriving a property that should not hold.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — any arbitrary usize is treated as having a valid physical address after masking, which is trusted without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — a corrupted linked list returns its raw field value as trusted length

### φ3: is_empty_allows_nonempty_l4_dom
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** is_empty empties l3/l2/l1 tables but page_closure always includes l4_table dom — an "empty" page table still has a non-empty page_closure if l4_table dom is non-empty, breaking the intuition that empty means no pages in use

### φ4: is_empty_kernel_entries_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** is_empty only constrains L4 entries from kernel_l4_end..512 — kernel-range entries (0..kernel_l4_end) are completely unconstrained and can have arbitrary present entries while the page table is considered "empty"
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a trivial tautology (`x == x`), proving nothing about the kernel entries being unconstrained. Moreover, kernel-range L4 entries being unconstrained in `is_empty` is intentional design — kernel mappings are shared across all address spaces and persist regardless of whether a process's user-space page table is "empty."

### φ5: container_map_4k_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no constraining predicate visible in the spec — containers can claim arbitrary pages in their map without any subset-of-allocated constraint in the exposed wf()
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is again a trivial tautology (`x == x`), which proves nothing about any actual spec gap. While the observation about `container_map_4k` being an unconstrained Ghost field may be valid, this PHI candidate does not demonstrate it — it would verify regardless of whether constraining predicates exist or not.

