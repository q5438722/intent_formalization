# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__container_tree__container_tree_check_is_ancestor/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_ancestor → `container_tree_check_is_ancestor`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Any node with smaller depth being an ancestor would collapse the tree into a total order, violating branching structure.

### φ2: never_ancestor → `container_tree_check_is_ancestor`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** No ancestry ever holding would mean the tree has no parent-child paths, contradicting tree well-formedness.

### φ3: depth_one_diff_always_ancestor → `container_tree_check_is_ancestor`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A depth difference of exactly 1 does not imply ancestry — siblings' parents are different nodes at the same depth.

### φ4: subtree_uppertree_disagree → `container_tree_check_is_ancestor`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The two ensures clauses both equal ret; if subtree_set and uppertree_seq could disagree the spec would be inconsistent.

### φ5: ancestor_independent_of_a_ptr → `container_tree_check_is_ancestor`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Ancestry being independent of which candidate node is tested would mean all shallower nodes are equivalent, destroying tree branching.

