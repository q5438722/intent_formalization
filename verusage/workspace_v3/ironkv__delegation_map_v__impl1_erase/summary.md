# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl1_erase/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: erase_preserves_length → `erase`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Erase with start < end must shrink the sequence; unchanged length means elements were not removed.

### φ2: erase_all_leaves_nonempty → `erase`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Erasing the entire range (start=0, end=len) on a non-empty vec must yield an empty result.

### φ3: erase_set_unchanged → `erase`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** With no-duplicate keys, erased elements must leave the set; an unchanged set means nothing was truly removed.

### φ4: erase_empties_with_prefix → `erase`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When start > 0, prefix elements must survive; an empty result means erase destroyed unerased data.

### φ5: erase_single_noop → `erase`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Erasing one element must change the sequence; a no-op erase means the element was silently kept.

