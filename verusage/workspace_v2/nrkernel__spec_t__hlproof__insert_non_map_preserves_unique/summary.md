# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlproof/spec_t__hlproof__insert_non_map_preserves_unique.rs`
**Date:** 2026-03-24T13:20:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The `if_map_then_unique` / `inflight_maps_unique` invariant is intentionally scoped to prevent exact duplicate `ThreadState::Map` values. Physical frame overlap prevention is handled separately by `candidate_mapping_overlaps_inflight_pmem` (checked as a precondition in `insert_map_preserves_unique`). The vacuous truth for non-Map entries, empty maps, and state transitions replacing Map with non-Map are all correct by design.

## All Candidates

### φ1: insert_unmap_preserves_unique
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Inserting an Unmap that replaces an existing Map entry silently drops the uniqueness constraint for that Map — the old Map is removed and replaced by Unmap without checking if another thread had a duplicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Transitioning a thread from Map to Unmap is a normal state machine step — the thread is completing its map operation. The uniqueness invariant only needs to hold for active Map entries. Replacing a Map with Unmap correctly removes it from the set of entries that need uniqueness, and the prior state already ensured uniqueness.

### φ2: insert_idle_over_map_preserves_unique
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Overwriting a Map entry with Idle preserves uniqueness — tests that removing an in-flight map by going Idle doesn't break the invariant even if the map was the only thing preventing a duplicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Transitioning from Map to Idle is a cancellation or completion. If uniqueness held before, removing a Map entry can only make the remaining entries "more unique" — it cannot introduce duplicates. This is correct monotonic reasoning.

### φ3: if_map_then_unique_vacuous_for_non_map
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `if_map_then_unique` is vacuously true for non-Map entries — the uniqueness check provides no protection for Unmap or Idle thread states
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is by design — `if_map_then_unique` is specifically about Map entries. Non-Map entries (Unmap, Idle) don't participate in the physical frame uniqueness invariant because they don't represent active mapping operations that could cause physical frame aliasing.

### φ4: inflight_maps_unique_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The empty thread state trivially satisfies uniqueness — the universal quantifier over an empty domain is vacuously true
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty thread state having no uniqueness violations is trivially correct and expected. This is standard behavior for universally quantified invariants over empty domains.

### φ5: uniqueness_value_equality_not_frame_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two inflight maps with identical PTEs (same physical frame) but different vaddrs pass uniqueness because `if_map_then_unique` compares full `ThreadState` values including vaddr — allows double-mapping the same physical frame concurrently
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional — `inflight_maps_unique` prevents exact duplicate `ThreadState` values, while physical frame overlap is separately handled by `candidate_mapping_overlaps_inflight_pmem` (visible in other files). The `insert_map_preserves_unique` lemma requires `!candidate_mapping_overlaps_inflight_pmem` as a precondition, so the two checks work together to prevent both duplicate entries and overlapping physical frames.

