# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__translation/spec_t__mmu__translation__MASK_L2_PG_ADDR.rs`
**Date:** 2026-03-24T14:37:34Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mask_l2_pg_addr_low_21_bits_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits 0-20 should be clear in the 2MB page mask — if any leak through, physical address extraction for 2MB pages would include sub-page offset bits

### φ2: mask_l2_pg_addr_bit_21_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 21 should be the lowest set bit — if clear, the mask starts at bit 22+ and loses one bit of 2MB page physical address resolution

### φ3: mask_l2_pg_addr_high_bits_clear
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** No bits at or above position 52 should be set — if the mask extends beyond max physical address width, extracted addresses would include invalid high bits

### φ4: mask_l2_excludes_bits_12_to_20
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bits 12-20 are in MASK_ADDR (4KB) but should not be in MASK_L2_PG_ADDR (2MB) — if present, 2MB page addresses would extract sub-2MB offset bits as part of the physical base

### φ5: mask_l2_pg_addr_nonzero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For any MAX_PHYADDR_WIDTH in [32, 52], the L2 mask must be nonzero — a zero mask would make 2MB page translation impossible

