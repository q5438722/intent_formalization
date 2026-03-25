# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__translation__MASK_L3_PG_ADDR/original.rs`
**Date:** 2026-03-24T22:23:24Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `high_bits_above_phyaddr_set` is a false positive. The bitmask macro `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)` sets bits from position 12 through MAX_PHYADDR_WIDTH−1 inclusive, which are all strictly below MAX_PHYADDR_WIDTH. For every valid value of MAX_PHYADDR_WIDTH in [32, 52], shifting the mask right by MAX_PHYADDR_WIDTH produces 0, directly contradicting the `!= 0` claim. The spec is correct: no bits at or above the physical address width are set, so the mask cannot produce invalid physical addresses from this cause.

## All Candidates

### φ1: mask_is_zero → `MASK_L3_PG_ADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The mask selecting physical page address bits should never be zero; that would mean no address bits are extractable.

### φ2: lower_twelve_bits_nonzero → `MASK_L3_PG_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits 0–11 are the page offset; a page-address mask with any of these bits set would corrupt the page offset during address translation.

### φ3: bit12_not_set → `MASK_L3_PG_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bit 12 is the lowest bit of an L3 page address; if the mask excludes it, the translated physical address loses its least significant address bit.

### φ4: high_bits_above_phyaddr_set → `MASK_L3_PG_ADDR`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Bits at or above MAX_PHYADDR_WIDTH are beyond the physical address space; setting them would produce invalid physical addresses.
- **Verdict:** FALSE_POSITIVE (high)

### φ5: mask_all_ones → `MASK_L3_PG_ADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An all-ones mask means no masking at all, which would pass through page-offset bits and reserved high bits, violating page-table address extraction semantics.

