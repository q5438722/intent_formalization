# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal_aux1.rs`
**Date:** 2026-03-24T13:48:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives: the external_body `lemma_bits_align_to_usize` trusts 13 bit-alignment properties without proof, the walk path idempotency depends transitively on this unverified lemma, and `Walk::result` returns `arbitrary()` for path.len()==1 Page entries leaving L0 page walks undefined. One false positive: the `all_mb0_bits_are_zero` tautology proves nothing about the external_body predicate.

## True Positives (Spec Issues)

### lemma_bits_align_external_body
- **Confidence:** high
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all 13 bit-alignment preservation postconditions are trusted without proof. These properties are critical for walk idempotency and could be incorrect if the bitmask/alignment interaction has edge cases. Note: in a separate file this same lemma was verified with `by (bit_vector)` proofs, but in this version it's external_body.

### iter_walk_path_idempotent_via_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux1` is verified but depends entirely on the external_body `lemma_bits_align_to_usize`. The walk path idempotency guarantee is only as strong as the unverified bit-alignment lemma it relies on, making the entire proof chain a trust assumption.

### walk_result_path_len1_page_arbitrary
- **Confidence:** medium
- **Reasoning:** `Walk::result` falls through to `arbitrary()` for path.len() == 1 with a Page entry. Since `arbitrary()` can return any value including `Valid` with garbage fields, this means a single-step L0 page walk has completely undefined results. While x86-64 doesn't support L0 huge pages, the spec doesn't prevent this case from being constructed.

## All Candidates

### φ1: lemma_bits_align_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bits_align_to_usize` is `external_body` — all 13 bit-alignment preservation properties are trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all 13 bit-alignment preservation postconditions are trusted without proof. These properties are critical for walk idempotency and could be incorrect if the bitmask/alignment interaction has edge cases. Note: in a separate file this same lemma was verified with `by (bit_vector)` proofs, but in this version it's external_body.

### φ2: iter_walk_path_idempotent_via_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_iter_walk_result_vbase_equal_aux1` depends on the external_body `lemma_bits_align_to_usize` — the walk idempotency proof chain is only as strong as the unverified bit-alignment lemma
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux1` is verified but depends entirely on the external_body `lemma_bits_align_to_usize`. The walk path idempotency guarantee is only as strong as the unverified bit-alignment lemma it relies on, making the entire proof chain a trust assumption.

### φ3: walk_next_l0_reads_pml4_offset
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The first walk step reads from pml4 + l0_bits * 8 — tests that the L0 entry address computation is correct and the walk always produces a 1-entry path after the first step

### φ4: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — the predicate governing PDE validity is entirely opaque with no specification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This φ is a tautology (`P || !P`) that holds for any boolean expression regardless of whether `all_mb0_bits_are_zero` is external_body. It demonstrates nothing about the predicate's actual behavior or any spec gap.

### φ5: walk_result_path_len1_page_arbitrary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Walk::result` handles path lengths 2, 3, 4 for Page entries but falls through to `arbitrary()` for path.len() == 1 — a single-entry Page walk has undefined result
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Walk::result` falls through to `arbitrary()` for path.len() == 1 with a Page entry. Since `arbitrary()` can return any value including `Valid` with garbage fields, this means a single-step L0 page walk has completely undefined results. While x86-64 doesn't support L0 huge pages, the spec doesn't prevent this case from being constructed.

