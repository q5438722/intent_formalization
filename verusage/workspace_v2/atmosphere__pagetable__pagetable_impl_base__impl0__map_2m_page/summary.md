# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__map_2m_page.rs`
**Date:** 2026-03-24T07:23:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate is a true positive. The page table spec lacks cross-level disjointness constraints between the L4, L3, L2, and L1 table domains. Specifically, `wf_l4` fixes `l4_table@.dom() = {cr3}` but nothing prevents `cr3` from also appearing in `l3_tables@.dom()`, `l2_tables@.dom()`, or `l1_tables@.dom()`. This aliasing would mean hardware interprets the same physical page as entries at multiple page table levels simultaneously — a real spec gap that could mask correctness issues in page table management operations.

## True Positives (Spec Issues)

### page_closure_l4_l3_can_overlap
- **Confidence:** medium
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (which equals `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously — no cross-level disjointness predicate exists. This means the same physical page could serve as both the L4 page table and an L3 page table, which would cause hardware to interpret the same memory as two different levels of page table entries — a real structural soundness gap.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: page_entry_to_map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry contains only addr/write/execute_disable — the user/supervisor bit is silently dropped, making kernel-only and user-accessible pages indistinguishable in mappings

### φ3: page_closure_l4_l3_can_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No cross-level disjointness constraint — the same physical page can appear in both l4_table and l3_tables domains, meaning it serves as two different page table levels simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (which equals `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously — no cross-level disjointness predicate exists. This means the same physical page could serve as both the L4 page table and an L3 page table, which would cause hardware to interpret the same memory as two different levels of page table entries — a real structural soundness gap.

### φ4: page_entry_to_map_entry_drops_ps_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry does not preserve the ps (page size) bit — a 2M huge page entry and a 4K page table pointer with the same address become indistinguishable after conversion

### φ5: page_entry_to_map_entry_drops_present_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry does not preserve the present bit — non-present (invalid) entries produce the same MapEntry as present (valid) entries, allowing non-present mappings to appear valid

