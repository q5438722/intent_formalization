# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__translation/spec_t__mmu__translation__MASK_L1_PG_ADDR.rs`
**Date:** 2026-03-24T14:37:02Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mask_l1_pg_addr_low_30_bits_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MASK_L1_PG_ADDR_SPEC = bitmask_inc!(30, MAX_PHYADDR_WIDTH-1)` should have bits 0-29 clear — if any low bits leak through, 1GB page physical address extraction would include sub-page offset bits

### φ2: mask_l1_pg_addr_bit_30_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 30 should be the lowest set bit — if clear, the mask would miss the lowest valid address bit for 1GB pages, shifting physical addresses by 1GB

### φ3: mask_l1_pg_addr_subset_of_52bit_range
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** No bits at or above position 52 should be set — if the mask extends beyond the maximum physical address width, extracted addresses would include invalid high bits

### φ4: mask_l1_pg_addr_narrower_than_mask_addr
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bits 12-29 should be zero in L1 mask but set in MASK_ADDR — tests that the 1GB page mask correctly excludes the 18 bits that MASK_ADDR includes for 4KB-aligned directory addresses; if these bits leaked in, 1GB page addresses would be corrupted

### φ5: mask_l1_width_depends_on_phyaddr_width
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** For any valid MAX_PHYADDR_WIDTH ≥ 32, the mask must be nonzero and include at least bit 30 — a zero or too-small mask would make 1GB page translation impossible

