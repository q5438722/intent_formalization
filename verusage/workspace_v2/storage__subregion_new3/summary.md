# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_subregion/subregion_new3.rs`
**Date:** 2026-03-24T15:16:27Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. They confirm correct properties of vstd's `subrange` axiom: length preservation, offset-based indexing, empty subranges for zero length, and consistent splitting of adjacent subregions. No spec gaps found.

## All Candidates

### φ1: subregion_view_length_matches
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The subregion view length should equal the requested length — if `subrange` produced a different length, all offset calculations within the subregion would be invalid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `get_subregion_view` uses `subrange(start, start + len)` which produces a sequence of length `len` by the vstd subrange axiom. Correct by definition.

### φ2: subregion_view_byte_offset
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Subregion byte at relative offset k should equal region byte at absolute offset start+k — if the mapping were wrong, reads/writes through the subregion would access incorrect physical addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `subrange(start, start + len)[k] == region.state[start + k]` by the vstd subrange indexing axiom. Correct by definition.

### φ3: views_differ_allows_writable_addr_change
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** When only one address is writable and only that address differs, the predicate should hold — tests that the writable address function correctly permits targeted mutations

### φ4: zero_length_subregion_view_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-length subregion should produce an empty view — if it had non-zero length, phantom bytes would appear in the subregion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `subrange(start, start + 0)` = `subrange(start, start)` produces an empty sequence of length 0. Correct by the subrange axiom.

### φ5: adjacent_subregions_cover_range
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A subregion starting at `start+len1` should align with the second half of a combined subregion — if subrange arithmetic were inconsistent, splitting a region into adjacent subregions would produce misaligned views
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both sides reduce to `region.state[start + len1 + k]` via the subrange indexing axiom. The left side: `subrange(start+len1, start+len1+len2)[k] = region.state[start+len1+k]`. The right side: `subrange(start, start+len1+len2)[len1+k] = region.state[start+len1+k]`. Correct by definition.

