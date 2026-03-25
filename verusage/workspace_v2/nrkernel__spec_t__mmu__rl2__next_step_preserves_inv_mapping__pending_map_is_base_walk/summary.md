# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_mapping__pending_map_is_base_walk.rs`
**Date:** 2026-03-24T14:18:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One false positive: the equality between `MASK_L3_PG_ADDR_SPEC` and `MASK_ADDR_SPEC` is definitionally true — both are the same bitmask expression. This is correct x86 semantics where 4KB page addresses and directory addresses share the same physical address field layout.

## All Candidates

### φ1: mask_l3_pg_addr_equals_mask_addr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Both are `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)` — if they're equal, L3 page addresses use the same mask as directory addresses, meaning L3 pages have no additional address bits beyond the standard mask
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both `MASK_L3_PG_ADDR_SPEC` and `MASK_ADDR_SPEC` are defined as `bitmask_inc!(12usize, MAX_PHYADDR_WIDTH - 1)`. They are identical by definition. This is correct — L3 (4KB) pages use the same address mask as directory entries since both extract bits 12..MAX_PHYADDR_WIDTH-1. This is standard x86 page table semantics.

### φ2: pde_zero_entry_is_invalid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An all-zero PDE entry should be Invalid since P bit (bit 0) is 0 — tests that the entry-zero case correctly falls to Invalid rather than being misclassified

### φ3: pde_p_bit_required_for_directory
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Without the P bit set, a PDE should never be classified as Directory — tests that the `v & MASK_FLAG_P == MASK_FLAG_P` guard works correctly

### φ4: pde_p_bit_required_for_page
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Without the P bit set, a PDE should never be classified as Page — tests the same guard for the Page variant

### φ5: mask_l1_pg_addr_subset_of_mask_addr
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `MASK_L1_PG_ADDR` (bits 30..MAX_PHYADDR_WIDTH-1) should be a subset of `MASK_ADDR` (bits 12..MAX_PHYADDR_WIDTH-1) — if the masks don't nest, L1 page addresses could include bits outside the standard address mask

