# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__create_intersect/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: intersect_always_empty → `create_intersect`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean every intersection is vacuously empty regardless of inputs, indicating a trivial spec

### φ2: self_intersect_empty → `create_intersect`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Self-intersection must equal self; if it equals empty the spec loses all set information at boundary

### φ3: result_leaks_outside_other → `create_intersect`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An intersection element not in the second operand violates the fundamental intersection contract

### φ4: intersect_equals_union → `create_intersect`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Conflating intersection with union would make the result a superset instead of a subset of operands

### φ5: intersect_no_filtering → `create_intersect`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean intersection never filters anything from self, making it equivalent to identity on the first operand

