# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_obtain_bit_index_3.rs`
**Date:** 2026-03-24T11:14:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_obtain_bit_index_3_aux` is an `external_body` axiom trusting the witness existence property for bitwise non-subset without proof. The other three candidates are false positives — they are correct mathematical consequences derived from the verified wrapper `lemma_obtain_bit_index_3` or from concrete bit_vector reasoning on specific inputs.

## True Positives (Spec Issues)

### obtain_bit_index_3_aux_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_obtain_bit_index_3_aux` is `external_body` — the property that `a & b != b` implies existence of a bit set in `b` but not `a` below `hi` is mathematically true, but trusted without proof. This is a real verification trust gap.

## All Candidates

### φ1: obtain_bit_index_3_aux_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_obtain_bit_index_3_aux` is `external_body` — the existence of a bit set in `b` but not in `a` when `a & b != b` is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_obtain_bit_index_3_aux` is `external_body` — the property that `a & b != b` implies existence of a bit set in `b` but not `a` below `hi` is mathematically true, but trusted without proof. This is a real verification trust gap.

### φ2: not_subset_has_witness
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any pair where `a & b != b` has a witness bit set in `b` but clear in `a` — this depends entirely on the unverified external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct wrapper around the verified `lemma_obtain_bit_index_3`, which itself is fully proved (modulo the external_body dependency). The property — that bitwise non-subset implies a witness bit — is mathematically correct and expected.

### φ3: zero_not_superset_of_nonzero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Using `lemma_obtain_bit_index_3` with `a=0` derives that any nonzero usize has a set bit — this piggybacks on the external_body axiom to get `lemma_obtain_bit_index_1`-like functionality
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Any nonzero value has a set bit — this is a correct mathematical consequence. The `0 & b != b` assertion is verified by bit_vector, and the rest follows from `lemma_obtain_bit_index_3`. This is expected behavior, not a spec gap.

### φ4: witness_bit_position_unique_power_of_two
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** For a=0, b=1 the only witness is bit 0 — tests that the external_body axiom returns the correct specific index for concrete inputs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `a=0, b=1`, the only set bit in `b` is bit 0, so `i == 0` is the unique witness. The bit_vector assertion correctly constrains `i` to 0. This is a correct boundary case, not a spec issue.

### φ5: is_bit_set_wrap_at_64
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `1usize << 64` wraps to `1usize << 0` on 64-bit, so `is_bit_set(a, 64) == is_bit_set(a, 0)` — the spec fn conflates bit 0 and "bit 64" due to missing bounds check in its definition

