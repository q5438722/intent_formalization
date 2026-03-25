# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__step_WriteNonpos_preserves_inv_unmapping__inflight_walks.rs`
**Date:** 2026-03-24T15:00:31Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One candidate evaluated: `view_opaque_base_walk_only` is a false positive. The property simply confirms that `PTMem::view`'s opaque definition correctly maps base-aligned valid walks to map entries, which is exactly how `is_base_pt_walk` and the `Map::new` constructor are defined.

## All Candidates

### φ1: lemma_pt_walk_external_body_completeness
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_pt_walk` is `external_body` with `unimplemented!()` — it asserts every walk is complete with path length 1..4, but this is trusted without proof; if the walk could diverge or produce empty paths, downstream reasoning would be unsound

### φ2: lemma_pt_walk_valid_implies_p_bit
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_pt_walk` trusts that valid walk results have P-bit set on the last entry — this is asserted via `external_body` without verifying the `pt_walk` definition actually guarantees it

### φ3: lemma_pt_walk_intermediate_are_directories
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_pt_walk` trusts that all intermediate walk entries are Directory variants with P-bit set — this structural invariant over the walk path is asserted without proof

### φ4: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — if it returns true for arbitrary entries (including ones with reserved bits set), the PDE view would parse invalid entries as valid Directory/Page variants

### φ5: view_opaque_base_walk_only
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is opaque and defined via `is_base_pt_walk` — revealing it should show that base-aligned valid walks produce map entries; if the opaque barrier hides incorrect filtering logic, the view map could silently drop valid mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is defined as `Map::new(|va| self.is_base_pt_walk(va), ...)` where `is_base_pt_walk` checks exactly `va < MAX_BASE && pt_walk(va).result() matches Valid { vbase, .. } && vbase == va`. The preconditions match this definition precisely, so revealing the opaque spec correctly yields `contains_key(va)`. This is the intended behavior.

