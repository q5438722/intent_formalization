# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/storage__subregion_new3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: zero_len_subregion_matches_full → `new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A zero-length subregion should have empty state, not the full region's state

### φ2: offset_subregion_equals_full_region → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A subregion starting at a non-zero offset should never equal the full region's state

### φ3: different_starts_same_view → `new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subregions at distinct offsets should not be universally extensionally equal

### φ4: identical_views_reported_differing → `new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Comparing a region view with itself must always satisfy the differ-only-where-allowed predicate

### φ5: boundary_subregion_has_zero_len → `new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A subregion with positive length at the region boundary must not report zero-length state

