# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__impl0__lemma_write_seq_push.rs`
**Date:** 2026-03-24T13:30:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `lemma_fold_left_push` and `lemma_write_seq` are both `external_body` lemmas with `unimplemented!()` bodies — fundamental properties trusted without proof. Two false positives: the push decomposition and pml4 preservation through push are downstream consequences of the same two external_body lemmas, not independent findings.

## True Positives (Spec Issues)

### lemma_fold_left_push_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_fold_left_push` is `external_body` with `unimplemented!()` — this is a fundamental property of `fold_left` that should be provable by induction but is instead trusted. It's an unverified trust assumption in the proof chain.

### lemma_write_seq_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — pml4 preservation and domain monotonicity through `write_seq` are trusted without proof. These properties are likely correct but remain unverified trust assumptions.

## All Candidates

### φ1: lemma_fold_left_push_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_fold_left_push` is `external_body` with `unimplemented!()` — the fundamental fold-left-push equivalence is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_fold_left_push` is `external_body` with `unimplemented!()` — this is a fundamental property of `fold_left` that should be provable by induction but is instead trusted. It's an unverified trust assumption in the proof chain.

### φ2: lemma_write_seq_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` with `unimplemented!()` — pml4 preservation and domain monotonicity are trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — pml4 preservation and domain monotonicity through `write_seq` are trusted without proof. These properties are likely correct but remain unverified trust assumptions.

### φ3: write_seq_push_depends_on_two_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_push` depends on both `lemma_fold_left_push` and `lemma_write_seq` (both external_body) — the push decomposition rests on two unverified trust assumptions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a downstream consequence of the two external_body lemmas already captured by φ1 and φ2. The push decomposition itself is correctly proven given those assumptions — flagging the transitive dependency is a duplicate, not a new finding.

### φ4: write_seq_push_simplifies_to_write
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single-element write_seq should equal a direct write — tests fold_left base case consistency

### φ5: write_seq_push_pml4_preserved
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pml4 preservation through push relies on `lemma_write_seq_push` which transitively depends on two external_body lemmas
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same as φ3 — this is a downstream consequence of the already-flagged external_body lemmas. The pml4 preservation through push is correctly derived from `lemma_write_seq_push`, which itself depends on the two already-captured trust assumptions.

