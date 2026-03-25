# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__translation/spec_t__mmu__translation__lemma_bit_indices_less_512.rs`
**Date:** 2026-03-24T14:38:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 6
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The trivial `true` ensures, sub-page l3_bits being zero, disjoint bit ranges, lossless reconstruction, and high-bit independence are all correct properties that follow directly from the bitvector definitions of the extraction macros.

## All Candidates

### φ1: l0_l1_bits_independent
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Placeholder — l0 and l1 extract non-overlapping ranges so same l0 doesn't constrain l1; replaced with a real test below
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is literally `true`, making this a trivial tautology. It tests nothing about the spec — any program entails `true`.

### φ2: l0_bits_covers_9_bits
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When all 9 bits (39-47) are set, l0_bits should return 511 (2^9 - 1) — tests that the extraction range is exactly 9 bits wide, not wider or narrower

### φ3: l3_bits_zero_below_page_boundary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Addresses below 4096 (one page) should have zero l3_bits since bits 12-20 are all zero — tests that sub-page offsets don't pollute the L3 index
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** 4095 = 0xFFF has only bits 0-11 set. `l3_bits!` extracts bits 12-20, which are all zero for this input. Correct by bitvector arithmetic.

### φ4: bit_ranges_non_overlapping
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The four bit extraction ranges (L0: 39-47, L1: 30-38, L2: 21-29, L3: 12-20) must be disjoint — overlapping ranges would cause one level's index to be influenced by another level's bits
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The four bitmask ranges (39-47, 30-38, 21-29, 12-20) are adjacent non-overlapping 9-bit fields. Their AND being zero is correct by construction and verified by bitvector reasoning.

### φ5: all_index_bits_reconstruct_addr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Reassembling the four 9-bit indices (shifted back) should recover bits 12-47 of the original address — if not, the extraction macros are losing or corrupting address information used for page table indexing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The four macros extract disjoint 9-bit fields covering bits 12-47. Shifting them back and OR-ing reconstructs exactly those bits. This is correct by bitvector arithmetic — it confirms the macros are lossless for bits 12-47.

### φ6: bits_ignore_high_48_plus
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** l0_bits extracts bits 39-47 so should be unaffected by bits 48-63 — tests that non-canonical high bits don't influence the L0 page table index
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `l0_bits!` masks with `bitmask_inc!(39, 47)` which only selects bits 39-47. Bits 48-63 are zeroed by the AND, so truncating the input to bits 0-47 first makes no difference. Correct by bitvector reasoning.

