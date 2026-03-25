# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__syscall_new_thread_with_endpoint__impl0__syscall_new_thread_with_endpoint.rs`
**Date:** 2026-03-24T06:59:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Two (`container_map_4k_allows_double_ownership` and `pagetable_is_empty_allows_nonempty_l4_dom`) have tautological ensures clauses (P ∨ ¬P patterns) that are trivially true regardless of the spec, making them uninformative. The third (`page_closure_includes_l4_table`) correctly derives a property from the open spec definition of `page_closure`, but this is desirable by design — the L4 table pages are intentionally part of the page closure. The underlying concerns (missing disjointness on container_map, vacuous forall on is_empty) are real but the PHI formulations fail to capture them with non-trivial ensures clauses.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures l == self.value_list_len without requiring wf() — the raw field is exposed regardless of internal consistency

### φ3: container_map_4k_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is an unconstrained Ghost field with no disjointness invariant — two distinct containers can claim the same page since no predicate in the visible spec prevents it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `pa.container_map_4k@[c2].contains(p) || !pa.container_map_4k@[c2].contains(p)` is a tautology (P ∨ ¬P) — it is trivially true for any boolean expression regardless of the spec. This PHI proves nothing about double ownership or missing disjointness constraints.

### φ4: pagetable_is_empty_allows_nonempty_l4_dom
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When kernel_l4_end == 512 the forall in is_empty is vacuously true — the L4 table can contain arbitrary non-empty entries since no user-space index is checked
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `pt.l4_table@.dom().len() > 0 || pt.l4_table@.dom().len() == 0` is a tautology — every natural number is either zero or positive. This proves nothing about the vacuous forall when kernel_l4_end == 512.

### φ5: page_closure_includes_l4_table
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** page_closure includes l4_table domain unconditionally — if l4_table contains unexpected pointers (e.g. overlapping with l3/l2/l1 tables), page_closure silently absorbs them without checking disjointness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of the open spec definition of `page_closure`, which explicitly includes `self.l4_table@.dom()` as a union component. Including the L4 table in the page closure is intentional — it tracks all page table pages. The disjointness concern would need a separate PHI with non-tautological ensures.

