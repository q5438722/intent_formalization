# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_wf/original.rs`
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
- **Why flagged:** Aligning down should never produce a value larger than the original input

### φ2: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize should always be aligned to b; if not, the spec is wrong

### φ3: aligned_identity_broken → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a is already aligned to b, align_to_usize should be the identity; deviation indicates inconsistency

### φ4: align_to_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 should always return the input unchanged since every integer is 1-aligned

### φ5: aligned_zero_size_vacuous → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned with size 0 causes division by zero in modulo; if this verifies, the spec admits undefined behavior

