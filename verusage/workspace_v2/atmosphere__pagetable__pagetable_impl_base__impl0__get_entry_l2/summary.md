# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/pagetable/pagetable__pagetable_impl_base__impl0__get_entry_l2.rs`
**Date:** 2026-03-24T07:19:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The L2 intra-table disjointness property confirms that the active (non-commented) `disjoint_l2` spec correctly handles the same-table case via `pi == pj && l2i != l2j`. The PCID/IOID exclusivity is the intended design — x86-64 page tables use either PCID or IOID, never both, and the XOR spec correctly enforces this.

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: page_entry_to_map_entry_drops_user_bit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** page_entry_to_map_entry silently drops the user/supervisor bit — kernel-only and user-accessible pages become indistinguishable in MapEntry

### φ3: page_entry_to_map_entry_drops_present_and_ps
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** page_entry_to_map_entry produces identical MapEntry regardless of present bit — non-present entries yield valid-looking MapEntries indistinguishable from present ones

### φ4: disjoint_l2_commented_out_intra_table
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** disjoint_l2 covers intra-table uniqueness via the pi==pj && l2i!=l2j case — verifying this confirms the active (non-commented) spec correctly replaced the commented-out version
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The active `disjoint_l2` spec includes the case `pi == pj && l2i != l2j`, which directly covers intra-table uniqueness. This PHI confirms the refactored spec correctly subsumes the commented-out version — a desirable property.

### φ5: pcid_ioid_exclusive_or
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** pcid_ioid_wf uses `!=` (XOR) meaning exactly one must be Some — a page table can never have both PCID and IOID simultaneously, but the spec allows neither to be set (both None satisfies != trivially as false != false is false... actually both None gives false != false = false, so this is not satisfied — the XOR is correct)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pcid_ioid_wf` is defined as `self.pcid.is_Some() != self.ioid.is_Some()`, which is strict XOR — exactly one must be Some. Given `pcid.is_Some()`, `ioid.is_None()` follows directly. This is the intended mutual exclusivity between PCID and IOID for a page table.

