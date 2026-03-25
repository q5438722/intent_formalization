# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_pt_walk_result_vbase_equal.rs`
**Date:** 2026-03-24T13:52:18Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `walk_next` is an opaque external_body stepping function, `lemma_iter_walk_equals_pt_walk` bridges the opaque walk to the specified walk without proof, and `lemma_iter_walk_result_vbase_equal` asserts walk idempotency without verification. Two false positives: the result vaddr ≤ original and walk idempotency are correct and desirable page table properties, even though their proofs depend on the unverified lemma chain.

## True Positives (Spec Issues)

### walk_next_external_body_unconstrained
- **Confidence:** high
- **Reasoning:** `walk_next` is `external_body` with `unimplemented!()` — the core page table walk stepping function is completely opaque. Since `iter_walk` builds on it and `lemma_iter_walk_equals_pt_walk` bridges it to `pt_walk`, this is a foundational unverified trust assumption.

### iter_walk_equals_pt_walk_external_body
- **Confidence:** high
- **Reasoning:** `lemma_iter_walk_equals_pt_walk` is `external_body` — the equivalence between the opaque `walk_next`-based `iter_walk` and the fully specified `pt_walk` is asserted without proof. This is a critical bridge lemma; if incorrect, all reasoning that flows through `iter_walk` would be unsound.

### iter_walk_result_vbase_equal_external_body
- **Confidence:** high
- **Reasoning:** `lemma_iter_walk_result_vbase_equal` is `external_body` — walk path and vaddr idempotency are entirely unverified. This lemma is directly used by `lemma_pt_walk_result_vbase_equal` to derive key properties about walk stability.

## All Candidates

### φ1: walk_next_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `walk_next` is `external_body` with `unimplemented!()` — the entire walk stepping function is opaque, so identical walks could produce different results
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `walk_next` is `external_body` with `unimplemented!()` — the core page table walk stepping function is completely opaque. Since `iter_walk` builds on it and `lemma_iter_walk_equals_pt_walk` bridges it to `pt_walk`, this is a foundational unverified trust assumption.

### φ2: iter_walk_equals_pt_walk_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_iter_walk_equals_pt_walk` is `external_body` — the equivalence between iterative and direct walk is trusted without proof, yet `iter_walk` uses the opaque `walk_next`
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_iter_walk_equals_pt_walk` is `external_body` — the equivalence between the opaque `walk_next`-based `iter_walk` and the fully specified `pt_walk` is asserted without proof. This is a critical bridge lemma; if incorrect, all reasoning that flows through `iter_walk` would be unsound.

### φ3: iter_walk_result_vbase_equal_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_iter_walk_result_vbase_equal` is `external_body` — walk idempotency (re-walking from the result vaddr produces the same path) is entirely unverified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_iter_walk_result_vbase_equal` is `external_body` — walk path and vaddr idempotency are entirely unverified. This lemma is directly used by `lemma_pt_walk_result_vbase_equal` to derive key properties about walk stability.

### φ4: pt_walk_result_vaddr_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The walk result vaddr is always ≤ the original vaddr (due to alignment rounding down) — but this depends on the chain of external_body lemmas; if alignment doesn't round down, this could be violated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The walk result vaddr is produced by `align_to_usize(vaddr, size)` which computes `sub(vaddr, vaddr % size)`, which is always ≤ vaddr. While the proof chain goes through external_body lemmas, the property itself is mathematically correct and desirable — alignment always rounds down.

### φ5: pt_walk_result_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Walk result idempotency (re-walking from the aligned result vaddr gives the same result) depends on external_body lemmas — if false, the page table would give different translations for the same virtual page
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Walk result idempotency is a correct and desirable property — re-translating from the page-aligned base address of a translation should yield the same translation. This is fundamental to page table semantics: all addresses within a page must map to the same frame.

