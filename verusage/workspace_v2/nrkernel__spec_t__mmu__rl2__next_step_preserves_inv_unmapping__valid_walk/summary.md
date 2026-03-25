# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_unmapping__valid_walk.rs`
**Date:** 2026-03-24T14:34:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_pt_walk_complete_always_true` relies on the `external_body` `lemma_pt_walk` to prove completeness, which is an unverified trust assumption (though the property happens to be trivially true from the open spec). Three false positives: valid walk from `is_base_pt_walk` follows from pattern match semantics, vaddr boundedness is a direct conjunct, and vaddr preservation is by construction.

## True Positives (Spec Issues)

### lemma_pt_walk_complete_always_true
- **Confidence:** medium
- **Reasoning:** While `pt_walk` does set `complete: true` in all visible branches, the proof relies on the `external_body` `lemma_pt_walk` rather than unfolding the open spec. The `complete` field being always true is provable from the open definition alone, but the fact that this proof goes through the external_body path confirms an unverified trust assumption is being used.

## All Candidates

### φ1: view_only_contains_valid_walks
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `is_base_pt_walk` destructures result as `WalkResult::Valid { vbase, pte }` — tests that the pattern match implies the result variant; if the match semantics are looser than expected, non-Valid results could sneak into the view domain
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_base_pt_walk` uses `self.pt_walk(vaddr).result() matches WalkResult::Valid { vbase, pte }` which is a pattern match that only succeeds when the result is the `Valid` variant. This directly implies `result() is Valid` by Verus match semantics.

### φ2: view_pte_equals_walk_pte
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After revealing the opaque `view`, the PTE at `va` should equal the walk result's PTE — tests that the `Map::new` value function correctly extracts the PTE from the walk result

### φ3: lemma_pt_walk_complete_always_true
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` is `external_body` and asserts `.complete` for all walks — but `pt_walk` sets `complete: true` in all branches anyway; the external_body is redundant here but still an unverified trust assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** While `pt_walk` does set `complete: true` in all visible branches, the proof relies on the `external_body` `lemma_pt_walk` rather than unfolding the open spec. The `complete` field being always true is provable from the open definition alone, but the fact that this proof goes through the external_body path confirms an unverified trust assumption is being used.

### φ4: is_base_pt_walk_vaddr_bounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `is_base_pt_walk` includes `vaddr < MAX_BASE` — tests that the bound is actually enforced; if MAX_BASE were incorrectly large or the check were missing, walks for non-canonical addresses could pollute the view
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_base_pt_walk` explicitly includes `vaddr < MAX_BASE` as its first conjunct. The ensures directly follows from unfolding this definition.

### φ5: walk_vaddr_preserved_in_result
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pt_walk` constructs `Walk { vaddr, ... }` in every branch — tests that the walk records the queried virtual address faithfully; if a different vaddr were stored, `is_base_pt_walk`'s `vbase == vaddr` check would compare against the wrong address
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pt_walk` constructs `Walk { vaddr, ... }` in every branch of the match, using the input `vaddr` parameter. The Walk struct stores it directly, so `.vaddr == va` by construction.

