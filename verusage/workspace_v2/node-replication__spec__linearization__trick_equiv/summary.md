# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_linearization/spec__linearization__trick_equiv.rs`
**Date:** 2026-03-24T12:08:52Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. `behavior_equiv` is a standard trace equivalence relation that compares only external (Start/End) labels and ignores internal steps and states. This is the intended design for refinement between `SimpleLog` and `AsynchronousSingleton` — equivalence means identical observable behavior, not identical internal state trajectories.

## All Candidates

### φ1: behavior_equiv_both_inited
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Any two Inited behaviors are equivalent regardless of their states — `behavior_equiv` only checks structural variant, not state content, so unrelated initial states are considered equivalent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `behavior_equiv` is a trace equivalence relation that only compares observable (external) labels. Two Inited behaviors have no observable steps, so they are trivially equivalent. The states are irrelevant because equivalence is about matching external action sequences, not internal state.

### φ2: internal_steps_freely_added
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An arbitrary internal step with any state can be prepended to an equivalent behavior — `behavior_equiv` skips internal steps without checking state validity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Internal steps are invisible in trace equivalence by design — `behavior_equiv` explicitly skips internal labels on both sides. Prepending an internal step doesn't change the observable behavior. This is the standard stuttering/internal-action treatment in refinement proofs.

### φ3: trick_equiv_swaps_states
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The state in an internal step can be replaced with any other state without breaking equivalence — `trick_equiv` proves internal step states are completely irrelevant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Since `behavior_equiv` only examines labels (not states) and internal labels are skipped entirely, the state attached to an internal step is irrelevant. The `trick_equiv` lemma correctly captures this — it's a utility for refinement proofs where intermediate states may differ.

### φ4: infinite_internal_steps_equiv
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Arbitrarily many internal steps can be stacked on the `a` side — `behavior_equiv` peels them off recursively, so any number of unvalidated internal transitions are invisible

### φ5: async_singleton_internal_skip
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The state in an internal step on the `b` side can also be freely swapped — `behavior_equiv` ignores the state of internal transitions on both sides

