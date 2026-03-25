# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__impl__calculate_page_block_at.rs`
**Date:** 2026-03-24T11:28:27Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `const_facts` is `external_body` trusting constant equalities that could be verified by `compute`. Four false positives test properties of pure arithmetic helper functions (`block_start_at`, `page_start`) that are intentionally general — bounds enforcement and degenerate case exclusion are delegated to higher-level callers and runtime type constraints.

## True Positives (Spec Issues)

### const_facts_external_body
- **Confidence:** medium
- **Reasoning:** `const_facts` is `external_body` trusting 11 constant equalities without proof. These could be verified by `compute` assertions, so using `external_body` is an unnecessary trust gap.

## All Candidates

### φ1: const_facts_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `const_facts` is `external_body` — all constant equalities are trusted without proof, though they could be verified by `compute`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `const_facts` is `external_body` trusting 11 constant equalities without proof. These could be verified by `compute` assertions, so using `external_body` is an unnecessary trust gap.

### φ2: calculate_no_block_size_lower_bound
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `calculate_page_block_at` has no precondition requiring `block_size > 0` — with `block_size == 0`, all block indices map to the same address, and arithmetic `page_start + 0 * idx == page_start` allows this degenerate case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `block_start_at` is a pure arithmetic helper — `0 * 5 == 0 * 0` is trivially true. The degenerate `block_size == 0` case is excluded at higher levels (e.g., `is_block_ptr1` requires `block_size >= size_of::<Node>()`). A helper function not enforcing this is normal design.

### φ3: page_idx_unbounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `page_id.idx` has no upper bound — idx=1000 exceeds `SLICES_PER_SEGMENT=512`, placing the page start beyond the segment; `calculate_page_block_at` doesn't constrain this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `page_start` is a pure address computation function that correctly computes `segment_start + SLICE_SIZE * idx` for any `idx`. Bounds are enforced by callers — `calculate_page_block_at` constrains the result to `<= segment_start + SEGMENT_SIZE`, which indirectly limits meaningful `page_id.idx` values.

### φ4: block_start_at_negative_idx
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `block_start_at` accepts negative `block_idx` — computing an address before the page start; `calculate_page_block_at` uses `usize` for idx preventing this at runtime, but the spec function allows it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `block_start_at` operates on mathematical `int`, where negative indices are well-defined arithmetic. The runtime function `calculate_page_block_at` uses `usize` for `idx`, preventing negative values in practice. A spec-level helper being more general than its runtime callers is standard design.

### φ5: calculate_overflow_not_checked
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The result of `calculate_page_block_at` is bounded by `segment_start + SEGMENT_SIZE < usize::MAX` — but there's no check that `page_start + block_size * idx` doesn't overflow `usize` in the intermediate computation; the precondition ensures the mathematical result fits, relying on Verus's overflow checking
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures just restates that the block address is within the segment, which follows directly from the precondition. Verus automatically checks `usize` overflow in the exec code (`page_start + block_size * idx`), and the precondition ensures the result fits within `usize::MAX`. There's no gap here.

