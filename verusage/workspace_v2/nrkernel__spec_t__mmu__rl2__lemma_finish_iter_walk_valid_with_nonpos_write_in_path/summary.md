# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_finish_iter_walk_valid_with_nonpos_write_in_path.rs`
**Date:** 2026-03-24T13:44:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. `finish_iter_walk` correctly completes within 4 levels by construction, the `all_mb0_bits_are_zero` tautology proves nothing about the external_body gap, and the nonpos-write invalidation property is an exact duplicate of an existing verified lemma confirming correct page table semantics.

## All Candidates

### φ1: walk_next_preserves_vaddr
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `walk_next` should preserve the virtual address being walked — if it changes vaddr, the walk diverges from its original translation target

### φ2: walk_next_increments_path_len
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Each `walk_next` step should append exactly one entry — growing by more or fewer would break the 4-level page table traversal

### φ3: finish_iter_walk_always_complete
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Starting from an empty incomplete walk, `finish_iter_walk` applies up to 4 steps — the walk should always complete since every non-Directory PDE sets complete=true and after 4 levels there are no more Directory options
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `finish_iter_walk` applies `walk_next` up to 4 times starting from path.len()==0. Each `walk_next` step appends one entry and sets `complete: !(entry is Directory)`. After 4 steps (path.len()==4), `walk_next` uses `arbitrary()` which cannot produce a Directory variant in x86's 4-level structure. The walk always completes within 4 levels — this is correct and desirable.

### φ4: all_mb0_bits_are_zero_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` — the predicate is entirely opaque with no body constraining its value
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This φ is a tautology (`P ==> P`), which is trivially true for any proposition regardless of whether `all_mb0_bits_are_zero` is external_body. It does not actually demonstrate any unsoundness or spec gap — it proves nothing about the predicate's behavior.

### φ5: nonpos_write_invalidates_only_at_idx
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Duplicate of existing lemma — if Verus verifies this, it confirms the lemma but also that clearing the P bit at any ancestor in the walk path always invalidates, with no mechanism for a sibling entry to "rescue" the walk
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is an exact duplicate of the already-proven lemma `lemma_finish_iter_walk_valid_with_nonpos_write_in_path` in the source. The property is correct: clearing the P bit at an entry along a valid walk's path invalidates the walk, because subsequent steps read the now-non-present entry and produce Invalid. This is desirable behavior.

