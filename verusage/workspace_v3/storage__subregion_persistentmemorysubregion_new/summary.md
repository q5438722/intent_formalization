# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/storage__subregion_persistentmemorysubregion_new/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: start_always_zero → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean the subregion ignores the caller-supplied start offset, always anchoring at 0

### φ2: len_always_zero → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean the subregion always has zero length, discarding the requested size

### φ3: view_len_mismatches_len → `new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean the materialized view has a different byte count than the logical length, breaking internal consistency between len() and view()

### φ4: start_len_interchangeable → `new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean start and len are always equal, collapsing two independent parameters into one

### φ5: subregion_covers_full_region → `new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean every subregion spans the entire backing region, making sub-region slicing meaningless

