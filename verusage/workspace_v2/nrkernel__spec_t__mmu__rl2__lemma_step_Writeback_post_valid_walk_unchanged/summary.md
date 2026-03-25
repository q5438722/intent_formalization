# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_Writeback_post_valid_walk_unchanged.rs`
**Date:** 2026-03-24T13:56:56Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. Empty store buffer identity, writer_mem alias consistency, TSO store buffer forwarding divergence, structural equality implying functional equality, and single-element containment are all correct and expected properties following from open spec definitions and standard vstd semantics.

## All Candidates

### φ1: core_mem_empty_sbuf_equals_pt_mem
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When a core's store buffer is empty, its view should equal the global page table memory — tests that `write_seq` of empty seq is identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write_seq(seq![])` is `seq![].fold_left(pt_mem, f)` which returns `pt_mem` by the base case of `fold_left`. This is correct and expected — an empty store buffer means no pending writes.

### φ2: writer_mem_equals_core_mem_writer
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `writer_mem` is an inline alias for `core_mem(writes.core)` — tests that the alias is consistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `writer_mem` is marked `#[verifier(inline)]` and defined as `self.core_mem(self.writes.core)`. This equality is correct by construction — it's a definitional alias.

### φ3: core_mem_diverges_with_sbuf
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A single pending store buffer entry makes the core's view diverge from global memory — tests that TSO correctly models store buffer forwarding
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This correctly models TSO store buffer forwarding — `write_seq(seq![(addr, value)])` applies one write, making `core_mem.read(addr) == value` while `pt_mem.read(addr)` retains the old value. This is the intended TSO semantics.

### φ4: valid_core_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_core` is `external_body` — even structurally equal constants could produce different validity results for the same core
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `c1

### φ5: contains_fst_single_element
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A single-element sequence should contain its first element — tests that `contains_fst` existential witness works for the trivial case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A single-element sequence containing its first element is trivially correct — the witness `i=0` satisfies the existential in `contains_fst`. This is a basic correctness property of the predicate.

