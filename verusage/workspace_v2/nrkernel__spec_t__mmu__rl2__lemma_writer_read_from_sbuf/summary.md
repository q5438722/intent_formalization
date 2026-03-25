# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_writer_read_from_sbuf.rs`
**Date:** 2026-03-24T14:11:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: `lemma_write_seq_read` is an external_body lemma trusting the fold_left read-after-unique-write property without proof, and `lemma_writer_read_from_sbuf` transitively depends on it for store buffer consistency. One false positive: pml4 alignment follows directly from the `wf` definition.

## True Positives (Spec Issues)

### lemma_write_seq_read_external_body
- **Confidence:** high
- **Reasoning:** `lemma_write_seq_read` is `external_body` with `unimplemented!()` — the property that a unique-address write in a `fold_left` sequence produces the expected value requires an inductive proof that is not verified. This is used by `lemma_writer_read_from_sbuf` to establish store buffer read consistency.

### writer_read_from_sbuf_chains_external
- **Confidence:** medium
- **Reasoning:** `lemma_writer_read_from_sbuf` is a verified proof, but its body directly calls `lemma_write_seq_read` which is `external_body`. The store buffer read consistency guarantee is a transitive trust assumption — only as strong as the unverified underlying lemma.

## All Candidates

### φ1: lemma_write_seq_read_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_read` is `external_body` — the property that writing a unique address into a fold_left sequence produces the expected value is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq_read` is `external_body` with `unimplemented!()` — the property that a unique-address write in a `fold_left` sequence produces the expected value requires an inductive proof that is not verified. This is used by `lemma_writer_read_from_sbuf` to establish store buffer read consistency.

### φ2: wf_ptmem_range_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `wf_ptmem_range` is `external_body` — the well-formedness predicate for page table memory range is entirely opaque and could depend on any state field

### φ3: writer_read_from_sbuf_chains_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The writer-reads-from-sbuf lemma depends on `lemma_write_seq_read` (external_body) — the store buffer read consistency guarantee chains through an unverified lemma
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_writer_read_from_sbuf` is a verified proof, but its body directly calls `lemma_write_seq_read` which is `external_body`. The store buffer read consistency guarantee is a transitive trust assumption — only as strong as the unverified underlying lemma.

### φ4: wf_implies_pml4_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Well-formedness requires pml4 alignment to 4096 — tests that the alignment constraint is correctly propagated; if too weak, misaligned pml4 could pass wf
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `wf` explicitly includes `aligned(self.pt_mem.pml4 as nat, 4096)` as a conjunct. This follows directly from the definition — it's a correct and expected property, not a spec gap.

### φ5: wf_walks_sbuf_domain_match
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `wf` requires both `walks` and `sbuf` domains equal `valid_core` — tests that walk entries always have corresponding sbuf entries

