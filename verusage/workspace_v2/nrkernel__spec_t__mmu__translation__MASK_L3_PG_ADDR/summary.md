# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__translation/spec_t__mmu__translation__MASK_L3_PG_ADDR.rs`
**Date:** 2026-03-24T14:37:57Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One false positive: the equality between `MASK_L3_PG_ADDR_SPEC` and its own definition is tautological. The shared mask between L3 pages and directory entries is correct x86-64 behavior.

## All Candidates

### φ1: mask_l3_pg_addr_low_12_bits_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits 0-11 should be clear for 4KB page address extraction — if any low bits leak, physical addresses would include sub-page offsets

### φ2: mask_l3_pg_addr_bit_12_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 12 must be the lowest set bit — if clear, the mask would start too high and lose physical address bits

### φ3: mask_l3_pg_addr_high_bits_clear
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits at or above position 52 should be zero — if set, extracted addresses would exceed maximum physical address width

### φ4: mask_l3_equals_mask_addr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MASK_L3_PG_ADDR_SPEC` and `MASK_ADDR_SPEC` share the same definition `bitmask_inc!(12, MAX_PHYADDR_WIDTH-1)` — if they're identical, L3 page address extraction uses the same mask as directory address extraction, which is correct for 4KB pages but means no additional validation distinguishes page vs directory entries at the mask level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `MASK_L3_PG_ADDR_SPEC` is literally defined as `bitmask_inc!(12usize, MAX_PHYADDR_WIDTH - 1)`, so the ensures is a tautology. Both L3 page entries and directory entries extract physical addresses from bits 12..MAX_PHYADDR_WIDTH-1, which is correct x86-64 semantics — 4KB pages and directory entries use the same address mask.

### φ5: mask_l3_pg_addr_nonzero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For any valid MAX_PHYADDR_WIDTH, the mask must be nonzero — a zero mask would make 4KB page address extraction impossible

