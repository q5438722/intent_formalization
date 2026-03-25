# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__get_entry_1g_l3.rs`
**Date:** 2026-03-24T07:15:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. `disjoint_l4_no_cross_level_check` has a tautological ensures clause (P ∨ ¬P) that proves nothing. `l4_dom_singleton_cr3` is a correct and intentional consequence of the spec design — each PageTable struct models one address space with one L4 table rooted at cr3; multi-address-space support comes from having multiple PageTable instances rather than multiple L4 tables within one.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: disjoint_l4_no_cross_level_check
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** disjoint_l4 only checks L4 entries don't alias each other, but never checks that L4 target addresses are disjoint from L2/L1 table addresses — cross-level aliasing is unconstrained
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `a == p || a != p` is a tautology that holds for any values. While the observation about missing cross-level disjointness is interesting, this PHI proves nothing about the spec.

### φ3: l4_dom_singleton_cr3
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** wf_l4 constrains l4_table domain to exactly {cr3} — only a single L4 table exists, meaning PCID-based multi-address-space sharing at the L4 level is impossible under this spec
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from `wf_l4` which explicitly constrains `self.l4_table@.dom() =~= Set::<PageMapPtr>::empty().insert(self.cr3)`. A single L4 table per PageTable struct is intentional — each process/address space has its own PageTable instance with its own cr3. Multiple address spaces are modeled as multiple PageTable instances, not multiple L4 entries within one.

### φ4: tlb_submap_no_staleness_check
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** tlb_submap_of_mapping ensures TLB entries are always consistent submaps of current mappings — but real TLBs can hold stale entries after mapping changes until explicit flush, so this overly strong invariant may mask TLB shootdown bugs

### φ5: zero_entry_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Relies on two independent external_body claims (usize2page_entry special-casing v==0 and usize2pa producing addr==0 for v==0) agreeing — if either is wrong the combined property is unsound

