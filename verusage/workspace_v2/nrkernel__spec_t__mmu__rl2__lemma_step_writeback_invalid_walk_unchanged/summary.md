# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_writeback_invalid_walk_unchanged.rs`
**Date:** 2026-03-24T13:59:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: the `MAX_PHYADDR_WIDTH` gap between the external_body value (52) and the axiom constraint (32..=52) leaves all dependent address masks underspecified. One false positive: equal-input-implies-equal-output is trivially true for any spec function.

## True Positives (Spec Issues)

### max_phyaddr_width_gap
- **Confidence:** high
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with runtime value 52, but the axiom only constrains it to `32..=52`. All address masks (`MASK_ADDR`, `MASK_L1_PG_ADDR`, etc.) depend on this value, so Verus cannot pin down the actual hardware physical address width, making all dependent verification unsound for any specific assumption.

## All Candidates

### φ1: nonneg_nonpos_write_disjoint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A write cannot be both nonneg (P: 0→1) and nonpos (P: 1→0) — tests mutual exclusion of the two write classifications

### φ2: write_seq_single_equals_write
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `write_seq` of a single-element sequence should equal a single write — tests that fold_left correctly applies the single step

### φ3: max_phyaddr_width_gap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52 but axiom only constrains to 32..=52 — the verification accepts any width in the range
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with runtime value 52, but the axiom only constrains it to `32..=52`. All address masks (`MASK_ADDR`, `MASK_L1_PG_ADDR`, etc.) depend on this value, so Verus cannot pin down the actual hardware physical address width, making all dependent verification unsound for any specific assumption.

### φ4: pt_walk_l0_invalid_single_path
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When the L0 entry is non-present, the walk should terminate immediately — tests early termination for non-present root entries

### φ5: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — the PDE validity predicate is entirely opaque and could behave inconsistently
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `e1 == e2` means the inputs are identical, so any spec function (including external_body) must return the same result — spec functions are deterministic by construction in Verus. This proves nothing about the external_body gap.

