# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__empty/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: empty_contains_zero → `empty`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The empty commit mask should not contain element 0; if entailed, the spec contradicts emptiness.

### φ2: empty_contains_boundary_511 → `empty`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The empty mask should not contain the maximum valid bit index 511; if entailed, the ensures is over-permissive.

### φ3: empty_is_nonempty → `empty`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An empty set must have no members; if an element exists, the postcondition is inconsistent.

### φ4: empty_equals_singleton → `empty`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The empty set should never be extensionally equal to a singleton set; if entailed, the spec is unsound.

### φ5: empty_contains_negative → `empty`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Negative indices are outside the valid bit range; if the empty mask contains −1, the view abstraction is broken.

