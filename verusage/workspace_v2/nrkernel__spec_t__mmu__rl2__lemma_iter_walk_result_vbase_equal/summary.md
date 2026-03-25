# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal.rs`
**Date:** 2026-03-24T13:47:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `walk_next` is entirely `external_body` making the core walk mechanism unverified, and both auxiliary lemmas (`aux1` path equality, `aux2` vaddr idempotency) are `external_body` trust assumptions without proof. Two false positives: eager flag computation beyond path length is safe in spec mode, and page-aligned invalid vaddr reporting is correct x86 semantics.

## True Positives (Spec Issues)

### walk_next_external_body_unconstrained
- **Confidence:** high
- **Reasoning:** `walk_next` is `external_body` with `unimplemented!()` — the entire page table walk stepping function is completely opaque and unverified. Since `iter_walk` depends on it, all walk results inherit this trust gap. This is a fundamental unverified assumption in the MMU specification.

### lemma_aux1_external_body_path_equality
- **Confidence:** high
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux1` is `external_body` with `unimplemented!()` — the path equality property (walking from an aligned result vaddr produces the same path) is asserted without proof. This is an unverified trust assumption in the walk idempotency chain.

### lemma_aux2_external_body_vaddr_idempotent
- **Confidence:** high
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux2` is `external_body` with `unimplemented!()` — the vaddr idempotency property is asserted without proof. Combined with aux1, the entire `lemma_iter_walk_result_vbase_equal` is built on two unverified assumptions.

## All Candidates

### φ1: walk_next_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `walk_next` is `external_body` with `unimplemented!()` — completely opaque, so identical inputs could produce different outputs
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `walk_next` is `external_body` with `unimplemented!()` — the entire page table walk stepping function is completely opaque and unverified. Since `iter_walk` depends on it, all walk results inherit this trust gap. This is a fundamental unverified assumption in the MMU specification.

### φ2: lemma_aux1_external_body_path_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_iter_walk_result_vbase_equal_aux1` is `external_body` — the claimed path equality between walking from the result vaddr and the original walk is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux1` is `external_body` with `unimplemented!()` — the path equality property (walking from an aligned result vaddr produces the same path) is asserted without proof. This is an unverified trust assumption in the walk idempotency chain.

### φ3: lemma_aux2_external_body_vaddr_idempotent
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_iter_walk_result_vbase_equal_aux2` is `external_body` — the vaddr idempotency property is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_iter_walk_result_vbase_equal_aux2` is `external_body` with `unimplemented!()` — the vaddr idempotency property is asserted without proof. Combined with aux1, the entire `lemma_iter_walk_result_vbase_equal` is built on two unverified assumptions.

### φ4: flags_eagerly_computed_beyond_path
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Walk::flags` eagerly indexes `path[2]` and `path[3]` even when path.len() == 2 — the computation of `flags2` and `flags3` reads beyond the path but the final result selects `flags1`, relying on Verus not triggering the out-of-bounds access in spec mode
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In Verus spec mode, out-of-bounds indexing on `Seq` returns an arbitrary value but doesn't cause unsoundness — the result is never used because the `if path.len() == 2` branch selects `flags1`. The final flags value is correct for the actual path length.

### φ5: walk_result_invalid_aligns_to_page
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An invalid walk result aligns the vaddr to PAGE_SIZE — for already page-aligned vaddrs this is identity, but for non-aligned vaddrs the reported Invalid address differs from the actual queried address
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For a page-aligned vaddr, `align_to_usize(vaddr, PAGE_SIZE) == vaddr` is trivially correct. The alignment behavior for non-aligned addresses is intentional — invalid walk results report at page granularity, which matches x86 page fault semantics.

