# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__get_entry_l1.rs`
**Date:** 2026-03-24T07:17:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive and one false positive. The `page_entry_to_map_entry_drops_user_bit` finding identifies a real spec gap: `MapEntry` lacks a `user` field, so the user/supervisor permission bit is silently discarded when converting `PageEntry` to `MapEntry`, making kernel-only and user-accessible pages indistinguishable in the mapping abstraction. The `l4_dom_singleton_cr3` finding is an expected structural property of the x86-64 page table design where each PageTable struct owns exactly one L4 table.

## True Positives (Spec Issues)

### page_entry_to_map_entry_drops_user_bit
- **Confidence:** medium
- **Reasoning:** The `MapEntry` struct contains only `addr`, `write`, and `execute_disable` — it omits the `user` bit entirely. This means the mapping layer cannot distinguish user-accessible from kernel-only pages, which is a real spec gap: page table mappings used for access control checks lose the user/supervisor distinction.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: zero_entry_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Relies on two independent external_body claims (usize2page_entry and usize2pa) both correctly handling zero — if either is wrong the combined property is unsound

### φ3: page_entry_to_map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** page_entry_to_map_entry produces identical MapEntry regardless of the user bit — the user/supervisor permission is silently dropped during translation, meaning kernel-only and user-accessible pages become indistinguishable in mappings
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `MapEntry` struct contains only `addr`, `write`, and `execute_disable` — it omits the `user` bit entirely. This means the mapping layer cannot distinguish user-accessible from kernel-only pages, which is a real spec gap: page table mappings used for access control checks lose the user/supervisor distinction.

### φ4: page_entry_to_map_entry_drops_present_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** page_entry_to_map_entry converts non-present entries into MapEntry without checking present bit — a non-present page table entry produces a valid-looking MapEntry that could be used in mappings

### φ5: l4_dom_singleton_cr3
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** wf_l4 constrains l4_table domain to exactly {cr3} — only one L4 table exists per PageTable, making PCID-based multi-address-space sharing at L4 level impossible
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from `wf_l4` which explicitly sets `self.l4_table@.dom() =~= Set::empty().insert(self.cr3)`. A single L4 table per PageTable struct is the intended x86-64 design — each process/address space has its own PageTable struct with its own cr3, and PCID switching swaps the entire struct.

