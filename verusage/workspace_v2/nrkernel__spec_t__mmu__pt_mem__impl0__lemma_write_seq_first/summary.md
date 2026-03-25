# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__impl0__lemma_write_seq_first.rs`
**Date:** 2026-03-24T13:28:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. `PTMem` is a low-level spec abstraction where last-write-wins and unbounded domain expansion are correct by design. Address validity and write conflict detection are enforced at higher layers in the refinement stack.

## All Candidates

### φ1: write_seq_single_equals_write
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single-element `write_seq` should equal a single `write` — tests that `fold_left` correctly reduces to the base case

### φ2: write_seq_first_commutes_with_disjoint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Disjoint writes should both be visible — tests that `write_seq` correctly applies all writes to distinct addresses

### φ3: write_seq_last_wins
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When the same address is written twice, the last write wins — no conflict detection or error for duplicate writes to the same page table entry
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Last-write-wins is standard map semantics and the correct behavior for sequential page table writes. This models the physical reality that writing to the same memory location overwrites the previous value. Conflict detection is handled at higher abstraction layers.

### φ4: write_seq_preserves_unwritten
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `write_seq` should not modify addresses not in the write sequence — tests that map insert only affects the target key

### φ5: write_seq_unbounded_expansion
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `write_seq` can create entries at arbitrary addresses not in the original domain — no bounds checking on whether the address is within allocated page table regions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem` is a low-level memory abstraction using `Map<usize, usize>`. Bounds checking on valid page table addresses is enforced at higher layers (directory invariants, allocated region tracking). The memory model itself correctly allows any write to expand the domain, as this is how map-based memory models work.

