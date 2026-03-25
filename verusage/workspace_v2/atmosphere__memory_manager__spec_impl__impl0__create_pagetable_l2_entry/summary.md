# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/memory_manager/memory_manager__spec_impl__impl0__create_pagetable_l2_entry.rs`
**Date:** 2026-03-24T07:06:52Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. `page_closure_includes_l4_table` restates the open spec definition of page_closure, which intentionally includes L4 table pointers. `is_empty_allows_nonempty_kernel_entries` has a tautological ensures clause that proves nothing about the spec. `resolve_l4_ignores_kernel_region_present` reflects deliberate x86-64 design where kernel-region L4 entries are always treated as valid mappings to ensure kernel address space is always resolvable.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: zero_entry_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The external_body on usize2page_entry ensures all fields are false/zero when v==0, combined with spec_usize2pa(0)==0, making is_empty() trivially true — but this relies on two independent external_body claims agreeing

### φ3: page_closure_includes_l4_table
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_closure unconditionally absorbs all l4_table domain pointers — if l4_table domain overlaps with l3/l2/l1 domains, page_closure silently merges them with no disjointness check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from the open spec definition of `page_closure`, which is `l3_tables@.dom() + l2_tables@.dom() + l1_tables@.dom() + l4_table@.dom()`. Including l4_table in page_closure is intentional — the page table owns its L4 page. Disjointness between table levels is a separate invariant concern, not a defect in page_closure's definition.

### φ4: is_empty_allows_nonempty_kernel_entries
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** is_empty only constrains indices >= kernel_l4_end — kernel region L4 entries are completely unconstrained, allowing present entries with arbitrary mappings in a supposedly "empty" page table
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `present || !present` is a tautology (P ∨ ¬P) that holds for any boolean regardless of spec content. While the underlying observation about kernel entries being unconstrained by `is_empty` is valid design intent (kernel mappings are shared/persistent), this PHI proves nothing because its ensures is trivially true.

### φ5: resolve_l4_ignores_kernel_region_present
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** spec_resolve_mapping_l4 returns Some for ALL kernel-region indices (l4i < kernel_l4_end) regardless of the present bit — a non-present kernel entry is still treated as a valid mapping, bypassing the present-bit check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional x86-64 page table design. The kernel region (indices < `kernel_l4_end`) contains shared kernel mappings that must always be traversable regardless of the present bit — `spec_resolve_mapping_l4` deliberately returns `Some` for `l4i < kernel_l4_end` to ensure kernel address resolution always succeeds. This is a desirable property, not a spec gap.

