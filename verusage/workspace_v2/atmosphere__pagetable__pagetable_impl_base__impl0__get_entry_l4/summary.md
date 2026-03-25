# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__get_entry_l4.rs`
**Date:** 2026-03-24T07:20:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 0

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: page_entry_to_map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** page_entry_to_map_entry silently drops the user/supervisor bit — kernel-only and user-accessible pages become indistinguishable in MapEntry

### φ3: page_entry_to_map_entry_drops_present_and_ps
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry is identical whether the source PageEntry is present or not, and whether ps is set — non-present and huge-page entries are conflated with normal present entries

### φ4: wf_l4_no_cross_level_disjointness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** There is no cross-level disjointness check — an L3 table pointer could equal cr3, meaning the same physical page serves as both L4 and L3 table simultaneously

### φ5: l4_kernel_region_no_present_check
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** wf_l4 imposes no constraints on kernel-region L4 entries (indices below kernel_l4_end) — they can be present, non-present, have ps set, or point to arbitrary addresses with no well-formedness checks

