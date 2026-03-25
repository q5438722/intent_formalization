# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv.rs`
**Date:** 2026-03-24T14:14:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Entry size constants are correct by definition and match x86-64 page table geometry. The `is_supervisor = !flag_US` inversion is intentional x86 semantics. The `update_range` at index 0 correctly reduces to prefix replacement via empty subrange concatenation.

## All Candidates

### φ1: write_commutative_different_addrs
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Writes to different addresses should commute — if they don't, the write_seq fold_left order matters even for disjoint addresses, which would be a spec gap

### φ2: write_overwrites_same_addr
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A second write to the same address should fully overwrite the first — tests that Map::insert idempotency holds for PTMem, which is foundational for store buffer reasoning

### φ3: l3_entry_size_equals_page_size
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Entry sizes should form a geometric progression based on 512 entries per level — if the constants are wrong, walk results would produce incorrect vbase/size mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `L3_ENTRY_SIZE = PAGE_SIZE = 4096`, `L2_ENTRY_SIZE = 512 * L3_ENTRY_SIZE = 512 * 4096`, `L1_ENTRY_SIZE = 512 * L2_ENTRY_SIZE = 512 * 512 * 4096`. These follow directly from the constant definitions. This is correct x86-64 page table structure.

### φ4: from_bits_supervisor_inverted
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `from_bits` sets `is_supervisor = !flag_US` — a US=true (user-accessible) entry has is_supervisor=false; tests that the inversion is intentional and not a bug
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `from_bits` defines `is_supervisor = !flag_US`, so `US=true` → `is_supervisor=false` and `US=false` → `is_supervisor=true`. This is correct x86 semantics — the US bit grants user-mode access, so its absence means supervisor-only. The inversion is intentional.

### φ5: update_range_at_zero_equals_prefix_replace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Updating at index 0 should replace the prefix — tests that `subrange(0, 0)` correctly produces an empty sequence and doesn't corrupt the result
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `update_range(s, 0, new) = s.subrange(0, 0) + new + s.subrange(new.len(), s.len())`. `s.subrange(0, 0)` is the empty sequence, so concatenation gives `new + s.subrange(new.len(), s.len())`. This is correct by vstd Seq axioms.

