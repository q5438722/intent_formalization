# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_mapping__valid_not_pending_is_not_in_sbuf.rs`
**Date:** 2026-03-24T14:22:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives all targeting the `external_body` `lemma_pt_walk`: Valid result P bit, path-layer correspondence, intermediate present-and-Directory invariant, and last-not-Directory invariant are all asserted without proof. These collectively form the unverified structural foundation of the page table walk specification. One false positive: the opaque view domain correctly maps to `is_base_pt_walk` by `Map::new` axioms after reveal.

## True Positives (Spec Issues)

### lemma_pt_walk_valid_result_p_bit
- **Confidence:** high
- **Reasoning:** This property is asserted by the `external_body` `lemma_pt_walk` without proof. The link between a Valid walk result and the P bit being set in the last raw memory entry is a critical trust assumption — it ensures walk validity corresponds to actual present entries in memory.

### lemma_pt_walk_path_layer_matches_index
- **Confidence:** high
- **Reasoning:** This is asserted by the `external_body` `lemma_pt_walk` without proof. The correspondence between path index `i` and PDE layer `Ghost(i as nat)` is critical — if the layer assignment is wrong, the PDE view function would extract addresses using the wrong mask for that level.

### lemma_pt_walk_intermediate_present_and_directory
- **Confidence:** high
- **Reasoning:** Asserted by `external_body` `lemma_pt_walk` without proof. Intermediate entries being Directory with P bit set ensures the walk follows valid directory chains. If wrong, the walk could traverse non-present entries or treat leaf pages as directories.

### lemma_pt_walk_last_not_directory
- **Confidence:** high
- **Reasoning:** Asserted by `external_body` `lemma_pt_walk` without proof. This invariant is critical because `Walk::result` returns `arbitrary()` when the last entry is not Page — if a Directory could appear last, walk results would be unconstrained.

## All Candidates

### φ1: lemma_pt_walk_valid_result_p_bit
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts that Valid walk results have P bit set in the last entry — this is trusted without proof and links walk result validity to raw memory content
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This property is asserted by the `external_body` `lemma_pt_walk` without proof. The link between a Valid walk result and the P bit being set in the last raw memory entry is a critical trust assumption — it ensures walk validity corresponds to actual present entries in memory.

### φ2: lemma_pt_walk_path_layer_matches_index
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts each path entry equals PDE view at the corresponding layer — trusted without proof; the layer index `i as nat` must match the actual nesting depth
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is asserted by the `external_body` `lemma_pt_walk` without proof. The correspondence between path index `i` and PDE layer `Ghost(i as nat)` is critical — if the layer assignment is wrong, the PDE view function would extract addresses using the wrong mask for that level.

### φ3: lemma_pt_walk_intermediate_present_and_directory
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts intermediate entries are Directory with P bit set — trusted without proof; if wrong, walk could traverse non-present or leaf entries as directories
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Asserted by `external_body` `lemma_pt_walk` without proof. Intermediate entries being Directory with P bit set ensures the walk follows valid directory chains. If wrong, the walk could traverse non-present entries or treat leaf pages as directories.

### φ4: view_opaque_domain_equals_base_walks
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After revealing the opaque `view`, domain membership should imply `is_base_pt_walk` — tests that the `Map::new` domain predicate correctly gates the view
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is defined as `Map::new(|va| self.is_base_pt_walk(va), ...)`. After `reveal`, `contains_key(va)` directly implies the domain predicate `is_base_pt_walk(va)` by `Map::new` axioms. This is correct by construction.

### φ5: lemma_pt_walk_last_not_directory
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_pt_walk` asserts the last path entry is never Directory — trusted without proof; if violated, `Walk::result` would fall to `arbitrary()` for the non-Page case
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Asserted by `external_body` `lemma_pt_walk` without proof. This invariant is critical because `Walk::result` returns `arbitrary()` when the last entry is not Page — if a Directory could appear last, walk results would be unconstrained.

