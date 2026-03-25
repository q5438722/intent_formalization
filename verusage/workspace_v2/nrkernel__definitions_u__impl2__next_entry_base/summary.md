# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__impl2__next_entry_base.rs`
**Date:** 2026-03-24T12:18:48Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `overflow_bounds` is an `external_body` axiom that trusts concrete arithmetic overflow bounds without proof. The other two are false positives — the max-index end-of-region address is intended behavior, and the one-past-valid-range case operates outside the function's recommended preconditions.

## True Positives (Spec Issues)

### overflow_bounds_external_body
- **Confidence:** medium
- **Reasoning:** `overflow_bounds` is `external_body` with `unimplemented!()` body — the concrete arithmetic bounds for u64 overflow safety are trusted without proof. While the facts are numerically correct (easily checked by a calculator), they form the unverified foundation for the `next_entry_base` exec function's overflow safety.

## All Candidates

### φ1: overflow_bounds_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `overflow_bounds` is `external_body` — the u64 overflow safety of max entry calculations is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `overflow_bounds` is `external_body` with `unimplemented!()` body — the concrete arithmetic bounds for u64 overflow safety are trusted without proof. While the facts are numerically correct (easily checked by a calculator), they form the unverified foundation for the `next_entry_base` exec function's overflow safety.

### φ2: entry_size_exec_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `ArchExec::entry_size` is `external_body` — the exec-to-spec correspondence for layer entry size is trusted without implementation

### φ3: next_entry_base_off_by_one
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `next_entry_base` computes `base + (idx+1)*entry_size` which equals `entry_base(idx) + entry_size` — it returns the END of the idx-th entry, not the START of the (idx+1)-th, but these are the same value, potentially confusing callers about semantics

### φ4: next_entry_base_at_max_idx
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** At the last valid index, `next_entry_base` equals `base + num_entries * entry_size` — the function can produce addresses at the very end of the layer's address space
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** At the last valid index `num_entries - 1`, `next_entry_base` computes `base + num_entries * entry_size`, which is exactly the end of the mapped region. This is the intended one-past-the-end address for the layer, used in page table traversal.

### φ5: next_entry_base_exceeds_parent_entry
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When `idx == num_entries` (one past valid range), `next_entry_base` produces an address beyond the parent entry's mapped region — no runtime guard prevents this since the precondition only bounds `idx <= X86_NUM_ENTRIES`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The φ's precondition sets `idx == num_entries(layer)`, which violates the `recommends` for `next_entry_base` (valid indices are `0..num_entries-1`). The exec `next_entry_base` requires `idx <= X86_NUM_ENTRIES` as a loose overflow bound, but callers are expected to stay within valid index ranges.

