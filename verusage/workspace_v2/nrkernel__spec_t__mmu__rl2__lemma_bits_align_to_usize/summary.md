# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_bits_align_to_usize.rs`
**Date:** 2026-03-24T13:37:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The bitmask range, alignment idempotency, L2-alignment zeroing L3 bits, and zero-alignment identity are all correct, desirable mathematical properties of the bit manipulation and alignment functions, verified by bit_vector reasoning.

## All Candidates

### φ1: bitmask_l0_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** l0_bits extracts bits 39-47 (9 bits) so the result must be < 512 — tests that the bitmask correctly bounds the index
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting 9 bits (39-47) correctly yields a value in [0, 512). This is a correct and desirable property verified by bit_vector reasoning — it confirms the bitmask is properly constructed.

### φ2: align_to_usize_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Aligning an already-aligned address should be identity — tests that `sub(a, 0) == a`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An already-aligned address returning itself after alignment is correct and expected. `sub(a, 0) == a` is trivially true and this is desirable behavior.

### φ3: align_to_usize_rounds_down
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `align_to_usize` should always round down — if it rounds up, aligned addresses could exceed the original

### φ4: l3_bits_zero_for_page_aligned
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An L2-aligned address should have zero L3 bits — tests that L2 alignment implies the L3 index field is zero
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An L2-aligned address (2MB-aligned) has bits 12-20 all zero, so l3_bits being zero is correct. This is a desirable mathematical property of the bit-field hierarchy.

### φ5: align_to_usize_underflow
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Aligning zero should produce zero regardless of alignment size — tests that `sub(0, 0 % b) == 0` doesn't underflow
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Aligning zero to any positive alignment produces zero — `sub(0, 0 % b) == sub(0, 0) == 0`. This is correct and expected behavior with no underflow risk.

