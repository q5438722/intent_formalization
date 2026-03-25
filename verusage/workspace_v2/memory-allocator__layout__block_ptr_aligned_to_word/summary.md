# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__block_ptr_aligned_to_word.rs`
**Date:** 2026-03-24T11:17:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives identify external_body trust gaps: `const_facts` trusts constant equalities that could be verified by `compute`, `mod_mul` trusts a provable arithmetic property, and `size_of_node` trusts layout facts about the `Node` struct. One false positive: `block_start_no_overflow_check` is a correct computation result, and the overflow concern is already handled by `is_block_ptr1`'s range constraints.

## True Positives (Spec Issues)

### const_facts_external_body
- **Confidence:** medium
- **Reasoning:** `const_facts` is `external_body` trusting 11 constant equalities without proof. These could be verified by `compute` (as the non-external version in the other file does), so using `external_body` is an unnecessary trust gap.

### mod_mul_external_body
- **Confidence:** medium
- **Reasoning:** `mod_mul` is `external_body` trusting the property `b % c == 0 ==> (a*b) % c == 0` without proof. While mathematically correct, this is a fundamental arithmetic lemma that should be provable via nonlinear_arith, and `block_ptr_aligned_to_word` critically depends on it.

### size_of_node_external_body
- **Confidence:** medium
- **Reasoning:** `size_of_node` is `external_body` trusting that `Node` (containing a single `*mut Node`) has size 8 and alignment 8. While correct for the current struct definition on 64-bit, this is an unverified assumption that would silently break if the struct changes.

## All Candidates

### φ1: const_facts_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `const_facts` is `external_body` — all constant equalities are trusted without proof, so if any constant definition were wrong, the axiom would hide the inconsistency
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `const_facts` is `external_body` trusting 11 constant equalities without proof. These could be verified by `compute` (as the non-external version in the other file does), so using `external_body` is an unnecessary trust gap.

### φ2: block_ptr_in_single_slice
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `is_block_ptr1` constrains blocks to fit within a single SLICE_SIZE region — but blocks near the end of a slice with `block_size > 1` could extend past the slice boundary, and the spec only checks the start address

### φ3: mod_mul_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mod_mul` is `external_body` — the modular arithmetic property `b % c == 0 ==> (a*b) % c == 0` is trusted without proof, and `block_ptr_aligned_to_word` depends on it for alignment correctness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mod_mul` is `external_body` trusting the property `b % c == 0 ==> (a*b) % c == 0` without proof. While mathematically correct, this is a fundamental arithmetic lemma that should be provable via nonlinear_arith, and `block_ptr_aligned_to_word` critically depends on it.

### φ4: block_start_no_overflow_check
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `block_start` computes `page_start + start_offset + block_idx * block_size` with no overflow guard — for large `block_idx` or `block_size`, the sum could exceed the segment boundary without being caught by the spec
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This just confirms that `block_start` at segment 0, page 0, block 0 equals `start_offset(8)` — which is the correct and expected computation. The overflow concern is addressed by `is_block_ptr1` which constrains `ptr <= segment_start + SEGMENT_SIZE < usize::MAX`.

### φ5: size_of_node_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `size_of_node` is `external_body` — the size and alignment of `Node` (a struct containing `*mut Node`) are trusted assertions; if the struct layout changes (e.g., adding fields), these would silently become wrong
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `size_of_node` is `external_body` trusting that `Node` (containing a single `*mut Node`) has size 8 and alignment 8. While correct for the current struct definition on 64-bit, this is an unverified assumption that would silently break if the struct changes.

