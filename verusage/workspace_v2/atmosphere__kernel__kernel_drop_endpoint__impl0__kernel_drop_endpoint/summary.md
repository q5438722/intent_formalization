# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__kernel_drop_endpoint__impl0__kernel_drop_endpoint.rs`
**Date:** 2026-03-24T06:43:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property proven (`page_is_mapped(p) implies mapped_pages_4k/2m/1g contains p`) is a trivial restatement of the `page_is_mapped` definition. While the underlying concern — that mapped pages have no subset relationship to allocated pages — may be a real architectural question, this particular PHI doesn't demonstrate it; it would need to show a page simultaneously mapped and not allocated, which requires access to the `wf()` predicate's internals (which are `closed spec` / `external_body` and therefore opaque).

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) — this is trusted without proof, and if MEM_MASK has the wrong value, arbitrary addresses pass validation

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — a corrupted linked list returns its raw field value as trusted length

### φ3: pagetable_is_empty_allows_nonempty_l4_dom
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** is_empty accesses l4_table@[cr3] assuming it exists but never constrains l4_table's domain — page_closure includes l4_table.dom() which is never emptied, so an "empty" page table still holds page frames

### φ4: usize2page_entry_zero_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The external_body ensures on usize2page_entry for v==0 guarantee all perm fields are false and addr is 0, which matches is_empty() — but this relies on unverified bit-masking assumptions about MEM_MASK and the flag masks all producing 0 when applied to 0

### φ5: page_is_mapped_no_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_is_mapped is defined as the disjunction of mapped_pages_4k/2m/1g but there is no constraint requiring mapped pages to be a subset of allocated pages — a page can be "mapped" without being "allocated"
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is literally the definition of `page_is_mapped` — it's a tautology. The PHI was flagged because mapped pages aren't constrained to be a subset of allocated pages, but the property actually proven here just restates the open spec definition. It reveals no spec gap; it's definitionally true.

