# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__definitions_u__impl2__entry_base/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: entry_base_idx_zero_not_base → `ArchExec::entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** entry_base with idx=0 must equal base; if this verifies, the spec is inconsistent

### φ2: entry_base_exceeds_double_max → `ArchExec::entry_base`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Under valid preconditions the result should be bounded by 2*MAX_BASE; provability means the spec allows unbounded results

### φ3: entry_base_collision_different_indices → `entry_base (Arch spec)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Distinct indices must map to distinct entry bases; provability means the address space has collisions

### φ4: inv_allows_zero_entry_size → `inv`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** inv requires 0 < entry_size(i); if this verifies, the invariant fails to exclude degenerate layers

### φ5: layer_size_not_multiplicative → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** inv enforces the multiplicative layer-size relation; provability means the structural invariant is vacuous or contradictory

