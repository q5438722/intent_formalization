# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__translation__MASK_ADDR/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mask_is_zero → `MASK_ADDR`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A zero mask would erase all address bits, making address translation completely non-functional

### φ2: lower_twelve_bits_leak → `MASK_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Lower 12 bits represent the page offset and must be zero in the address mask; nonzero would corrupt page-aligned addresses

### φ3: mask_all_ones → `MASK_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An all-ones mask would pass through non-physical high bits, defeating the purpose of physical address masking

### φ4: bit12_not_set → `MASK_ADDR`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 12 is the lowest physical-frame bit and must be included in the mask; excluding it would truncate valid frame addresses

### φ5: exceeds_max_phyaddr → `MASK_ADDR`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bits 52–63 exceed the maximum 52-bit physical address width; if set, the mask would accept non-existent physical addresses

