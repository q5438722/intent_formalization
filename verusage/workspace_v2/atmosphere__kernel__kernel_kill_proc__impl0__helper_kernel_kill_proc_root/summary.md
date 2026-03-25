# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/kernel/kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_root.rs`
**Date:** 2026-03-24T06:48:08Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One of three candidates is a true positive. The `container_map_4k_allows_double_ownership` PHI correctly identifies that the Ghost field `container_map_4k` lacks any disjointness predicate, allowing two distinct containers to claim the same physical page — a real spec gap for a page allocator. The other two candidates (`mapped_pages_not_subset_of_allocated` and `free_pages_overlap_allocated`) are false positives: both have ensures clauses that trivially follow from their preconditions without revealing any actual spec weakness. Their underlying observations about missing constraints may be valid, but the PHI formulations fail to meaningfully demonstrate them.

## True Positives (Spec Issues)

### container_map_4k_allows_double_ownership
- **Confidence:** medium
- **Reasoning:** The ensures clause is not a tautology — it requires both containment predicates to hold simultaneously under the assumption that both are in the preconditions, which Verus confirms is satisfiable. The real issue is that `container_map_4k` is a Ghost field with no disjointness constraint in the visible spec, meaning two distinct containers can claim the same page. A well-formed allocator should enforce exclusive page ownership across containers.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — any arbitrary usize is treated as having a valid physical address after masking, trusted without proof

### φ2: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body ensures l == self.value_list_len unconditionally without requiring wf() — corrupted linked list returns raw field as trusted length

### φ3: container_map_4k_allows_double_ownership
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k is a Ghost field with no disjointness predicate — two different containers can simultaneously claim ownership of the same physical page without contradiction
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The ensures clause is not a tautology — it requires both containment predicates to hold simultaneously under the assumption that both are in the preconditions, which Verus confirms is satisfiable. The real issue is that `container_map_4k` is a Ghost field with no disjointness constraint in the visible spec, meaning two distinct containers can claim the same page. A well-formed allocator should enforce exclusive page ownership across containers.

### φ4: mapped_pages_not_subset_of_allocated
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** mapped_pages_4k has no visible constraint requiring it to be a subset of allocated_pages_4k — a page can be in the mapped set without being allocated, since both are independent closed external_body specs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause trivially restates the precondition (`mapped_pages_4k().contains(p)` implies itself). This proves nothing about the relationship between mapped and allocated pages. While the observation about missing subset constraints may be valid, this PHI doesn't demonstrate it.

### φ5: free_pages_overlap_allocated
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** free_pages_4k and allocated_pages_4k are both closed external_body specs with no visible disjointness constraint — a page can be simultaneously free and allocated without contradiction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause merely conjoins the two preconditions, which is trivially true given the requires clause. This doesn't demonstrate that the spec *allows* overlap — it only shows that assuming overlap doesn't lead to contradiction, which is vacuously true for any pair of independent predicates. A meaningful test would derive `false` or an absurd consequence from the overlap assumption, or show overlap is constructible from wf().

