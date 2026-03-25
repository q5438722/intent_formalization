# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_spec__impl1__ps_entries_exist_in_mapped_pages_l3.rs`
**Date:** 2026-03-24T07:26:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives were found. The `va_1g_valid` copy-paste bug (high confidence) causes `when_used_as_spec(spec_va_2m_valid)` instead of `spec_va_1g_valid`, silently weakening all 1G address validation throughout the spec including `wf_mapping_1g`. The missing cross-level disjointness between page table level domains (medium confidence) allows the same physical page to appear in both `l4_table` and `l3_tables` simultaneously. The kernel-region resolve behavior is intentional design — the kernel region is always considered present regardless of the actual entry's present bit.

## True Positives (Spec Issues)

### page_closure_l4_l3_can_overlap
- **Confidence:** medium
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (which equals `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously — no cross-level disjointness predicate exists. This means the same physical page could serve as both the L4 and an L3 page table, which would cause hardware to interpret the same memory at two different levels.

### wf_mapping_1g_va_1g_valid_is_2m_valid
- **Confidence:** high
- **Reasoning:** `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` — a clear copy-paste bug from `va_2m_valid`. This makes `va_1g_valid(va)` in spec context evaluate `spec_va_2m_valid(va)`, so `wf_mapping_1g`'s constraint that all mapped VAs satisfy `va_1g_valid` is silently weakened to only require 2M alignment instead of 1G alignment.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: resolve_l4_returns_some_for_kernel_region
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** spec_resolve_mapping_l4 returns Some for ALL kernel-region indices (l4i < kernel_l4_end) regardless of whether the entry is actually present — the `|| l4i < self.kernel_l4_end` branch bypasses the present-bit check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional behavior. The `spec_resolve_mapping_l4` function deliberately returns `Some` for kernel-region indices (`l4i < kernel_l4_end`) regardless of the present bit. The kernel region is shared across all address spaces and is always considered "resolved" — the spec models this by unconditionally returning the entry for kernel indices.

### φ3: page_closure_l4_l3_can_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No cross-level disjointness constraint — the same physical page can appear in both l4_table and l3_tables domains simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (which equals `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously — no cross-level disjointness predicate exists. This means the same physical page could serve as both the L4 and an L3 page table, which would cause hardware to interpret the same memory at two different levels.

### φ4: page_not_mapped_ignores_ps_entries
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** page_not_mapped only checks mapping_4k/2m/1g Ghost fields — with empty mappings it vacuously returns true even if page table entries at L3/L2/L1 actually reference pa, decoupling the "not mapped" predicate from actual table contents

### φ5: wf_mapping_1g_va_1g_valid_is_2m_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** va_1g_valid has `when_used_as_spec(spec_va_2m_valid)` — a copy-paste bug that makes va_1g_valid resolve to spec_va_2m_valid, so wf_mapping_1g's constraint that mapped addresses satisfy va_1g_valid is silently weakened to 2M alignment
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` — a clear copy-paste bug from `va_2m_valid`. This makes `va_1g_valid(va)` in spec context evaluate `spec_va_2m_valid(va)`, so `wf_mapping_1g`'s constraint that all mapped VAs satisfy `va_1g_valid` is silently weakened to only require 2M alignment instead of 1G alignment.

