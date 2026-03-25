# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__schedule_idle_cpu__impl0__schedule_idle_cpu.rs`
**Date:** 2026-03-24T06:52:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive and one false positive. The `container_map_4k_allows_double_ownership` PHI identifies a genuine spec gap: the Ghost field `container_map_4k` lacks any disjointness predicate, allowing two containers to claim the same physical page simultaneously. The `is_empty_kernel_entries_unconstrained` PHI is a false positive — its ensures clause is a tautology that proves nothing, and the underlying design choice (kernel entries unconstrained by `is_empty`) is intentional since kernel mappings are shared infrastructure.

## True Positives (Spec Issues)

### container_map_4k_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The ensures clause correctly demonstrates that two distinct containers can both contain the same page pointer under the given preconditions — Verus confirmed this is satisfiable. A sound page allocator should enforce exclusive page ownership across containers.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field value as trusted length

### φ3: page_entry_to_map_entry_drops_present
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** spec_page_entry_to_map_entry copies addr even when present==false — non-present entries with stale addresses produce MapEntries indistinguishable from valid mappings

### φ4: container_map_4k_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no disjointness constraint — two distinct containers can simultaneously claim the same physical page
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `container_map_4k` Ghost field has no disjointness constraint in the visible spec. The ensures clause correctly demonstrates that two distinct containers can both contain the same page pointer under the given preconditions — Verus confirmed this is satisfiable. A sound page allocator should enforce exclusive page ownership across containers.

### φ5: is_empty_kernel_entries_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** PageTable::is_empty only constrains L4 entries at indices >= kernel_l4_end — kernel-range entries (0..kernel_l4_end) are completely unconstrained and could contain arbitrary mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a trivial tautology (`x == x`), proving nothing about kernel entries being unconstrained. Moreover, kernel-range L4 entries being unconstrained in `is_empty` is intentional design — kernel mappings are shared across all address spaces and persist regardless of whether a process's user-space page table is "empty."

