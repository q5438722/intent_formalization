# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_obtain_bit_index_1.rs`
**Date:** 2026-03-24T11:11:41Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_obtain_bit_index_1_aux` is an `external_body` axiom trusting the existence of a set bit below `hi` in any nonzero value without proof. While mathematically correct, this represents an unverified trust assumption. The other four candidates are false positives — they are either correct mathematical consequences of the axioms (boundary cases, tautological reveals, fundamental bit properties) or have vacuously true ensures from contradictory preconditions.

## True Positives (Spec Issues)

### obtain_bit_aux_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_obtain_bit_index_1_aux` is `external_body` — the property that any nonzero value has a set bit below `hi` is mathematically true but unverified. This is a trust gap: the axiom could be subtly wrong (e.g., off-by-one in `hi` bound) with no verification to catch it.

## All Candidates

### φ1: obtain_bit_aux_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_obtain_bit_index_1_aux` is `external_body` — the existence of a set bit index below `hi` in a nonzero value is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_obtain_bit_index_1_aux` is `external_body` — the property that any nonzero value has a set bit below `hi` is mathematically true but unverified. This is a trust gap: the axiom could be subtly wrong (e.g., off-by-one in `hi` bound) with no verification to catch it.

### φ2: obtain_bit_hi_1_forces_bit_0
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `hi=1`, the only valid index is 0 — the external_body axiom forces `is_bit_set(a, 0)` for any nonzero `a` with `a >> 1 == 0`, which should only be `a == 1`; if the axiom is wrong this narrows incorrectly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The preconditions `a != 0` and `a >> 1 == 0` together force `a == 1`, so the only set bit is bit 0. The axiom correctly returns `i == 0` and `is_bit_set(a, 0)` — this is the expected and correct behavior for this boundary case.

### φ3: is_bit_set_opaque_reveal
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_bit_set` is a non-opaque spec fn whose body is `a & (1usize << b) == (1usize << b)` — revealing it should be equivalent to the body, but if bitvector reasoning interacts unexpectedly with the spec definition, this could fail or succeed unsoundly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_bit_set` is a plain spec fn with body `a & (1usize << b) == (1usize << b)`. Revealing it and then proving equivalence with its own body is a tautology — this is standard Verus usage, not a spec gap.

### φ4: obtain_bit_index_any_nonzero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The lemma claims any nonzero usize has a set bit below 64 — this relies on `global size_of usize == 8` and the external_body axiom; if either is wrong for a target platform, this existential is unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Any nonzero 64-bit integer must have at least one set bit — this is a fundamental mathematical fact. With `global size_of usize == 8` fixing usize to 64 bits, the property is correct and expected.

### φ5: obtain_bit_aux_hi_zero_vacuous
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `hi=0` and `a >> 0 == 0` (meaning `a == 0`), the preconditions `a != 0` and `a >> 0 == 0` are contradictory — but if the external_body axiom doesn't handle this edge correctly, it could derive `false` from a satisfiable-looking precondition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The preconditions `a != 0` and `a >> 0 == 0` are contradictory since `a >> 0 == a` and `a != 0` implies `a >> 0 != 0`. The `requires` is unsatisfiable, so the `ensures false` is vacuously true — no soundness issue.

