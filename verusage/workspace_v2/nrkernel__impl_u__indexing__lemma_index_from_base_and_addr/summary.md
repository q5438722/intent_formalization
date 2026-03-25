# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__indexing/impl_u__indexing__lemma_index_from_base_and_addr.rs`
**Date:** 2026-03-24T12:37:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `subtract_mod_eq_zero` and `div_mul_cancel` are both `external_body` axioms trusting correct but unproven mathematical facts (alignment subtraction closure and division-multiplication cancellation). The other three are false positives — downstream consequences proved by nonlinear arithmetic or redundant with the identified trust gaps.

## True Positives (Spec Issues)

### subtract_mod_eq_zero_external_body
- **Confidence:** medium
- **Reasoning:** `subtract_mod_eq_zero` is `external_body` with `unimplemented!()` body — alignment closure under subtraction is a correct mathematical fact but trusted without proof. Used in the aligned-address-equals-entry-base proof path.

### div_mul_cancel_external_body
- **Confidence:** medium
- **Reasoning:** `div_mul_cancel` is `external_body` with old-style `requires`/`ensures` syntax — the division-multiplication roundtrip for aligned values is trusted without proof. Used alongside `subtract_mod_eq_zero` to establish the exact address equality.

## All Candidates

### φ1: subtract_mod_eq_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `subtract_mod_eq_zero` is `external_body` — closure of alignment under subtraction is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `subtract_mod_eq_zero` is `external_body` with `unimplemented!()` body — alignment closure under subtraction is a correct mathematical fact but trusted without proof. Used in the aligned-address-equals-entry-base proof path.

### φ2: div_mul_cancel_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `div_mul_cancel` is `external_body` — the division-multiplication roundtrip for aligned values is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `div_mul_cancel` is `external_body` with old-style `requires`/`ensures` syntax — the division-multiplication roundtrip for aligned values is trusted without proof. Used alongside `subtract_mod_eq_zero` to establish the exact address equality.

### φ3: aligned_addr_equals_entry_base
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An aligned address equals the entry base at its index — depends on both `subtract_mod_eq_zero` and `div_mul_cancel` external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of the lemma's ensures clause. The trust gaps are already captured by φ1 and φ2; this adds no new unverified assumption.

### φ4: index_in_bounds
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The computed index is always within bounds — verified by nonlinear_arith without external_body dependency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved purely by `nonlinear_arith` with no external_body dependency. Standard floor-division bound.

### φ5: addr_between_entry_bases
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any address falls between consecutive entry bases at its index — verified by nonlinear_arith, tests the floor-division localization property
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved purely by `nonlinear_arith` with no external_body dependency. Standard floor-division localization property.

