# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__remove_l2_entry.rs`
**Date:** 2026-03-24T07:25:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives identifying the same class of spec gap: missing cross-level disjointness constraints between page table level domains. The spec defines `wf_l4`, `wf_l3`, `wf_l2`, and `wf_l1` independently, and while within-level disjointness is partially addressed (via `disjoint_l4`, `disjoint_l3`, `disjoint_l2`), no predicate prevents the same physical page pointer from appearing in multiple level domains simultaneously. This extends to all level pairs (L4/L3, L3/L2, L2/L1), though only L4/L3 and L3/L2 were tested here.

## True Positives (Spec Issues)

### page_closure_l4_l3_can_overlap
- **Confidence:** medium
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (constrained to `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously, with no cross-level disjointness predicate. This means the same physical page could be interpreted as both an L4 and L3 page table — a real structural soundness gap that could mask aliasing bugs in page table management.

### page_closure_l3_l2_can_overlap
- **Confidence:** medium
- **Reasoning:** Similarly, no predicate prevents the same `PageMapPtr` from appearing in both `l3_tables@.dom()` and `l2_tables@.dom()`. The `wf_l3` and `wf_l2` predicates independently constrain their respective domains but never assert mutual exclusion. Hardware would interpret the same physical page as two different levels of page table entries simultaneously, which is a real spec gap.

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
- **Why flagged:** No cross-level disjointness constraint — the same physical page can appear in both l4_table and l3_tables domains simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The spec allows `cr3` to appear in both `l4_table@.dom()` (constrained to `{cr3}` by `wf_l4`) and `l3_tables@.dom()` simultaneously, with no cross-level disjointness predicate. This means the same physical page could be interpreted as both an L4 and L3 page table — a real structural soundness gap that could mask aliasing bugs in page table management.

### φ4: page_entry_to_map_entry_drops_present_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapEntry does not preserve the present bit — non-present entries produce the same MapEntry as present entries, losing validity information

### φ5: page_closure_l3_l2_can_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No cross-level disjointness between L3 and L2 table domains — the same physical page could serve as both an L3 and L2 page table simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Similarly, no predicate prevents the same `PageMapPtr` from appearing in both `l3_tables@.dom()` and `l2_tables@.dom()`. The `wf_l3` and `wf_l2` predicates independently constrain their respective domains but never assert mutual exclusion. Hardware would interpret the same physical page as two different levels of page table entries simultaneously, which is a real spec gap.

