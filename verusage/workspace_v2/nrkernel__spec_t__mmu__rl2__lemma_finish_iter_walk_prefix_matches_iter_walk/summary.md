# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_finish_iter_walk_prefix_matches_iter_walk.rs`
**Date:** 2026-03-24T13:38:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `all_mb0_bits_are_zero` is an external_body predicate critical to PDE interpretation that is unverified. Two false positives: the L3/directory address mask equality is correct by construction, and the 4-step walk bound correctly matches x86-64 page table structure.

## True Positives (Spec Issues)

### all_mb0_bits_are_zero_external_body
- **Confidence:** medium
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` — it's a completely opaque predicate used in the critical `PDE::view` function to determine whether a page directory entry is valid or invalid. While this specific φ shows functional consistency (same inputs → same output), the underlying issue is that the predicate is unverified and could permit or reject entries incorrectly.

## All Candidates

### φ1: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — completely opaque, so its return value could be inconsistent for identical inputs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` — it's a completely opaque predicate used in the critical `PDE::view` function to determine whether a page directory entry is valid or invalid. While this specific φ shows functional consistency (same inputs → same output), the underlying issue is that the predicate is unverified and could permit or reject entries incorrectly.

### φ2: pde_view_invalid_when_p_bit_zero
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When the P (present) bit is 0, PDE view should return Invalid — tests that non-present entries are correctly classified regardless of other bits

### φ3: mask_l3_pg_addr_equals_mask_addr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MASK_L3_PG_ADDR` and `MASK_ADDR` are both `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)` — if they should differ (L3 pages vs directory addresses), their equality could mask a missing distinction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both `MASK_L3_PG_ADDR` and `MASK_ADDR` are defined as `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)` — this equality is correct by construction. L3 page addresses and directory addresses both use bits 12 through MAX_PHYADDR_WIDTH-1 on x86, as L3 is the leaf level with 4KB page granularity.

### φ4: walk_next_path_length_exceeds_4
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `walk_next` on a walk with path.len() >= 4 uses `arbitrary()` for the address — the walk continues with an unspecified address, producing a 5+ length path that exceeds the 4-layer page table

### φ5: iter_walk_max_4_steps
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `iter_walk` applies `walk_next` at most 4 times — the result should have at most 4 path entries matching the 4-level x86 page table
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `iter_walk` structurally applies `walk_next` at most 4 times (the function is unrolled 4 times). Each call adds one path entry, so the maximum path length is 4. This correctly matches the x86-64 4-level page table walk and is a desirable property.

