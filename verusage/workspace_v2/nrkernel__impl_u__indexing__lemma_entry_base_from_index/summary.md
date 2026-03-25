# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__indexing/impl_u__indexing__lemma_entry_base_from_index.rs`
**Date:** 2026-03-24T12:36:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `mod_add_zero`, `aligned_transitive`, and `mult_less_mono_both1` are all `external_body` axioms trusting correct but unproven mathematical facts (alignment additivity, divisibility transitivity, and multiplication monotonicity). The other two are false positives — downstream consequences of the verified lemma with no new trust gaps beyond those already identified.

## True Positives (Spec Issues)

### mod_add_zero_external_body
- **Confidence:** medium
- **Reasoning:** `mod_add_zero` is `external_body` with `unimplemented!()` body — closure of alignment (divisibility) under addition is a correct mathematical fact but trusted without proof. Used in the alignment proofs for both `entry_base_from_index` and `next_entry_base_from_index`.

### aligned_transitive_external_body
- **Confidence:** medium
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — divisibility transitivity is trusted without proof. Used to derive that `idx * entry_size` aligned to `entry_size` implies alignment to any `n` that divides `entry_size`.

### mult_less_mono_both1_external_body
- **Confidence:** medium
- **Reasoning:** `mult_less_mono_both1` is `external_body` with `unimplemented!()` body — multiplication monotonicity with mixed strict/non-strict bounds is trusted without proof. Used to prove index monotonicity of `entry_base_from_index`.

## All Candidates

### φ1: mod_add_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mod_add_zero` is `external_body` — closure of alignment under addition is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mod_add_zero` is `external_body` with `unimplemented!()` body — closure of alignment (divisibility) under addition is a correct mathematical fact but trusted without proof. Used in the alignment proofs for both `entry_base_from_index` and `next_entry_base_from_index`.

### φ2: aligned_transitive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `aligned_transitive` is `external_body` — divisibility transitivity is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — divisibility transitivity is trusted without proof. Used to derive that `idx * entry_size` aligned to `entry_size` implies alignment to any `n` that divides `entry_size`.

### φ3: mult_less_mono_both1_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mult_less_mono_both1` is `external_body` — multiplication monotonicity with mixed strict/non-strict inequalities is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mult_less_mono_both1` is `external_body` with `unimplemented!()` body — multiplication monotonicity with mixed strict/non-strict bounds is trusted without proof. Used to prove index monotonicity of `entry_base_from_index`.

### φ4: entry_base_aligned_from_base_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Entry base alignment to `entry_size` follows from base alignment — the proof chain uses both `aligned_transitive` and `mod_add_zero` external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of the lemma's ensures clause. The trust gaps are already captured by φ1 and φ2; this adds no new unverified assumption.

### φ5: next_entry_base_no_gap
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The gap between consecutive entries is exactly `entry_size` — tests that entries are contiguous with no dead space
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `base + (idx+1)*entry_size == base + idx*entry_size + entry_size` is basic algebra proved by `nonlinear_arith`. No external_body dependency for this property.

