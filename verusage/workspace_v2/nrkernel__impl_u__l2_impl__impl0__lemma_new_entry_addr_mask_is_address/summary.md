# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l2_impl/impl_u__l2_impl__impl0__lemma_new_entry_addr_mask_is_address.rs`
**Date:** 2026-03-24T13:06:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `MAX_PHYADDR_WIDTH` as `external_body` with a weaker axiom than its concrete value creates a specification gap between the implementation (52) and verification assumptions (32..=52). One false positive: dirty/access bits being clear in new entries is correct and desirable behavior verified by bitvector reasoning.

## True Positives (Spec Issues)

### max_phyaddr_width_external_body
- **Confidence:** medium
- **Reasoning:** `MAX_PHYADDR_WIDTH` is declared `external_body` with implementation value 52, but the axiom only guarantees `32 <= MAX_PHYADDR_WIDTH <= 52`. The verifier treats it as opaque, so proofs cannot rely on the concrete value — this is an intentional design choice to support variable physical address widths, but it creates a gap where the spec is weaker than the implementation.

## All Candidates

### φ1: new_entry_present_bit_set
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Present bit is always set in the constructed entry — tests that OR with MASK_FLAG_P correctly sets bit 0

### φ2: new_entry_dirty_access_clear
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** New entries always have dirty (bit 6) and access (bit 5) cleared — tests that none of the flag ORs accidentally set these hardware-managed bits
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property — newly constructed page table entries should have dirty (bit 6) and access (bit 5) cleared since these are hardware-managed bits set by the CPU on access/write. The proof is fully verified via `by (bit_vector)` assertions.

### φ3: addr_mask_roundtrips_through_flags
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Address bits survive ORing all flags — tests that flag bits don't overlap with MASK_ADDR address field

### φ4: l1_page_pat_bit_clear
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** L1 page entries have PAT bit (bit 12) clear — relies on MASK_L1_PG_ADDR zeroing bit 12, so the address can't set it

### φ5: max_phyaddr_width_external_body
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52 but the axiom only constrains it to 32..=52 — the verifier treats it as opaque, creating a gap between spec and implementation
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is declared `external_body` with implementation value 52, but the axiom only guarantees `32 <= MAX_PHYADDR_WIDTH <= 52`. The verifier treats it as opaque, so proofs cannot rely on the concrete value — this is an intentional design choice to support variable physical address widths, but it creates a gap where the spec is weaker than the implementation.

