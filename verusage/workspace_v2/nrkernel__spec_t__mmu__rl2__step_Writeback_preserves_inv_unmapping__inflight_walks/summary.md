# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__step_Writeback_preserves_inv_unmapping__inflight_walks.rs`
**Date:** 2026-03-24T14:52:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

Two false positives: `update_range` prefix replacement and `combine` NX propagation are correct by their definitions. One true positive: `lemma_write_seq` is an `external_body` proof trusting pml4 preservation and domain monotonicity without verification — a closeable gap since the property follows by straightforward induction over the write sequence.

## True Positives (Spec Issues)

### lemma_write_seq_pml4_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` body — it asserts `write_seq(writes).pml4 == self.pml4` without proof. While the property is correct (each `write` preserves `pml4` by construction), it's trusted rather than verified. This is an unverified trust assumption that could be proved by induction on `writes.len()`.

## All Candidates

### φ1: memop_valid_size_excludes_3
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `valid_op_size` only allows 1, 2, 4, 8 — a 3-byte operation is rejected; tests that the size whitelist doesn't accidentally include non-power-of-2 sizes which x86 doesn't support as atomic memory operations

### φ2: update_range_at_zero_replaces_prefix
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `update_range` at index 0 should replace the prefix and preserve the suffix — tests that the subrange+concat construction correctly handles the boundary; incorrect slicing would corrupt physical memory updates
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `update_range(s, 0, new)` expands to `s.subrange(0,0) + new + s.subrange(new.len(), s.len())`, which is `Seq::empty() + new + suffix`. The length, prefix, and suffix properties follow directly from sequence concatenation axioms. Correct by construction.

### φ3: walk_result_vaddr_invalid_returns_input
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** For invalid walks, `vaddr()` should return the original queried address — tests that the Invalid variant faithfully records the input; if it returned a different address, error handling would target the wrong virtual address

### φ4: combine_flags_disable_execute_propagates
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `combine` uses OR for `disable_execute` — a single NX-flagged directory entry in the walk chain forces the final mapping to be non-executable; if this propagation failed, code execution from non-executable pages would be allowed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` defines `disable_execute: self.disable_execute || other.disable_execute`. If `f1.disable_execute` is true, the OR is true regardless of `f2`. This is the intended security-conservative behavior for NX propagation through page table walks.

### φ5: lemma_write_seq_pml4_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` — pml4 preservation through arbitrary write sequences is asserted without proof; if `write_seq` could modify pml4, the page table root would shift silently
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` body — it asserts `write_seq(writes).pml4 == self.pml4` without proof. While the property is correct (each `write` preserves `pml4` by construction), it's trusted rather than verified. This is an unverified trust assumption that could be proved by induction on `writes.len()`.

