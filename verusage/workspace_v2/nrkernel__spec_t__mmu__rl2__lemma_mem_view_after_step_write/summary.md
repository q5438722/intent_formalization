# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_mem_view_after_step_write.rs`
**Date:** 2026-03-24T13:49:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives: `PTMem::view` is an external_body function critical for reasoning about PTE mappings, `lemma_write_seq_push` is an unverified inductive step used in the core write correctness proof, and `can_flip_polarity` has commented-out pending map/unmap conditions that allow unsafe polarity transitions with outstanding operations.

## True Positives (Spec Issues)

### ptmem_view_external_body
- **Confidence:** high
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the function that interprets raw page table memory as a `Map<usize, PTE>` is completely opaque. This is a critical trust assumption since the PTE view is used in step_WriteNonneg/step_WriteNonpos postconditions to reason about mapping changes.

### lemma_write_seq_push_external_body
- **Confidence:** high
- **Reasoning:** `lemma_write_seq_push` is `external_body` with `unimplemented!()` — the inductive decomposition of `write_seq` over a pushed element is trusted without proof. This lemma is directly used in `lemma_mem_view_after_step_write` to reason about writer memory after store buffer operations, making it a key unverified link in the proof chain.

### can_flip_polarity_commented_out_conditions
- **Confidence:** high
- **Reasoning:** The commented-out conditions `self.hist.pending_maps

## All Candidates

### φ1: ptmem_view_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — the mapping from raw page table memory to PTE map is completely opaque, so identical memories could theoretically produce different PTE views
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the function that interprets raw page table memory as a `Map<usize, PTE>` is completely opaque. This is a critical trust assumption since the PTE view is used in step_WriteNonneg/step_WriteNonpos postconditions to reason about mapping changes.

### φ2: lemma_write_seq_push_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_push` is `external_body` — the inductive step for write_seq decomposition is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq_push` is `external_body` with `unimplemented!()` — the inductive decomposition of `write_seq` over a pushed element is trusted without proof. This lemma is directly used in `lemma_mem_view_after_step_write` to reason about writer memory after store buffer operations, making it a key unverified link in the proof chain.

### φ3: wf_ptmem_range_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `wf_ptmem_range` is `external_body` — the well-formedness condition on page table memory ranges is completely opaque and unverified

### φ4: can_flip_polarity_commented_out_conditions
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `can_flip_polarity` has commented-out conditions for `pending_maps` and `pending_unmaps` — polarity can flip even with outstanding pending maps/unmaps, potentially allowing unsafe interleaving of mapping and unmapping operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The commented-out conditions `self.hist.pending_maps

### φ5: writer_sbuf_empty_after_core_change
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When a different core writes, the precondition `!tso.is_empty() ==> core == writes.core` combined with `writer_sbuf_subset_tso_writes` forces the old writer's sbuf to be empty — tests that the invariant correctly prevents concurrent writers

