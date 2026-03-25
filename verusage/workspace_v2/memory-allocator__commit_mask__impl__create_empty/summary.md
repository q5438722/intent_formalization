# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__impl__create_empty.rs`
**Date:** 2026-03-24T10:59:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives were found, all stemming from missing `j < 64` bounds in `external_body` axioms. Three relate to `lemma_is_bit_set`: the `!0usize` axiom, OR biconditional, and AND biconditional all lack guards for out-of-range bit positions where shift wrapping breaks the intended semantics. The fourth is `lemma_view`'s first ensures clause which doesn't bound `a` to `[0,8)`, allowing spec-level out-of-bounds array access. The disjoint axiom candidate is a false positive — it's a correct consequence of the other bounded axioms when `j < 64`.

## True Positives (Spec Issues)

### lemma_is_bit_set_not0_no_bound
- **Confidence:** high
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps (e.g., `1usize << 64 == 0` on a 64-bit platform), so `is_bit_set(!0usize, 64)` would expand to `!0usize & 0 == 0` which is `0 == 0 == true` — but this is accidental and the axiom asserts bit-set semantics for non-existent bit positions.

### or_bit_set_no_bound
- **Confidence:** high
- **Reasoning:** The OR biconditional axiom lacks a `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, and the biconditional relates three `is_bit_set` calls at wrapped positions where the logical meaning of "bit j is set" breaks down — this could enable unsound reasoning chains.

### and_bit_set_no_bound
- **Confidence:** high
- **Reasoning:** Same issue as OR — the AND biconditional axiom has no `j < 64` guard. For out-of-range bit positions, `1usize << j` wraps and the biconditional asserts equivalences about wrapped bit operations that don't correspond to actual bit semantics.

### lemma_view_external_body_decomposition
- **Confidence:** high
- **Reasoning:** The first ensures clause of `lemma_view` decomposes any `i` in `self@` into `a = i / 64` and `b = i % 64`, then asserts `is_bit_set(self.mask[a], b)` — but there's no guard that `0 <= a < 8`. The `view()` definition does constrain elements to `0 <= t.0 < 8`, but the external_body lemma doesn't faithfully reflect this, so for out-of-range `i` the axiom accesses `self.mask[a]` out of bounds in spec.

## All Candidates

### φ1: lemma_is_bit_set_not0_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_bit_set(!0usize, j)` is asserted for ALL `j: usize` with no `j < 64` guard — for `j >= 64`, `1usize << j` is undefined/zero, so the external_body axiom claims a property about out-of-range bit positions that shouldn't hold
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The axiom `forall|j: usize| is_bit_set(!0usize, j)` has no `j < 64` guard. For `j >= 64`, `1usize << j` wraps (e.g., `1usize << 64 == 0` on a 64-bit platform), so `is_bit_set(!0usize, 64)` would expand to `!0usize & 0 == 0` which is `0 == 0 == true` — but this is accidental and the axiom asserts bit-set semantics for non-existent bit positions.

### φ2: or_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The OR biconditional has no `j < 64` guard — for out-of-range bit positions where `1usize << j` wraps, this axiom could produce inconsistent conclusions about bit operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The OR biconditional axiom lacks a `j < 64` guard. For `j >= 64`, `1usize << j` wraps around, and the biconditional relates three `is_bit_set` calls at wrapped positions where the logical meaning of "bit j is set" breaks down — this could enable unsound reasoning chains.

### φ3: and_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The AND biconditional has no `j < 64` guard — for `j >= 64`, `1usize << j` wraps around, and asserting biconditional equivalence at these positions could enable unsound reasoning
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same issue as OR — the AND biconditional axiom has no `j < 64` guard. For out-of-range bit positions, `1usize << j` wraps and the biconditional asserts equivalences about wrapped bit operations that don't correspond to actual bit semantics.

### φ4: lemma_view_external_body_decomposition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view` is `external_body` and its first ensures has no bound on `a` — for `i >= 512` or `i < 0`, `a` could be out of `[0,8)` range, making `self.mask[a]` an out-of-bounds access in the spec that the trusted axiom doesn't guard against
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The first ensures clause of `lemma_view` decomposes any `i` in `self@` into `a = i / 64` and `b = i % 64`, then asserts `is_bit_set(self.mask[a], b)` — but there's no guard that `0 <= a < 8`. The `view()` definition does constrain elements to `0 <= t.0 < 8`, but the external_body lemma doesn't faithfully reflect this, so for out-of-range `i` the axiom accesses `self.mask[a]` out of bounds in spec.

### φ5: disjoint_axiom_redundant_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The sixth axiom clause (`a & b == 0 ==> !(is_bit_set(a,j) && is_bit_set(b,j))`) is claimed to be "implied by previous properties" but being external_body means the implication is never checked — if the AND biconditional axiom is wrong for any edge case, this redundant axiom adds an independent unsound assumption
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Given `j < 64`, the AND biconditional axiom (`is_bit_set(a & b, j) <==> is_bit_set(a, j) && is_bit_set(b, j)`) combined with `a & b == 0` and the zero axiom (`j < 64 ==> !is_bit_set(0, j)`) already implies `!is_bit_set(b, j)` when `is_bit_set(a, j)`. The sixth clause is indeed redundant under the bounded case, and the property shown here is a correct consequence of the other axioms at `j < 64`.

