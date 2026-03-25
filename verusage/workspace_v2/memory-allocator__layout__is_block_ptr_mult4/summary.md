# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__is_block_ptr_mult4.rs`
**Date:** 2026-03-24T11:29:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives identify external_body trust gaps: `block_ptr_aligned_to_word` trusts the alignment property without proof, `size_of_node` trusts the layout of `Node`, and their combination enables the mod-8 alignment guarantee that is stronger than what the verified `is_block_ptr_mult4` proves (mod-4). Two false positives: the tautological `is_block_ptr1` reflexivity proves nothing, and the shared-provenance property is a correct consequence of `is_block_ptr`'s definition.

## True Positives (Spec Issues)

### block_ptr_aligned_external_body
- **Confidence:** medium
- **Reasoning:** `block_ptr_aligned_to_word` is `external_body` trusting that all block pointers are aligned to `align_of::<Node>()` without proof. This alignment property is critical for safe pointer casting and depends on unverified assumptions about the allocator's layout.

### size_of_node_external_body
- **Confidence:** medium
- **Reasoning:** `size_of_node` is `external_body` trusting that `Node` has size 8 and alignment 8. While correct for the current single-field struct on 64-bit, this is unverified and would silently break if the struct definition changes.

### is_block_ptr_mult8
- **Confidence:** medium
- **Reasoning:** The 8-byte alignment of block pointers depends on two chained `external_body` lemmas: `size_of_node` (asserting `align_of::<Node>() == 8`) and `block_ptr_aligned_to_word` (asserting alignment to `align_of::<Node>()`). Both are trusted without proof, and the existing `is_block_ptr_mult4` only proves mod-4 alignment in the source, suggesting the stronger mod-8 property relies entirely on unverified assumptions.

## All Candidates

### φ1: is_block_ptr1_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_block_ptr1` is `external_body` with uninterpreted semantics — it could be trivially true or false for any input, making `is_block_ptr` depend on a completely unconstrained predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures is a tautology (`x == x`). While `is_block_ptr1` being `external_body` is noteworthy, this φ doesn't demonstrate any actual spec gap — it proves nothing beyond reflexivity.

### φ2: block_ptr_aligned_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `block_ptr_aligned_to_word` is `external_body` — the alignment guarantee for block pointers is entirely trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `block_ptr_aligned_to_word` is `external_body` trusting that all block pointers are aligned to `align_of::<Node>()` without proof. This alignment property is critical for safe pointer casting and depends on unverified assumptions about the allocator's layout.

### φ3: size_of_node_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `size_of_node` is `external_body` — the size and alignment of `Node` are trusted assertions that would silently become wrong if the struct layout changes
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `size_of_node` is `external_body` trusting that `Node` has size 8 and alignment 8. While correct for the current single-field struct on 64-bit, this is unverified and would silently break if the struct definition changes.

### φ4: is_block_ptr_any_provenance
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two `BlockId`s satisfying `is_block_ptr` for the same pointer must share provenance — but `is_block_ptr1` being unconstrained means nothing prevents contradictory `BlockId` metadata from satisfying the predicate simultaneously
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from `is_block_ptr`'s definition: both require `ptr@.provenance == block_id.page_id.segment_id.provenance`, so if both hold for the same `ptr`, the provenances must be equal. This is correct and expected — not a spec gap.

### φ5: is_block_ptr_mult8
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Block pointers are 8-byte aligned via two chained `external_body` lemmas (`size_of_node` + `block_ptr_aligned_to_word`) — the stronger alignment (mod 8 vs mod 4 in `is_block_ptr_mult4`) relies entirely on unverified trust assumptions
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The 8-byte alignment of block pointers depends on two chained `external_body` lemmas: `size_of_node` (asserting `align_of::<Node>() == 8`) and `block_ptr_aligned_to_word` (asserting alignment to `align_of::<Node>()`). Both are trusted without proof, and the existing `is_block_ptr_mult4` only proves mod-4 alignment in the source, suggesting the stronger mod-8 property relies entirely on unverified assumptions.

