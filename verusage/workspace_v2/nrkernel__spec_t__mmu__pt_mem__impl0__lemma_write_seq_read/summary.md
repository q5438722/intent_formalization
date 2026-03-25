# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__impl0__lemma_write_seq_read.rs`
**Date:** 2026-03-24T13:31:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The `PTMem` write/write_seq operations and the `lemma_write_seq_read` lemma exhibit correct, expected behavior: single writes store values, last-write-wins for duplicates, disjoint writes don't interfere, domain expansion is handled at higher layers, and empty sequences are identity. No spec gaps found in this batch.

## All Candidates

### φ1: write_seq_read_unique_key
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Tests that a single-element write_seq correctly stores the value — basic sanity check of the lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A single-element write_seq correctly returning the written value is the expected and desirable behavior of sequential writes. This is basic correctness, not a spec gap.

### φ2: write_seq_read_requires_uniqueness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** With duplicate keys, `lemma_write_seq_read` can't be applied (uniqueness precondition fails) — but fold_left still gives last-write-wins, testing that the spec is consistent even without the lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Last-write-wins for duplicate keys is standard map semantics via `fold_left`. This is correct behavior — sequential writes to the same address naturally result in the last value persisting.

### φ3: write_seq_read_does_not_require_domain
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_read` has no precondition requiring the address to be in the original domain — writes can target arbitrary addresses outside allocated page table memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem` is a low-level memory abstraction using `Map<usize, usize>`. Bounds checking on valid page table addresses is enforced at higher layers. The lemma correctly describes write behavior regardless of prior domain membership.

### φ4: write_seq_drop_last_preserves_earlier
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Earlier writes to distinct addresses survive later writes — tests that the inductive case correctly preserves non-overwritten entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Writes to distinct addresses not interfering with each other is correct and desirable behavior. This validates that the inductive proof correctly handles the non-overwritten case.

### φ5: write_seq_empty_no_effect
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty write_seq is identity — tests fold_left base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty write sequence producing no changes is the correct base case for `fold_left`. This is standard and expected behavior.

