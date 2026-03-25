# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_Writeback_post_valid_pt_walk_not_wraddr_in_path/original.rs`
**Date:** 2026-03-24T22:19:26Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `align_identity_when_unaligned` is a false positive. It targets `align_to_usize`, which is a `spec(checked)` function—not an executable function—so it falls outside the scope of exec-spec consistency analysis. Moreover, the asserted behavior (identity when unaligned) flatly contradicts the open definition `sub(a, a % b)`, meaning the property is either vacuously true due to type-level unsatisfiability of its preconditions or simply would not verify, making it uninformative about any real spec weakness.

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Downward alignment must never produce a value exceeding the original input

### φ2: align_identity_when_unaligned → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When the input is not already aligned, align_to_usize must actually change the value
- **Verdict:** FALSE_POSITIVE (high)

### φ3: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce a b-aligned result; failing that is a spec bug

### φ4: aligned_implies_zero → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Many non-zero addresses are validly aligned; collapsing aligned to only zero would break all page-table reasoning

### φ5: align_always_zero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For inputs at least as large as the alignment, the result must be nonzero; always-zero would be a catastrophic spec error

