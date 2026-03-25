# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_pt_walk_result_vaddr_indexing_bits_match.rs`
**Date:** 2026-03-24T13:51:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: the external_body `lemma_bits_align_to_usize` trusts bit-alignment preservation without proof, and the `indexing_bits_match` walk result property depends transitively on this unverified lemma. Two false positives: the `all_mb0_bits_are_zero` φ is a trivial tautology (`|| true`), and L3 page size equaling 4KB is correct by construction.

## True Positives (Spec Issues)

### lemma_bits_align_external_body_l3
- **Confidence:** high
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties including L3 (4KB) alignment are trusted without proof. This is a recurring unverified trust assumption in the MMU spec.

### indexing_bits_match_walk_result
- **Confidence:** medium
- **Reasoning:** `lemma_pt_walk_result_vaddr_indexing_bits_match` depends on the external_body `lemma_bits_align_to_usize` via `broadcast use`. The indexing bits match property for walk results inherits this unverified trust assumption.

## All Candidates

### φ1: lemma_bits_align_external_body_l3
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bits_align_to_usize` is `external_body` — L3 (4KB page) alignment preserving all 4 indexing bit fields is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties including L3 (4KB) alignment are trusted without proof. This is a recurring unverified trust assumption in the MMU spec.

### φ2: indexing_bits_match_walk_result
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Walk result vaddr preserving indexing bits depends transitively on the unverified `lemma_bits_align_to_usize` — if false, the walk result vaddr would not index to the same page table entries as the original vaddr
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_pt_walk_result_vaddr_indexing_bits_match` depends on the external_body `lemma_bits_align_to_usize` via `broadcast use`. The indexing bits match property for walk results inherits this unverified trust assumption.

### φ3: pt_walk_nonpresent_l0_single_entry
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When the L0 PDE has P=0, it's Invalid and the walk terminates with 1 entry — tests that non-present root entries produce immediate termination

### φ4: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — the predicate that determines PDE validity is opaque; different layers have different must-be-zero bits but the predicate could conflate them
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This φ is `P || !Q || true`, which is trivially true regardless of the values of P and Q. It demonstrates nothing about the external_body predicate — the `|| true` makes the entire ensures clause a tautology.

### φ5: pt_walk_l3_page_size_equals_page_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A 4-entry walk ending in Page produces a Valid result with L3_ENTRY_SIZE (4KB) — tests that the deepest walk level correctly maps to 4KB pages
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A 4-entry walk with a Page last entry directly matches the `path.len() == 4` branch in `Walk::result`, which uses `L3_ENTRY_SIZE` for the size. This is correct by construction — L3 is the leaf level with 4KB pages on x86-64.

