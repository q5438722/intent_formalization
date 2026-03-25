# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__erase/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: erase_removes_below_lo → `erase`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Keys strictly below `lo` are outside [lo, hi) and must be preserved, not erased

### φ2: empty_range_removes_key → `erase`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When lo >= hi the range [lo, hi) is empty so no key should ever be removed

### φ3: erase_introduces_new_key → `erase`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Erase should never introduce a key absent from the original map

### φ4: erase_changes_preserved_value → `erase`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A key outside the erase range that survives must retain its original mapped value

### φ5: erase_removes_at_hi_boundary → `erase`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The key at `hi` is at the exclusive upper bound and must not be erased

