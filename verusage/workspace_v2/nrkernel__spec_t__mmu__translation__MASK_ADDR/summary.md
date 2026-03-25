# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__translation/spec_t__mmu__translation__MASK_ADDR.rs`
**Date:** 2026-03-24T14:36:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One true positive confirming the previously identified MAX_PHYADDR_WIDTH axiom gap: the `external_body` constant is 52 but the axiom only constrains it to [32, 52], leaving proofs that depend on the exact width unverifiable.

## True Positives (Spec Issues)

### max_phyaddr_width_axiom_allows_any_value_32_52
- **Confidence:** high
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52, but the axiom only constrains it to `[32, 52]`. This is the known MAX_PHYADDR_WIDTH gap — proofs that depend on the exact value 52 cannot be verified, and `MASK_ADDR_SPEC` varies depending on which value in the range is assumed.

## All Candidates

### φ1: max_phyaddr_width_axiom_allows_any_value_32_52
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52, but the axiom only constrains it to `[32, 52]` — any proof using MASK_ADDR_SPEC could silently depend on the weaker range, producing masks with fewer valid bits than the actual hardware
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52, but the axiom only constrains it to `[32, 52]`. This is the known MAX_PHYADDR_WIDTH gap — proofs that depend on the exact value 52 cannot be verified, and `MASK_ADDR_SPEC` varies depending on which value in the range is assumed.

### φ2: mask_addr_low_12_bits_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MASK_ADDR_SPEC = bitmask_inc!(12, MAX_PHYADDR_WIDTH-1)` should have bits 0-11 clear — tests that the mask doesn't accidentally include low offset bits which would corrupt physical address extraction

### φ3: mask_addr_bit_12_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bit 12 should be the lowest set bit in MASK_ADDR — if clear, the mask would start at bit 13+ and lose one bit of physical address resolution

### φ4: mask_addr_high_bits_clear_above_52
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Bits at or above MAX_PHYADDR_WIDTH should be zero in MASK_ADDR — if set, the mask would extract bits beyond the physical address width, producing invalid physical addresses

### φ5: mask_addr_not_zero_for_any_valid_width
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For any MAX_PHYADDR_WIDTH in [32, 52], the mask should be nonzero — a zero mask would mean no physical address bits are extracted, breaking all address translation

