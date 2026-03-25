# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__impl2__entry_base.rs`
**Date:** 2026-03-24T12:17:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `mult_leq_mono_both` is an `external_body` axiom trusted without proof for multiplication monotonicity, which `entry_base` depends on for overflow checking. The other three are false positives — one-past-the-end indexing, empty architecture satisfying inv, and vacuous last-layer implication are all intended design choices.

## True Positives (Spec Issues)

### mult_leq_mono_both_external_body
- **Confidence:** medium
- **Reasoning:** `mult_leq_mono_both` is `external_body` with `unimplemented!()` body — monotonicity of multiplication over naturals is a correct mathematical fact but trusted without proof. The `entry_base` exec function depends on this axiom for overflow safety.

## All Candidates

### φ1: mult_leq_mono_both_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mult_leq_mono_both` is `external_body` — monotonicity of multiplication over naturals is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mult_leq_mono_both` is `external_body` with `unimplemented!()` body — monotonicity of multiplication over naturals is a correct mathematical fact but trusted without proof. The `entry_base` exec function depends on this axiom for overflow safety.

### φ2: entry_size_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `ArchExec::entry_size` is `external_body` — the exec accessor's correspondence with the spec is trusted without implementation

### φ3: entry_base_idx_equals_num_entries
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `idx` can equal `X86_NUM_ENTRIES` (512), not just be less than it — the precondition allows one-past-the-end index, producing an address potentially beyond the mapped region
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Allowing `idx == X86_NUM_ENTRIES` (512) is standard for computing one-past-the-end addresses in page table traversal. The bound `idx <= X86_NUM_ENTRIES` is intentional for computing the end of a mapped region.

### φ4: inv_no_layer_count_lower_bound
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An `Arch` with zero layers satisfies `inv()` — the invariant only bounds layers from above (`<= X86_NUM_LAYERS`) but has no minimum, so an empty architecture is considered valid
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** An empty `Arch` satisfying `inv()` is a vacuous base case. In practice, the x86 architecture always has 4 layers, and callers that need non-empty architectures add `layers.len() > 0` as a separate precondition. The invariant is intentionally structural.

### φ5: entry_size_is_next_vacuous_at_last
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `entry_size_is_next_layer_size` is vacuously true for the last layer since `i + 1 >= layers.len()` — the leaf layer's entry_size is unconstrained relative to any sub-structure
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The last layer has no sub-layer, so `entry_size_is_next_layer_size` being vacuously true is correct by design. The leaf layer's entry_size (4096 for x86) is constrained by the `0 < entry_size <= X86_MAX_ENTRY_SIZE` bound, not by a non-existent next layer.

