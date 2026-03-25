# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_iter_walk_equals_pt_walk.rs`
**Date:** 2026-03-24T13:45:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: the `MAX_PHYADDR_WIDTH` gap between the external_body value (52) and the axiom constraint (32..=52) means all dependent address masks are underspecified from Verus's perspective. Two false positives: the pt_walk/iter_walk equivalence is an existing proven lemma, and pt_walk determinism is a trivial tautology.

## True Positives (Spec Issues)

### max_phyaddr_width_gap
- **Confidence:** high
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with runtime value 52, but the axiom only constrains it to `32..=52`. All address masks (`MASK_ADDR`, `MASK_L1_PG_ADDR`, etc.) depend on this value, so from Verus's perspective, they could correspond to any physical address width in that range, making the verification unsound for any specific hardware assumption.

## All Candidates

### φ1: pt_walk_iter_walk_equivalence_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Duplicate of existing lemma — confirms the equivalence but `pt_walk` relies on `PDE::view` which uses `all_mb0_bits_are_zero` (external_body), propagating the opaque predicate through both walk implementations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is an exact duplicate of the already-proven `lemma_iter_walk_equals_pt_walk`. The equivalence between `pt_walk` and `iter_walk` is correct and desirable — it confirms the unrolled walk matches the iterative walk. The external_body dependency is a separate concern not exposed by this φ.

### φ2: pt_walk_layer0_non_present_invalid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When the L0 entry has P bit clear, the walk should terminate immediately with a single-entry path — tests that non-present entries at the root produce a 1-entry completed walk

### φ3: pt_walk_deterministic
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pt_walk` is a pure spec function — calling it twice on the same inputs must yield the same result, but the `all_mb0_bits_are_zero` external_body could theoretically break this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x == x` is a tautology for any spec function in Verus. Even with `external_body` predicates inside, spec functions are deterministic by construction — they map the same inputs to the same outputs. This proves nothing.

### φ4: walk_next_at_layer0_uses_pml4
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The first walk step depends only on pml4 and the L0 entry — two memories agreeing on these should produce identical first steps, testing that walk_next doesn't depend on unrelated memory

### φ5: max_phyaddr_width_gap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52 but axiom only constrains to `32..=52` — the actual value could be any width in that range, making MASK_ADDR and all address masks potentially wrong
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with runtime value 52, but the axiom only constrains it to `32..=52`. All address masks (`MASK_ADDR`, `MASK_L1_PG_ADDR`, etc.) depend on this value, so from Verus's perspective, they could correspond to any physical address width in that range, making the verification unsound for any specific hardware assumption.

