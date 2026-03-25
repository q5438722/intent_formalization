# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal_aux2.rs`
**Date:** 2026-03-24T13:50:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: the external_body `lemma_bits_align_to_usize` trusts 13+ bit-alignment preservation properties without proof, and the walk vaddr idempotency depends transitively on this unverified lemma. One false positive: the `all_mb0_bits_are_zero` equality-on-equal-inputs is a trivial tautology for spec functions.

## True Positives (Spec Issues)

### lemma_bits_align_external_body_l2
- **Confidence:** high
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties are trusted without proof. These properties are mathematically correct (L2 alignment zeroes bits 0-20, preserving bits 21+), but the proof is missing.

### iter_walk_vaddr_idempotent_via_external
- **Confidence:** medium
- **Reasoning:** The walk vaddr idempotency proof (`lemma_iter_walk_result_vbase_equal_aux2`) depends transitively on the external_body `lemma_bits_align_to_usize`. The entire proof chain for walk result stability inherits this unverified trust assumption.

## All Candidates

### φ1: lemma_bits_align_external_body_l2
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bits_align_to_usize` is `external_body` — L2 alignment preserving l0/l1/l2 bits is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties are trusted without proof. These properties are mathematically correct (L2 alignment zeroes bits 0-20, preserving bits 21+), but the proof is missing.

### φ2: iter_walk_vaddr_idempotent_via_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Walk vaddr idempotency depends on unverified `lemma_bits_align_to_usize` — if alignment doesn't preserve indexing bits, re-walking from the result vaddr could diverge
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The walk vaddr idempotency proof (`lemma_iter_walk_result_vbase_equal_aux2`) depends transitively on the external_body `lemma_bits_align_to_usize`. The entire proof chain for walk result stability inherits this unverified trust assumption.

### φ3: walk_next_entry_from_arbitrary_mem
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the L0 address is not in `mem.mem`'s domain, `PTMem::read` returns an arbitrary value via `Map::index` on a missing key — the walk proceeds with an unspecified PDE entry

### φ4: walk_result_directory_last_arbitrary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A 4-entry walk ending in Directory (complete but not Page or Invalid) falls through both `if` branches to `arbitrary()` — the result is completely undefined

### φ5: all_mb0_bits_are_zero_external_body_opaque
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — PDE validity is governed by an opaque predicate with no specification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This φ only proves that equal entries produce equal results (`e1 == e2 ==> f(e1) == f(e2)`), which is trivially true for any spec function in Verus — spec functions are deterministic by construction. It demonstrates nothing about the external_body gap.

