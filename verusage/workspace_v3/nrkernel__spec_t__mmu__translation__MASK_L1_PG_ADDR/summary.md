# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__translation__MASK_L1_PG_ADDR/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mask_is_zero → `MASK_L1_PG_ADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The L1 page address mask must be non-zero to select any physical address bits

### φ2: mask_low_bits_set → `MASK_L1_PG_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** L1 pages are 1GB-aligned so bits [0,29] must be zero in the mask; non-zero low bits would corrupt page alignment

### φ3: mask_exceeds_maxphyaddr → `MASK_L1_PG_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits at or above MAX_PHYADDR_WIDTH must be clear; set bits there would reference non-existent physical address space

### φ4: mask_all_ones → `MASK_L1_PG_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The mask must select only the L1 page-frame bits, not every bit; equaling all-ones would be an overly broad mask

### φ5: mask_bit30_unset → `MASK_L1_PG_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bit 30 is the lowest bit of a 1GB-aligned page frame address and must be included in the mask; its absence would lose address information

