# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_unmapping__pt_walk_valid_in_post_unchanged.rs`
**Date:** 2026-03-24T14:10:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `all_mb0_bits_are_zero` is an external_body predicate that gates PDE validity — its behavior is entirely unverified, so the classification of present entries as Directory/Page/Invalid depends on an opaque trust assumption. Three false positives: update_range length preservation follows from arithmetic, valid_op_size exclusion of 3 is trivially correct, and page alignment of base walk addresses is intentional x86 semantics.

## True Positives (Spec Issues)

### all_mb0_bits_are_zero_gates_pde_validity
- **Confidence:** medium
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` and gates `PDE::view` — when it returns false, present entries (P bit set) are treated as Invalid. Since the predicate is completely opaque, Verus cannot verify that it correctly checks the must-be-zero bits for each layer, potentially misclassifying valid or invalid entries.

## All Candidates

### φ1: max_base_exceeds_usize
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MAX_BASE = 512^3 * 4096 * 512 = 256TB` which exceeds `usize::MAX` on 64-bit — if this verifies, ALL usize vaddrs are in range and the `vaddr < MAX_BASE` check in `is_base_pt_walk` is vacuous

### φ2: update_range_length_preserved
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `update_range` concatenates three subranges — tests that the total length is preserved, which is correct but foundational for memory operation semantics
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `update_range` is `s.subrange(0, idx) + new + s.subrange(idx + new.len(), s.len())`. The lengths are `idx + new.len() + (s.len() - idx - new.len()) = s.len()`. This is correct by basic arithmetic on sequence concatenation.

### φ3: valid_op_size_excludes_3
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `valid_op_size` allows only 1/2/4/8 — tests that 3-byte operations are excluded; if the spec is too permissive, invalid memory access sizes could be admitted
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `valid_op_size` is defined as `op_size() == 1 || == 2 || == 4 || == 8`. Since 3 is none of these values, `op_size() != 3` follows trivially. This is correct and intentional — x86 memory operations are power-of-2 sized.

### φ4: is_base_pt_walk_vbase_alignment
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** For L3 (4KB) pages, `is_base_pt_walk` requires `vbase == vaddr` where `vbase = align_to_usize(vaddr, L3_ENTRY_SIZE)` — this forces vaddr to be page-aligned, potentially excluding valid non-aligned base addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_base_pt_walk` requires `vbase == vaddr` where `vbase = align_to_usize(vaddr, L3_ENTRY_SIZE)`. Since `align_to_usize(va, 4096) == va` iff `va % 4096 == 0`, this correctly forces page alignment. This is intentional — base addresses in x86 page tables are always page-aligned.

### φ5: all_mb0_bits_are_zero_gates_pde_validity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` and gates the PDE view — when it returns false, entries with P bit set are still treated as Invalid, but the predicate's behavior is completely unverified
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` and gates `PDE::view` — when it returns false, present entries (P bit set) are treated as Invalid. Since the predicate is completely opaque, Verus cannot verify that it correctly checks the must-be-zero bits for each layer, potentially misclassifying valid or invalid entries.

