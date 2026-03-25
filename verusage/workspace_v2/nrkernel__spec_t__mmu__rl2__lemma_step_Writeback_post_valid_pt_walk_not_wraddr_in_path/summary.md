# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_Writeback_post_valid_pt_walk_not_wraddr_in_path.rs`
**Date:** 2026-03-24T13:55:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `valid_core` is an external_body predicate with no specification, representing an unverified trust assumption for core validity throughout the MMU spec. Three false positives: pml4 preservation, write-read consistency, and write non-interference are all correct properties following directly from the open spec definitions and standard vstd map axioms.

## True Positives (Spec Issues)

### valid_core_external_body
- **Confidence:** medium
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` — the core validity predicate is completely opaque and unverified. While functional consistency on equal inputs holds trivially for spec functions, the underlying predicate itself is an unauditable trust assumption used throughout the MMU spec.

## All Candidates

### φ1: write_preserves_pml4
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `PTMem::write` explicitly preserves pml4 — tests that individual writes never change the page table root pointer
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem::write` explicitly constructs `PTMem { mem: self.mem.insert(addr, value), pml4: self.pml4 }` — pml4 preservation is correct by construction and desirable.

### φ2: write_seq_preserves_pml4
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A sequence of writes should never change pml4 since each individual write preserves it — but `fold_left` could have subtle interaction issues

### φ3: valid_core_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_core` is `external_body` — identical cores should get the same validity result, but this is unverified
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` — the core validity predicate is completely opaque and unverified. While functional consistency on equal inputs holds trivially for spec functions, the underlying predicate itself is an unauditable trust assumption used throughout the MMU spec.

### φ4: write_read_same_addr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing then reading the same address should return the written value — tests that `Map::insert` followed by `Map::index` is consistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from `Map::insert` then `Map::index` on the same key, which is a standard vstd axiom. Writing then reading the same address returning the written value is correct and expected.

### φ5: write_read_different_addr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing to one address should not affect reads from a different address — tests non-interference of memory writes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Map::insert` does not affect other keys — `Map::index` on a different key returns the original value. This is a standard map non-interference property and is correct.

