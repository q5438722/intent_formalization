# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/memory_manager/memory_manager__spec_impl__impl0__create_pagetable_l3_entry.rs`
**Date:** 2026-03-24T07:08:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. `no_mapping_infer_trivial_from_empty` is a trivially correct consequence of vacuous quantification over empty mapping domains. `page_closure_includes_l4_table` directly restates the open spec definition. `resolve_l4_kernel_ignores_present` reflects deliberate x86-64 design where kernel L4 entries are always resolvable.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: zero_entry_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Relies on two independent external_body claims (usize2page_entry and usize2pa both special-case v==0) agreeing — if either is wrong the combined property is unsound

### φ3: no_mapping_infer_trivial_from_empty
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** no_mapping_infer_not_mapped's external_body lets us derive page_not_mapped for ANY address from empty mappings — the proof lemma is trusted without verification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When all three mapping tables are empty, `page_not_mapped(p)` is trivially true by the open spec definition — the forall quantifiers over empty domains are vacuously satisfied. The external_body lemma `no_mapping_infer_not_mapped` is not even needed here; this is a mathematically correct and desirable property.

### φ4: page_closure_includes_l4_table
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_closure unconditionally absorbs all l4_table domain pointers without checking disjointness against l3/l2/l1 tables — overlapping domains are silently merged
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from the open spec definition of `page_closure`, which is the union of l3, l2, l1, and l4 table domains. Including L4 table pointers in the page closure is intentional — the page table owns its L4 page. Disjointness between levels is a separate concern not addressed by page_closure's definition.

### φ5: resolve_l4_kernel_ignores_present
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** spec_resolve_mapping_l4 returns Some for ALL kernel-region indices regardless of the present bit — a non-present kernel L4 entry is still treated as a valid resolved mapping
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional x86-64 page table design. Kernel-region L4 entries (indices < `kernel_l4_end`) contain shared kernel mappings that must always be traversable regardless of the present bit. The spec deliberately returns `Some` for `l4i < kernel_l4_end` to ensure kernel address resolution always succeeds.

