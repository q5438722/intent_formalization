# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_valid_not_pending_implies_equal.rs`
**Date:** 2026-03-24T14:15:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives targeting external_body lemmas: `lemma_view_addr_aligned` trusts both directory and page address alignment/overflow bounds from bitmask extraction without bitvector proof, `lemma_bit_indices_less_512` trusts the 9-bit index bound, and the combination of `lemma_write_seq` + `lemma_write_seq_idle` chains two unverified assumptions. One false positive: the L0 address computation is correct by construction from `pt_walk`'s definition.

## True Positives (Spec Issues)

### lemma_view_addr_aligned_external_body
- **Confidence:** high
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` with `unimplemented!()` — directory address 4KB-alignment and overflow bounds are asserted from PDE bitmask extraction without bitvector proof. This is a critical trust assumption since pt_walk uses these addresses to compute next-level table locations.

### lemma_bit_indices_less_512_external_body
- **Confidence:** high
- **Reasoning:** `lemma_bit_indices_less_512` is `external_body` with `unimplemented!()` — the <512 bound for 9-bit field extraction is provable by `by (bit_vector)` but is instead trusted without proof. This bound is used to ensure page table index validity throughout pt_walk.

### write_seq_idle_plus_write_seq
- **Confidence:** high
- **Reasoning:** Both `lemma_write_seq` (domain monotonicity) and `lemma_write_seq_idle` (read frame) are `external_body` with `unimplemented!()`. This chains two unverified assumptions to establish that disjoint-address writes preserve both domain membership and read values — a compound trust assumption.

### page_addr_aligned_external_body
- **Confidence:** high
- **Reasoning:** Same as `lemma_view_addr_aligned_external_body` but for Page entries — page frame address 4KB-alignment and overflow bounds from bitmask extraction are trusted without proof. This is essential for walk result correctness since page addresses feed into physical address computation.

## All Candidates

### φ1: lemma_view_addr_aligned_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view_addr_aligned` is `external_body` — directory address alignment and overflow bounds are trusted without proof from the PDE bit-extraction logic
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` with `unimplemented!()` — directory address 4KB-alignment and overflow bounds are asserted from PDE bitmask extraction without bitvector proof. This is a critical trust assumption since pt_walk uses these addresses to compute next-level table locations.

### φ2: lemma_bit_indices_less_512_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bit_indices_less_512` is `external_body` — the 9-bit extraction bound (<512) for each page table level index is trusted without bitvector proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_bit_indices_less_512` is `external_body` with `unimplemented!()` — the <512 bound for 9-bit field extraction is provable by `by (bit_vector)` but is instead trusted without proof. This bound is used to ensure page table index validity throughout pt_walk.

### φ3: pt_walk_l0_addr_in_pml4_range
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The first path entry address should be pml4 + l0_index * 8 — tests that pt_walk correctly computes the L0 address from pml4 base
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pt_walk` defines `l0_addr = add(self.pml4, l0_idx)` and constructs the path with `l0_addr` as the first entry's address in all branches. This is correct by construction — the ensures is a direct read-back of the definition.

### φ4: write_seq_idle_plus_write_seq
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Both `lemma_write_seq` and `lemma_write_seq_idle` are `external_body` — domain preservation and read frame together chain two unverified assumptions
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Both `lemma_write_seq` (domain monotonicity) and `lemma_write_seq_idle` (read frame) are `external_body` with `unimplemented!()`. This chains two unverified assumptions to establish that disjoint-address writes preserve both domain membership and read values — a compound trust assumption.

### φ5: page_addr_aligned_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view_addr_aligned` trusts page frame address alignment and overflow bounds — the bitmask extraction producing a 4KB-aligned address is unverified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same as `lemma_view_addr_aligned_external_body` but for Page entries — page frame address 4KB-alignment and overflow bounds from bitmask extraction are trusted without proof. This is essential for walk result correctness since page addresses feed into physical address computation.

