# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__map_4k_page.rs`
**Date:** 2026-03-24T07:22:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive was found: the lack of cross-level disjointness between page table level domains (L4/L3/L2/L1) allows the same physical page pointer to appear at multiple levels simultaneously, which could lead to aliasing where a single page serves as both an L4 and L3 table. The other two candidates are false positives — both use tautological ensures clauses (P ∨ ¬P) that prove nothing about the spec, regardless of whether the underlying observations about L1 user-bit constraints or kernel-region constraints have merit.

## True Positives (Spec Issues)

### page_closure_no_cross_level_disjointness
- **Confidence:** medium
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` and `l3_tables@.dom()` simultaneously — `wf_l4` constrains `l4_table@.dom()` to `{cr3}` and `wf_l3` has no exclusion of `cr3` from `l3_tables@.dom()`. This means the same physical page could serve as both L4 and L3 table, which is a real cross-level disjointness gap. The `page_closure` set union would count this page once (sets are idempotent), but the aliasing itself is a soundness concern.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: page_entry_to_map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry contains only addr/write/execute_disable — the user/supervisor bit is silently dropped, making kernel-only and user-accessible pages indistinguishable after conversion

### φ3: page_closure_no_cross_level_disjointness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No cross-level disjointness constraint — the same physical page pointer can appear in both l4_table and l3_tables domains simultaneously, meaning page_closure double-counts it
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` and `l3_tables@.dom()` simultaneously — `wf_l4` constrains `l4_table@.dom()` to `{cr3}` and `wf_l3` has no exclusion of `cr3` from `l3_tables@.dom()`. This means the same physical page could serve as both L4 and L3 table, which is a real cross-level disjointness gap. The `page_closure` set union would count this page once (sets are idempotent), but the aliasing itself is a soundness concern.

### φ4: user_only_no_l1_constraint
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** user_only constrains L4/L3/L2 entries to have user bit set when present, but if L1 entries are not similarly constrained, a kernel-only 4K page could exist under an otherwise user-accessible subtree
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `p.user || !p.user` is a tautology that holds for any boolean value. This PHI proves nothing about the spec — it would verify regardless of whether `user_only` constrains L1 entries or not. The observation about a potential L1 gap may be interesting, but this PHI does not demonstrate it.

### φ5: wf_l4_kernel_region_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** wf_l4 only constrains entries at indices >= kernel_l4_end — kernel-region L4 entries (below kernel_l4_end) can have arbitrary permission bits and point to arbitrary addresses with no well-formedness checks
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `ps || !ps` is a tautology. Moreover, kernel-region L4 entries being unconstrained by `wf_l4` is intentional — the kernel region is shared across address spaces and managed separately. The spec deliberately only constrains user-space entries (indices >= `kernel_l4_end`).

