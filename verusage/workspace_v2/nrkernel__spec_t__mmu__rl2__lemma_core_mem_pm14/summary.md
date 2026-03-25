# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_core_mem_pm14.rs`
**Date:** 2026-03-24T13:33:08Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_write_seq` is an external_body trust assumption. Three false positives: per-core store buffer isolation is intentional TSO modeling, the pml4 dependency is a duplicate of the external_body finding, and the TSO/sbuf disconnect is an artifact of manual state construction rather than a real spec gap.

## True Positives (Spec Issues)

### lemma_write_seq_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — its postconditions (pml4 preservation and domain monotonicity) are trusted without proof. This is an unverified trust assumption in the proof chain.

## All Candidates

### φ1: lemma_write_seq_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` with `unimplemented!()` — pml4 preservation and domain monotonicity are trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — its postconditions (pml4 preservation and domain monotonicity) are trusted without proof. This is an unverified trust assumption in the proof chain.

### φ2: valid_core_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `valid_core` is `external_body` with no spec body — completely opaque, the SMT solver could derive anything about core validity

### φ3: core_mem_ignores_other_sbufs
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `core_mem` only applies the store buffer of the given core — modifications to other cores' store buffers are invisible, meaning each core sees a potentially stale view of page table memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intentional TSO (Total Store Order) memory model design. Each core's view of page table memory includes only its own pending store buffer entries — this correctly models x86 TSO where a core sees its own writes before they become globally visible, but not other cores' buffered writes.

### φ4: core_mem_pml4_relies_on_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The proof that `core_mem` preserves `pml4` depends on `lemma_write_seq` (external_body) — the entire per-core memory consistency guarantee rests on an unverified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a duplicate of φ1 — it merely demonstrates a downstream consequence of the same `external_body` trust assumption. The issue is `lemma_write_seq` itself (already captured), not the fact that `lemma_core_mem_pml4` calls it.

### φ5: writes_tso_not_connected_to_sbuf
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Adding to a core's store buffer doesn't update `writes.tso` — the TSO tracking set and the actual store buffers are disconnected spec-level fields with no invariant linking them
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `Writes` struct and store buffers are maintained by different transition steps in the state machine. `writes.tso` is updated by the actual transition functions (write steps, drain steps), not by raw struct manipulation. This φ constructs a state manually without going through transitions — the invariant linking them is maintained by the transition system, not by the data types themselves.

