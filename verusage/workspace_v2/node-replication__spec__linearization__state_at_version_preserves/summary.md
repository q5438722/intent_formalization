# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_linearization/spec__linearization__state_at_version_preserves.rs`
**Date:** 2026-03-24T12:07:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. Prefix-independence under append and the version-0 base case are correct and intended properties of the `compute_nrstate_at_version` spec. No trust gaps exist in this file — all proofs are fully verified.

## All Candidates

### φ1: push_preserves_prefix_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Appending an operation preserves state at all prior versions — the new operation is invisible to prefix computations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `compute_nrstate_at_version` only accesses `log[0]` through `log[version-1]`. Appending to the end doesn't change any of these indices. This is the correct and intended prefix-independence property proved by `state_at_version_preserves`.

### φ2: version_zero_always_init
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Version 0 always returns `init_spec()` regardless of log contents — the initial state is hardcoded
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case `version == 0` returning `DT::init_spec()` is directly stated in the open spec definition. This is the intended initial state of the replicated state machine.

### φ3: single_op_state
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single-element log at version 1 applies the op to init_spec — tests that the recursion unfolds correctly for the minimal non-trivial case

### φ4: dispatch_mut_spec_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `dispatch_mut_spec` is a trait method with no constraints — any implementation can define arbitrary state transitions, and the spec blindly trusts it

### φ5: refinement_proof_phantom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `RefinementProof` is a unit-like tracked struct — its existence is unconditionally provable, meaning the refinement witness carries no actual proof content

