# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__translation__MASK_L2_PG_ADDR/original.rs`
**Date:** 2026-03-24T22:21:22Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `mask_all_ones` is a false positive. The claim that `MASK_L2_PG_ADDR_SPEC == 0xFFFF_FFFF_FFFF_FFFF` is arithmetically impossible: the `bitmask_inc!` macro's final left-shift by 21 always zeros out bits 0–20, and the axiom-constrained `MAX_PHYADDR_WIDTH` (32–52) means bits above index `MAX_PHYADDR_WIDTH - 1` are also zero. The mask correctly isolates bits [21, MAX_PHYADDR_WIDTH − 1], which is the expected physical-address field for an L2 page-table entry. No spec issue exists.

## All Candidates

### φ1: mask_is_zero → `MASK_L2_PG_ADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A zero mask would fail to extract any L2 page address bits, making all L2 translations collapse to address zero

### φ2: low_bits_set → `MASK_L2_PG_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits 0-20 must be clear in an L2 (2MiB) page address mask; setting them would include page-offset bits in the physical address extraction

### φ3: mask_all_ones → `MASK_L2_PG_ADDR`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An all-ones mask would pass through all 64 bits, defeating the purpose of masking the L2 page-table entry to its physical address field
- **Verdict:** FALSE_POSITIVE (high)

### φ4: bit20_set → `MASK_L2_PG_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 20 is the topmost offset bit for a 2MiB page; including it in the address mask would corrupt extracted physical addresses by one bit position

### φ5: bit21_clear → `MASK_L2_PG_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bit 21 is the lowest address bit for an L2 page; if clear, the mask would fail to capture the least-significant physical address bit for 2MiB pages

