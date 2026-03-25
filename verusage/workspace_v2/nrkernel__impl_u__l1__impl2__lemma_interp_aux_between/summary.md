# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_interp_aux_between.rs`
**Date:** 2026-03-24T12:44:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `lemma_interp_of_entry_between` and `lemma_entry_base_from_index` are both `external_body` axioms trusting correct but unproven properties — entry interpretation VA/frame bounds and entry base address strict monotonicity respectively. The other three are false positives — redundant ensures clauses from the same axioms and a verified downstream consequence.

## True Positives (Spec Issues)

### interp_of_entry_between_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_between` is `external_body` with `unimplemented!()` body — the VA bounding property `entry_base(i) <= va < next_entry_base(i)` for all keys in an entry's interpretation is trusted without proof. This is the foundational axiom for the entire interp_aux bounding chain.

### lemma_entry_base_strict_monotone_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_entry_base_from_index` is `external_body` with `unimplemented!()` body — strict monotonicity of `base + idx * entry_size` is a correct arithmetic fact but trusted without proof. Used by `lemma_interp_aux_between` for the inductive step.

## All Candidates

### φ1: interp_of_entry_between_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_between` is `external_body` — VA and frame size bounds for entry interpretations are trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_between` is `external_body` with `unimplemented!()` body — the VA bounding property `entry_base(i) <= va < next_entry_base(i)` for all keys in an entry's interpretation is trusted without proof. This is the foundational axiom for the entire interp_aux bounding chain.

### φ2: interp_of_entry_frame_bound_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The frame size upper bound `va + frame.size <= next_entry_base` is also external_body — if wrong, frames could extend past entry boundaries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the second ensures clause of the same `external_body` axiom already captured by φ1. Both VA bounds and frame size bounds are part of a single trust assumption — flagging them separately adds no new information.

### φ3: lemma_entry_base_strict_monotone_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_entry_base_from_index` is `external_body` — strict monotonicity of entry base addresses is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_entry_base_from_index` is `external_body` with `unimplemented!()` body — strict monotonicity of `base + idx * entry_size` is a correct arithmetic fact but trusted without proof. Used by `lemma_interp_aux_between` for the inductive step.

### φ4: next_entry_leq_next_base_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The gap-free property `next_entry_base(i) <= entry_base(j)` for `i < j` is external_body — if wrong, entry ranges could overlap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is another ensures clause of the same `lemma_entry_base_from_index` external_body already captured by φ3. The gap-free property is part of the same trust assumption.

### φ5: interp_aux_va_upper_bounded
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The verified `lemma_interp_aux_between` bounds all VA keys below `upper_vaddr` — depends on both external_body axioms for correctness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of `lemma_interp_aux_between`, which has a full proof body. The trust gaps are already captured by φ1 and φ3; this adds no new unverified assumption.

