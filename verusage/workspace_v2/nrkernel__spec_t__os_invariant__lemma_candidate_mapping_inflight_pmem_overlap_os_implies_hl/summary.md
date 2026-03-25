# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_candidate_mapping_inflight_pmem_overlap_os_implies_hl.rs`
**Date:** 2026-03-24T14:41:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: zero-size regions at the same base are considered overlapping due to the `region1.base == region2.base` equality check in `overlap`, which doesn't account for empty regions. Two false positives: self-overlap of non-empty regions is correct by construction, and the strict vs non-strict bound inconsistency between virtual and physical mapping checks is a direct restatement of the definitions reflecting intentional design choices.

## True Positives (Spec Issues)

### overlap_same_base_zero_size
- **Confidence:** medium
- **Reasoning:** Two zero-size regions at the same base overlap because `overlap` checks `region1.base == region2.base` before checking sizes. This means empty/degenerate regions at the same address are considered conflicting, which could unnecessarily block valid mappings with zero-size PTEs or cause spurious overlap detection.

## All Candidates

### φ1: overlap_reflexive_nonzero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any non-empty region should overlap with itself — tests that the `region1.base == region2.base` equality branch in `overlap` fires correctly for self-overlap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When `r.size > 0`, `overlap(r, r)` enters the `region1.base <= region2.base` branch (since `r.base <= r.base`), and the `region1.base == region2.base` check is trivially true. Self-overlap of non-empty regions is correct and expected.

### φ2: overlap_same_base_zero_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping due to the `region1.base == region2.base` check — this means empty regions at the same address block each other, which may be an unintended spec weakness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Two zero-size regions at the same base overlap because `overlap` checks `region1.base == region2.base` before checking sizes. This means empty/degenerate regions at the same address are considered conflicting, which could unnecessarily block valid mappings with zero-size PTEs or cause spurious overlap detection.

### φ3: is_in_crit_sect_map_executing
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `MapExecuting` is both in critical section and a map state — tests that the two predicates agree on intermediate map states; if `is_in_crit_sect` missed `MapExecuting`, concurrent operations could proceed unsafely

### φ4: candidate_mapping_in_bounds_pmem_strict_vs_nonstrict
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `candidate_mapping_in_bounds_pmem` uses `<=` (non-strict) while `candidate_mapping_in_bounds` uses `<` (strict) — the inconsistency means a physical mapping can touch the boundary of `range_mem` but a virtual mapping cannot touch `upper_vaddr`, which may cause edge-case mismatches
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct restatement of the `candidate_mapping_in_bounds_pmem` definition. The inconsistency between `<` (virtual) and `<=` (physical) is an intentional design choice — physical memory ranges are typically closed at the upper bound while virtual address spaces use exclusive upper bounds. This is not a spec gap.

### φ5: unmap_waiting_not_in_crit_sect
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `UnmapWaiting` is explicitly excluded from `is_in_crit_sect` — tests that waiting-to-unmap cores aren't considered in critical section; if wrong, the OS could deadlock by blocking new operations while waiting to start an unmap

