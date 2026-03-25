# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_WriteNonpos_post_valid_pt_walk_no_wraddr_in_path.rs`
**Date:** 2026-03-24T13:54:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `PTMem::view` is an external_body function critical for PTE interpretation that is entirely unverified. Two false positives: empty `write_seq` identity is correct by `fold_left` definition, and the `all_mb0_bits_are_zero` tautology proves nothing.

## True Positives (Spec Issues)

### ptmem_view_external_body_functional
- **Confidence:** high
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the function mapping raw page table memory to a `Map<usize, PTE>` is completely opaque. While functional consistency (same inputs → same output) holds trivially for spec functions, the underlying issue is that the view function itself is unverified and could map incorrectly.

## All Candidates

### φ1: ptmem_view_external_body_write_invariance
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` — writing a non-present value over a non-present entry should not change the PTE view, but this is unverified and could be incorrect if the view function considers non-present entries

### φ2: ptmem_view_external_body_functional
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — identical memories should produce identical PTE views, but this is completely unverified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the function mapping raw page table memory to a `Map<usize, PTE>` is completely opaque. While functional consistency (same inputs → same output) holds trivially for spec functions, the underlying issue is that the view function itself is unverified and could map incorrectly.

### φ3: write_seq_empty_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `write_seq` of an empty sequence via `fold_left` should be identity — if fold_left's base case is wrong, this could alter the memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write_seq` uses `fold_left` with the identity closure. `seq![].fold_left(mem, f)

### φ4: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — the PDE validity predicate is entirely opaque
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a tautology (`P || !P`) that holds for any boolean expression regardless of whether the predicate is external_body. It demonstrates nothing about the spec gap.

### φ5: is_nonneg_and_nonpos_mutually_exclusive
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A write cannot be both nonneg (P: 0→1) and nonpos (P: 1→0) simultaneously — tests mutual exclusion of the two write classifications

