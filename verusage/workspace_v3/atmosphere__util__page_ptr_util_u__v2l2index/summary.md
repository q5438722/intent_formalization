# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__v2l2index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: l2index_always_zero_4k → `v2l2index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every 4K-valid address maps to L2 index 0, the function cannot distinguish any L2 page-table entries for 4K pages

### φ2: l2index_never_reaches_max → `v2l2index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The ensures clause promises ret <= 0x1ff; if 0x1ff is unreachable, the upper bound is too loose and the full 9-bit index space is not usable

### φ3: l2index_determines_2m_addr → `v2l2index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two 2M-valid addresses can share an L2 index yet differ in higher bits (L3/L4); if entailed, the spec conflates distinct address ranges

### φ4: l2index_always_zero_2m → `v2l2index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** 2M alignment zeros bits 0–20 but bits 21–29 are free; if the index were always 0, the L2 table could only map one 2M region per L3 entry

### φ5: l2index_upper_half_unreachable → `v2l2index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the top bit of the 9-bit index is never set, only 256 of 512 L2 entries are addressable, halving usable virtual address space per L3 slot

