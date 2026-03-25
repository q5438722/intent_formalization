# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_preserves_inv.rs`
**Date:** 2026-03-24T14:13:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Nonneg/nonpos mutual exclusion follows from bitvector arithmetic (`x & 1` is 0 or 1). Walk vaddr preservation and completeness are correct by construction (all branches set these fields directly). The view self-consistency test is a tautology that doesn't probe the external_body gap.

## All Candidates

### φ1: nonneg_nonpos_not_both
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A write cannot simultaneously flip P bit from 0→1 and from 1→0 — tests mutual exclusion of nonneg and nonpos classifications
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_nonneg_write` requires `self.read(addr) & 1 == 0` while `is_nonpos_write` requires `self.read(addr) & 1 == 1`. These are mutually exclusive — `x & 1` is either 0 or 1. This is a correct bitvector property, not a spec gap.

### φ2: pt_walk_preserves_vaddr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The walk should record the original vaddr — tests that all branches of `pt_walk` correctly propagate the input vaddr to the Walk struct
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Every branch of `pt_walk` constructs `Walk { vaddr, ... }` using the input parameter directly. This is correct by construction — the vaddr field is always set to the input.

### φ3: pt_walk_always_complete
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pt_walk` constructs Walk with `complete: true` in all branches — if all branches indeed set this, the walk is never partial, meaning incomplete walks are impossible in rl3
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** All branches of `pt_walk` construct `Walk { ..., complete: true }`. This is correct by construction for rl3's page table walk, which always terminates (it's a pure spec function with bounded depth). Incomplete walks are an rl2 concept for in-progress hardware walks.

### φ4: ptmem_view_external_body_functional
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — even self-consistency of repeated indexing is technically trusted; the entire PTE interpretation is opaque
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `P ==> P` and `x == x` are tautologies that hold regardless of whether `view` is `external_body`. This proves nothing about the external_body gap — any expression equals itself.

### φ5: align_to_usize_leq_input
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `align_to_usize(a, b) = a - (a % b)` should always be ≤ a — tests that the alignment function rounds down correctly; used in Walk::result for vbase computation

