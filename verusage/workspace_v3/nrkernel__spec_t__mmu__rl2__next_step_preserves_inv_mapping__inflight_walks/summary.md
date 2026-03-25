# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_mapping__inflight_walks/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** align_to_usize rounds down, so the result must never exceed the input

### φ2: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce an aligned address; if the result is not aligned the function is broken

### φ3: aligned_successor_also_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If addr is aligned to size>1 then addr+1 cannot also be aligned, so this being provable would mean aligned is trivially true

### φ4: align_drops_too_much → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The rounding loss must be strictly less than b; losing b or more means we skipped an entire alignment boundary

### φ5: inflight_walks_preconditions_vacuous → `next_step_preserves_inv_mapping__inflight_walks`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the combined preconditions are unsatisfiable the invariant preservation theorem is vacuously true and proves nothing

