# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__defs__MAX_PHYADDR/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ret_is_zero → `MAX_PHYADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the return value is zero, there is no usable physical address space, indicating a degenerate spec.

### φ2: ret_below_32bit_mask → `MAX_PHYADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the max physical address is strictly below a 32-bit mask, the width axiom lower bound of 32 is violated.

### φ3: ret_exceeds_52bit_range → `MAX_PHYADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the mask reaches or exceeds 2^52, it contradicts the upper bound on MAX_PHYADDR_WIDTH and could allow out-of-range physical addresses.

### φ4: ret_is_usize_max → `MAX_PHYADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the max physical address equals usize::MAX, the mask imposes no effective upper bound, defeating address-space isolation.

### φ5: ret_is_even → `MAX_PHYADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A bitmask of the form (1<<w)-1 with w>=1 is always odd; an even result would indicate an off-by-one or shift error in the spec.

