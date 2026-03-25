# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_obtain_bit_index_3_aux.rs`
**Date:** 2026-03-24T11:15:59Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The lemma `lemma_obtain_bit_index_3_aux` is fully verified with bit_vector proofs (not external_body), and all tested properties are correct mathematical consequences: witness existence at hi=64, boundary behavior at hi=1, concrete case verification, bound propagation, and basic non-zero derivation. No spec issues were found.

## All Candidates

### φ1: not_subset_has_witness
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any pair where `a & b != b` has a witness bit set in `b` but clear in `a` below 64 — this is mathematically correct and serves as a sanity check that the lemma is usable at hi=64
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct mathematical fact — any `a & b != b` means `b` has bits not in `a`, so a witness exists below 64. The lemma is fully verified (not external_body), so this is an expected consequence.

### φ2: hi_1_forces_bit_0
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With hi=1, the only possible witness index is 0 — tests that the recursion base case correctly identifies the lowest bit
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `hi=1`, `i < 1` forces `i == 0`. The preconditions `a >> 1 == 0` and `b >> 1 == 0` constrain both to single-bit values, and `a & b != b` forces `a=0, b=1`. This is correct boundary behavior.

### φ3: concrete_witness_0_and_1
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** For a=0, b=1 the unique witness is bit 0 — verifies the lemma produces the correct concrete result
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `a=0, b=1`, the only set bit in `b` is bit 0, so `i == 0` is the unique witness. The bit_vector assertion correctly constrains `i`. This is expected behavior for concrete inputs.

### φ4: result_strictly_less_than_hi
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The result index is strictly below hi — a weaker consequence that tests the bound propagation of the decreases/ensures interaction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct weakening of the lemma's own ensures clause (`i < hi`). It's trivially a correct consequence, not a spec gap.

### φ5: witness_implies_b_nonzero
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If `a & b != b` then `b` must be nonzero (it has a bit not covered by `a`) — tests that the ensures is strong enough to derive this basic consequence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `a & b != b` trivially implies `b != 0` (if `b == 0` then `a & 0 == 0 == b`). The lemma's ensures provides a witness bit in `b`, which also implies `b != 0`. This is a correct and expected consequence.

