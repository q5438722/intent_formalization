# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_writenonneg_path_addrs_match.rs`
**Date:** 2026-03-24T14:00:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

All five candidates are true positives. `lemma_pt_walk` and `lemma_write_seq` are both `external_body` with `unimplemented!()`, trusting five distinct properties without proof: walk completeness, path-entry-to-memory correspondence, domain monotonicity of write sequences, no-Directory-last invariant, and intermediate entries being present directories. These collectively form the unverified foundation of the MMU page table walk specification.

## True Positives (Spec Issues)

### lemma_pt_walk_external_body_completeness
- **Confidence:** high
- **Reasoning:** `lemma_pt_walk` is `external_body` with `unimplemented!()` — walk completeness and the path length bound (1..=4) are asserted without proof. These are fundamental structural properties of the page table walk that the entire MMU verification depends on.

### lemma_pt_walk_path_entries_match_reads
- **Confidence:** high
- **Reasoning:** The correspondence between path entries and PDE views at their addresses is asserted by the external_body `lemma_pt_walk` without proof. This is the critical link between the walk's recorded path and actual memory contents — if incorrect, the walk could claim entries that don't match what's stored in memory.

### lemma_write_seq_domain_monotonic
- **Confidence:** high
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — domain monotonicity is trusted without proof. While this property is mathematically correct (each `Map::insert` only adds or overwrites keys), the inductive argument over `fold_left` is not verified.

### lemma_pt_walk_no_directory_last
- **Confidence:** high
- **Reasoning:** The assertion that the last path entry is never Directory comes from the external_body `lemma_pt_walk`. This is critical because `Walk::result` falls through to `arbitrary()` for Directory last entries, so if this invariant is wrong, walk results would be completely undefined.

### lemma_pt_walk_intermediate_present
- **Confidence:** high
- **Reasoning:** All intermediate entries being Directory with P bit set is asserted by the external_body `lemma_pt_walk` without proof. This structural invariant ensures the walk follows valid directory chains — if violated, the walk could traverse non-present or non-directory entries.

## All Candidates

### φ1: lemma_pt_walk_external_body_completeness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` is `external_body` — walk completeness and path length bounds are trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_pt_walk` is `external_body` with `unimplemented!()` — walk completeness and the path length bound (1..=4) are asserted without proof. These are fundamental structural properties of the page table walk that the entire MMU verification depends on.

### φ2: lemma_pt_walk_path_entries_match_reads
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts path entries equal PDE views at corresponding addresses — this correspondence is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The correspondence between path entries and PDE views at their addresses is asserted by the external_body `lemma_pt_walk` without proof. This is the critical link between the walk's recorded path and actual memory contents — if incorrect, the walk could claim entries that don't match what's stored in memory.

### φ3: lemma_write_seq_domain_monotonic
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` — domain monotonicity (writes never remove keys) is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` — domain monotonicity is trusted without proof. While this property is mathematically correct (each `Map::insert` only adds or overwrites keys), the inductive argument over `fold_left` is not verified.

### φ4: lemma_pt_walk_no_directory_last
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts the last path entry is never Directory — trusted without proof; if false, `Walk::result` would fall through to `arbitrary()`
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The assertion that the last path entry is never Directory comes from the external_body `lemma_pt_walk`. This is critical because `Walk::result` falls through to `arbitrary()` for Directory last entries, so if this invariant is wrong, walk results would be completely undefined.

### φ5: lemma_pt_walk_intermediate_present
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts all intermediate entries are Directory with P bit set — trusted without proof; if false, the walk could skip non-present entries
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** All intermediate entries being Directory with P bit set is asserted by the external_body `lemma_pt_walk` without proof. This structural invariant ensures the walk follows valid directory chains — if violated, the walk could traverse non-present or non-directory entries.

