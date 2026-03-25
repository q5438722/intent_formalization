# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__impl__create_intersect.rs`
**Date:** 2026-03-24T11:03:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four candidates are true positives identifying real soundness gaps in external_body axioms. Three stem from missing `j < 64` bounds in `lemma_is_bit_set` (the `!0usize` clause, OR biconditional, and AND biconditional all lack guards for out-of-range bit positions where shift wrapping invalidates the intended semantics). The fourth is `lemma_view`'s first ensures clause which doesn't bound `a` to `[0,8)`, allowing spec-level out-of-bounds array access for elements outside the valid range.

## True Positives (Spec Issues)

### is_bit_set_not0_no_bound
- **Confidence:** high
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps, so the axiom asserts properties about non-existent bit positions that could enable unsound reasoning.

### or_bit_set_no_bound
- **Confidence:** high
- **Reasoning:** The OR biconditional axiom lacks a `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, and the biconditional relates three `is_bit_set` calls at wrapped positions where the logical meaning breaks down.

### and_bit_set_no_bound
- **Confidence:** high
- **Reasoning:** The AND biconditional has no `j < 64` guard. For out-of-range bit positions, `1usize << j` wraps and the axiom asserts equivalences about wrapped bit operations that don't correspond to actual bit semantics.

### lemma_view_no_range_guard
- **Confidence:** high
- **Reasoning:** The first ensures of `lemma_view` decomposes any `i` in `self@` into `a = i/64` and `b = i%64` then accesses `self.mask[a]` without guarding `0 <= a < 8`. The `view()` definition constrains elements to valid ranges, but the external_body lemma doesn't faithfully reflect this bound, allowing spec-level out-of-bounds access for out-of-range `i`.

## All Candidates

### φ1: is_bit_set_not0_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_bit_set(!0usize, j)` is axiomatized for ALL `j: usize` with no `j < 64` guard — for `j >= 64`, `1usize << j` wraps, so the axiom asserts properties about out-of-range bit positions
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps, so the axiom asserts properties about non-existent bit positions that could enable unsound reasoning.

### φ2: or_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The OR biconditional has no `j < 64` guard — for out-of-range positions where `1usize << j` wraps, this axiom produces conclusions about wrapped bit operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The OR biconditional axiom lacks a `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, and the biconditional relates three `is_bit_set` calls at wrapped positions where the logical meaning breaks down.

### φ3: and_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The AND biconditional has no `j < 64` guard — for out-of-range bit positions, `1usize << j` wraps and the axiom asserts equivalences that don't hold for real bit semantics
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The AND biconditional has no `j < 64` guard. For out-of-range bit positions, `1usize << j` wraps and the axiom asserts equivalences about wrapped bit operations that don't correspond to actual bit semantics.

### φ4: lemma_view_no_range_guard
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view`'s first ensures decomposes any `i` in `self@` without guarding `0 <= a < 8` — for `i >= 512` or `i < 0`, `a` falls outside `[0,8)` making `self.mask[a]` an out-of-bounds spec access
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The first ensures of `lemma_view` decomposes any `i` in `self@` into `a = i/64` and `b = i%64` then accesses `self.mask[a]` without guarding `0 <= a < 8`. The `view()` definition constrains elements to valid ranges, but the external_body lemma doesn't faithfully reflect this bound, allowing spec-level out-of-bounds access for out-of-range `i`.

### φ5: not0_and_zero_inconsistency
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Combining unbounded `is_bit_set(!0, j)` with unbounded AND biconditional yields `is_bit_set(0, j)` for `j >= 64` — but `is_bit_set(0, j)` should be false, exposing a potential inconsistency in the axiom system at out-of-range positions

