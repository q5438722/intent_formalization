# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_single_write_crash_effect_on_pm_regions_view.rs`
**Date:** 2026-03-24T15:13:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: both `external_body` lemmas — crash-state uniqueness with no outstanding writes and the two-outcome crash guarantee for aligned chunk writes — are trusted without proof. These are the foundational crash consistency assumptions the multi-region proof builds on. Three false positives confirm correct definitional properties: region write isolation, write dispatch correctness, and flush distribution across regions.

## True Positives (Spec Issues)

### single_write_crash_region_external_body
- **Confidence:** high
- **Reasoning:** `lemma_single_write_crash_effect_on_pm_region_view` is `external_body` with `unimplemented!()`. It asserts that after one aligned chunk write, only two crash states are possible (old committed or fully flushed). This is the core crash atomicity guarantee and is entirely trusted without verification.

### no_writes_crash_committed_external_body
- **Confidence:** high
- **Reasoning:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts crash-state uniqueness when no writes are outstanding. The multi-region lemma's correctness depends on this unverified assumption.

## All Candidates

### φ1: single_write_crash_region_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_single_write_crash_effect_on_pm_region_view` is `external_body` — the two-outcome crash guarantee for a single aligned chunk write is trusted without proof; if more crash states were possible, the crash consistency argument would be unsound
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_single_write_crash_effect_on_pm_region_view` is `external_body` with `unimplemented!()`. It asserts that after one aligned chunk write, only two crash states are possible (old committed or fully flushed). This is the core crash atomicity guarantee and is entirely trusted without verification.

### φ2: no_writes_crash_committed_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` — the uniqueness of crash states with no outstanding writes is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts crash-state uniqueness when no writes are outstanding. The multi-region lemma's correctness depends on this unverified assumption.

### φ3: regions_write_only_modifies_target
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing to one region should not affect other regions — if the map-based write leaked across region indices, cross-region data corruption would go undetected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `write` map closure returns `pre_view` unchanged when `pos != index`. For `other != index`, the region is preserved. Correct by definition.

### φ4: regions_write_target_matches_region_write
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The multi-region write at the target index should equal the single-region write — if the map dispatched incorrectly, the written region would have wrong state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The map closure returns `pre_view.write(addr, bytes)` when `pos == index`. At index `index`, this equals `views[index].write(addr, bytes)`. Correct by definition.

### φ5: regions_flush_distributes
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Flushing all regions should equal flushing each individually — if the outer map didn't distribute correctly, some regions could flush with wrong state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `flush` map applies `pm.flush()` to each region. At index `i`, `views.flush()[i] == views[i].flush()` by the vstd map indexing axiom. Correct by definition.

