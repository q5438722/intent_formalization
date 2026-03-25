# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/host_impl_v/host_impl_v__make_send_only_event_results.rs`
**Date:** 2026-03-24T09:05:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The `well_typed_events` predicate is intentionally permissive — it enforces type correctness (receives are Receive, sends are Send, etc.) and a single clock-read bound, while leaving cardinality and ordering constraints to higher-level protocol specifications. The empty batch, unbounded sends, fixed ordering, and multiple receives are all deliberate design choices consistent with IronFleet-style distributed systems verification.

## All Candidates

### φ1: well_typed_allows_empty_everything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An EventResults with all empty sequences is trivially well-typed — a completely empty event batch passes validation, potentially allowing no-op steps to refine real protocol transitions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty event batch being well-typed is standard — it represents a no-op or internal step. Whether such steps can refine real transitions is constrained elsewhere (e.g., `next_ensures`), not by `well_typed_events` alone.

### φ2: ios_decoupled_from_event_seq
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `ios` and `event_seq()` can disagree arbitrarily — `well_typed_events` constrains only `recvs`, `clocks`, `sends`, and `clocks.len()`, but places no constraint on `ios` matching the actual events, so refinement proofs may silently use fabricated ios

### φ3: well_typed_no_upper_bound_on_sends
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `well_typed_events` bounds `clocks.len() <= 1` but places no upper bound on `sends` or `recvs` — a single protocol step can claim an unbounded number of sends, potentially allowing a single transition to flood the network
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional for I/O automata modeling in IronFleet-style systems — a single protocol step may produce multiple sends (e.g., broadcasting to all replicas). The bound on clocks (≤1) reflects that you read the clock at most once per step, while sends are naturally unbounded.

### φ4: event_seq_ordering_fixed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `event_seq()` concatenates in fixed order `recvs + clocks + sends` — if the actual network interleaving was send-then-receive, the event sequence misrepresents the real causal order, potentially allowing invalid refinement proofs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The fixed `recvs + clocks + sends` ordering is the intentional design — the implementation first receives, then reads the clock, then sends. This canonical ordering simplifies refinement proofs and matches the actual execution model where receives precede sends within a single step.

### φ5: well_typed_allows_multiple_recvs
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `well_typed_events` allows multiple receives in a single event batch with no upper bound — most I/O automaton models expect at most one receive per step, so allowing multiple receives could break single-delivery assumptions in protocol refinement
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** While many I/O automaton formalizations restrict to one receive per step, the IronFleet/SHT model this code is based on allows batching multiple receives in a single step for performance. Additional constraints on receive counts would be imposed at the protocol level, not at the event-results typing level.

