# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_invalid_walk_is_invalid_in_writer_mem.rs`
**Date:** 2026-03-24T13:46:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `valid_core` is an `external_body` predicate with no specification, representing an unverified trust assumption for core validity. Four false positives: the range ordering follows directly from `memories_disjoint`, zero-size range acceptance is standard interval semantics, `writer_mem`/`core_mem` equivalence is correct by construction, and empty store buffer identity is expected TSO behavior.

## True Positives (Spec Issues)

### valid_core_external_body_unconstrained
- **Confidence:** medium
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` — a completely opaque predicate used to determine core validity throughout the MMU specification. Its semantics are entirely unverified and unauditable, representing a trust assumption in the system.

## All Candidates

### φ1: valid_core_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_core` is `external_body` with `unimplemented!()` — completely opaque, so identical cores could theoretically get different validity results
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` — a completely opaque predicate used to determine core validity throughout the MMU specification. Its semantics are entirely unverified and unauditable, representing a trust assumption in the system.

### φ2: memories_disjoint_implies_range_ordering
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `memories_disjoint` uses strict ordering to separate physical memory and page table memory ranges — tests that the gap between ranges is at least 1 byte, which may be too weak (no alignment requirement)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This directly follows from the definition of `memories_disjoint()` which explicitly requires `range_mem.1 < range_ptmem.0 < range_ptmem.1`. The strict ordering is correct and desirable — it ensures non-overlapping, non-empty ranges.

### φ3: in_ptmem_range_zero_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `in_ptmem_range` with size=0 is satisfied for any address within or at the boundary — a zero-size region at the end of ptmem range passes the check, which may allow degenerate empty regions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Zero-size range checks passing is standard behavior for interval containment predicates. The predicate `addr + 0 <= range_ptmem.1` is trivially satisfied when `addr <= range_ptmem.1`. This is not a spec gap — callers that need non-empty ranges add their own `size > 0` preconditions.

### φ4: core_mem_writer_sbuf_relationship
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `writer_mem` should be equivalent to `core_mem(writes.core)` — tests that the inline helper correctly aliases the writer's view of page table memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `writer_mem` is defined as an inline alias for `core_mem(writes.core)`. Their equivalence is correct by construction and desirable — it confirms the helper correctly provides the writer core's memory view.

### φ5: pt_walk_agrees_with_core_mem_walk
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When a core's store buffer is empty, its view of page table memory should equal the global view — tests that `write_seq` of an empty sequence is identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When the store buffer is empty, `write_seq` of an empty sequence is identity via `fold_left` on an empty `Seq`. So `core_mem(core) == pt_mem`, making their walks identical. This is correct and expected TSO semantics.

