# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_subregion/subregion_persistentmemorysubregion_new.rs`
**Date:** 2026-03-24T15:18:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. They confirm correct properties of vstd's subrange axioms and Verus integer casting: subrange length preservation, u64↔nat roundtrip, offset-based indexing, full-range identity, and partition consistency. The `PersistentMemorySubregion::new` constructor and `get_subregion_view` are correctly specified.

## All Candidates

### φ1: subregion_view_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The subregion view length should equal the requested length — if subrange produced a different length, all offset-based reasoning within the subregion would be broken
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `subrange(start, start + len)` produces a sequence of length `len` by the vstd subrange axiom. Correct by definition.

### φ2: subregion_start_cast_roundtrip
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The u64→nat→u64 roundtrip should be lossless — if the cast dropped high bits, the stored start_ field would not match the spec-level start()
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `u64` values fit in `nat`, and `nat` values that originated from `u64` fit back in `u64`. The roundtrip `(start as nat) as u64 == start` is a correct property of Verus integer casting for values in `[0, u64::MAX]`.

### φ3: subregion_view_byte_matches_region
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Subregion byte at relative offset k should equal the region byte at absolute offset start+k — if the indexing were misaligned, reads through the subregion would access wrong physical bytes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `subrange(start, start+len)[k] == state[start + k]` by the vstd subrange indexing axiom. Correct by definition.

### φ4: full_region_subregion_equals_region
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A subregion spanning the entire region should equal the original — if it didn't, creating a full-coverage subregion would lose or alter data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `state.subrange(0, state.len())` is extensionally equal to `state` by the vstd subrange axiom. The proof uses `=~=` to establish this. Correct.

### φ5: adjacent_subregions_partition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The first portion of a combined subregion should match a subregion of just that portion — if they disagreed, splitting a subregion into two adjacent pieces would produce inconsistent views
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both sides reduce to `pm@.state[start + k]` via the subrange indexing axiom. For `k < len1`, `subrange(start, start+len1+len2)[k] == state[start+k]` and `subrange(start, start+len1)[k] == state[start+k]`. Correct.

