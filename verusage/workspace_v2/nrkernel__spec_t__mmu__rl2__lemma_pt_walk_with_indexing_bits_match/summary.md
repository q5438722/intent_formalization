# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_pt_walk_with_indexing_bits_match.rs`
**Date:** 2026-03-24T13:53:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: the external_body `lemma_bits_align_to_usize` trusts 8-byte alignment preserving all indexing bits without proof. Three false positives: the indexing bits match properties (same path, same PTE, validity symmetry) are correct and desirable page table semantics, verified by Verus with only an indirect dependency on the alignment broadcast hint.

## True Positives (Spec Issues)

### lemma_bits_align_external_body_word_size
- **Confidence:** high
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties are trusted without proof. 8-byte alignment zeroes only bits 0-2, which should indeed preserve bits 12+ (all indexing bits), but this is unverified.

## All Candidates

### φ1: lemma_bits_align_external_body_word_size
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bits_align_to_usize` is `external_body` — 8-byte alignment preserving all indexing bits is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_bits_align_to_usize` is `external_body` with `unimplemented!()` — all bit-alignment preservation properties are trusted without proof. 8-byte alignment zeroes only bits 0-2, which should indeed preserve bits 12+ (all indexing bits), but this is unverified.

### φ2: indexing_bits_match_implies_same_path
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two addresses with matching indexing bits produce the same walk path — depends on external_body `lemma_bits_align_to_usize`; if bit preservation is wrong, different addresses could share paths incorrectly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property — if two addresses have matching page table indices at all relevant levels, they must traverse the same path through the page table hierarchy. The proof is verified by Verus (not external_body); it merely uses the alignment lemma as a broadcast hint. The property itself follows from `pt_walk`'s structure.

### φ3: indexing_bits_match_implies_same_pte
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Addresses with matching indexing bits that produce valid walks share the same PTE — this means all addresses within a page map to the same physical frame, which depends on the unverified alignment lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is fundamental page table semantics — all virtual addresses within the same page must map to the same physical frame with the same flags. The proof is verified by Verus; the external_body dependency is only the alignment broadcast hint.

### φ4: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — a zero entry should have all must-be-zero bits clear, but the predicate is opaque so this could be false

### φ5: walk_path_determines_validity_symmetry
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Validity is symmetric for addresses with matching indexing bits — depends on external_body lemma chain; if incorrect, one address in a page could be valid while another is invalid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Validity symmetry for addresses sharing indexing bits is correct — if two addresses index the same page table entries, they must both be valid or both be invalid. This follows directly from the verified `lemma_pt_walk_with_indexing_bits_match`.

