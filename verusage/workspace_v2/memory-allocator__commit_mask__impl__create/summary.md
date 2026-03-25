# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__impl__create.rs`
**Date:** 2026-03-24T11:00:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives identify real soundness gaps in the external_body axioms: two missing `j < 64` guards in `lemma_is_bit_set` (the `!0usize` clause and the OR biconditional) and a missing `0 <= a < 8` guard in `lemma_view`'s decomposition ensures. These allow reasoning about out-of-range bit positions and out-of-bounds array accesses. The `lemma_change_one_entry` candidate is a false positive — it correctly derives the abstract view from the concrete mask value, which is the intended design pattern where the caller controls what value gets stored.

## True Positives (Spec Issues)

### is_bit_set_not0_no_bound
- **Confidence:** high
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, so the axiom asserts properties about non-existent bit positions that could enable unsound reasoning chains.

### or_bit_set_no_bound
- **Confidence:** high
- **Reasoning:** The OR biconditional axiom `is_bit_set(a | b, j) <==> is_bit_set(a, j) || is_bit_set(b, j)` lacks a `j < 64` guard. For out-of-range bit positions where shift wraps, this relates three `is_bit_set` calls at wrapped positions where the logical meaning breaks down.

### lemma_view_no_range_guard
- **Confidence:** high
- **Reasoning:** The first ensures of `lemma_view` decomposes `i` into `a = i/64` and `b = i%64` then accesses `self.mask[a]` without guarding `0 <= a < 8`. For `i >= 512` or `i < 0`, this produces an out-of-bounds array access in spec, and the external_body axiom doesn't faithfully reflect the `view()` definition which does constrain `0 <= t.0 < 8`.

## All Candidates

### φ1: create_full_external_body_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `create_full` is `external_body` and claims to set exactly bits 0..512 — since it's unverified, it could set any bits, and the upper bound `!contains(512)` is never checked

### φ2: is_bit_set_not0_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_bit_set(!0usize, j)` is axiomatized for ALL `j: usize` with no `j < 64` guard — for `j >= 64`, `1usize << j` wraps, so the external_body axiom asserts a property about out-of-range bit positions
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, so the axiom asserts properties about non-existent bit positions that could enable unsound reasoning chains.

### φ3: or_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The OR biconditional axiom has no `j < 64` guard — for out-of-range positions where `1usize << j` wraps, this produces conclusions about wrapped bit operations that don't correspond to real bit semantics
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The OR biconditional axiom `is_bit_set(a | b, j) <==> is_bit_set(a, j) || is_bit_set(b, j)` lacks a `j < 64` guard. For out-of-range bit positions where shift wraps, this relates three `is_bit_set` calls at wrapped positions where the logical meaning breaks down.

### φ4: lemma_view_no_range_guard
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view` is `external_body` and its first ensures decomposes any `i` in `self@` without guarding `0 <= a < 8` — for `i >= 512` or `i < 0`, `a` falls outside `[0,8)` making `self.mask[a]` an out-of-bounds spec access
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The first ensures of `lemma_view` decomposes `i` into `a = i/64` and `b = i%64` then accesses `self.mask[a]` without guarding `0 <= a < 8`. For `i >= 512` or `i < 0`, this produces an out-of-bounds array access in spec, and the external_body axiom doesn't faithfully reflect the `view()` definition which does constrain `0 <= t.0 < 8`.

### φ5: lemma_change_one_entry_no_new_mask_constraint
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `lemma_change_one_entry` is `external_body` and places no constraint on what `other.mask[i]` must be — it axiomatically derives `other@` from whatever value is in `other.mask[i]`, trusting that the implementation correctly sets it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma correctly derives `other@` from whatever value is actually in `other.mask[i]` — this is exactly the intended design. The lemma relates the abstract view to the concrete mask values, and the caller (`create`) is responsible for setting `other.mask[i]` to the correct bitmask before invoking this lemma.

