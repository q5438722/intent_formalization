# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__create_and_share_pages__impl0__share_mapping.rs`
**Date:** 2026-03-24T06:44:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The behavior where `spec_resolve_mapping_l4` returns `Some` for all L4 indices below `kernel_l4_end` regardless of the present bit is an intentional design choice reflecting x86-64 page table architecture: kernel address space entries in the L4 table are shared across all processes and always considered valid/resolved. The spec correctly distinguishes kernel-range entries (always resolved) from user-range entries (resolved only when present).

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — this is trusted without proof, meaning any arbitrary usize is treated as having a valid physical address after masking

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — a corrupted linked list returns its raw field as trusted length

### φ3: pagetable_is_empty_ignores_kernel_l4_entries
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** is_empty only checks L4 entries from kernel_l4_end..512 — kernel-range L4 entries (0..kernel_l4_end) can have arbitrary present entries while the page table is considered "empty", and page_closure always includes l4_table dom

### φ4: resolve_l4_returns_non_present_for_kernel_range
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** spec_resolve_mapping_l4 returns Some for ANY l4i < kernel_l4_end regardless of whether the entry is present — kernel-range entries are always "resolved" even if they contain garbage data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional design. The `spec_resolve_mapping_l4` function explicitly includes the condition `l4i < self.kernel_l4_end` as an alternative to `perm.present` — kernel-range L4 entries are always considered resolved because they represent the kernel's memory mappings which are always present by construction. The `recommends` clause (`self.kernel_l4_end <= l4i < 512`) indicates the function is primarily intended for user-space indices, and the kernel-range handling is a deliberate catch-all for kernel entries.

### φ5: resolve_l3_skips_ps_check
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** spec_resolve_mapping_l3 returns Some even when the L3 entry has ps=true (indicating a 1G huge page) — the resolution continues to L3 level without checking whether the entry is a leaf, allowing the same entry to be interpreted as both a 1G mapping and a pointer to an L2 table

