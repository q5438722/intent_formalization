# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_indexing_bits_match_len_decrease.rs`
**Date:** 2026-03-24T13:40:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: the predicate saturates at len=4 with no additional constraints for higher levels (relevant if 5-level paging is ever modeled), and bits 48-63 are unchecked, meaning non-canonical address pairs are treated as matching. Three false positives: len=0 trivial truth, reflexivity, and ignoring the page offset (bits 0-11) are all correct and expected properties of a page table indexing predicate.

## True Positives (Spec Issues)

### indexing_bits_match_len5_constrains_nothing_extra
- **Confidence:** medium
- **Reasoning:** The predicate saturates at len=4 — any len > 4 is equivalent to len=4 because no additional constraints are added. If a caller passes len=5 expecting stricter matching (e.g., a 5-level paging extension like LA57), the predicate silently accepts it with no extra checks. This is a spec weakness for future extensibility.

### indexing_bits_match_ignores_high_bits
- **Confidence:** medium
- **Reasoning:** Bits 48-63 are not checked by `indexing_bits_match`. On x86-64, bits 48-63 must be a sign-extension of bit 47 for canonical addresses. Non-canonical addresses differing only in bits 48-63 would be considered matching, which could mask canonicality violations if the predicate is used without a separate canonical address check.

## All Candidates

### φ1: indexing_bits_match_zero_trivial
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `indexing_bits_match` with len=0 is trivially true for any two addresses — no bits are required to match, yet this passes the predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** len=0 meaning "no levels to match" is the correct base case. All implications `len > k ==> ...` are vacuously true when len=0. This is standard and intentional.

### φ2: indexing_bits_match_reflexive
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any address matches itself at all lengths — tests basic reflexivity of the predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Reflexivity is a basic correctness property of any equality-based predicate. An address should always match itself at every level.

### φ3: indexing_bits_match_ignores_low_12
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two addresses within the same 4KB page match all 4 indexing levels — page table walks for same-page addresses follow identical paths, which is correct but means the predicate can't distinguish intra-page offsets
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Bits 0-11 are the page offset, not part of any page table index. Two addresses in the same 4KB page should follow identical page table walks. This is correct x86 semantics.

### φ4: indexing_bits_match_len5_constrains_nothing_extra
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `indexing_bits_match` only has 4 levels of constraints — len > 4 adds no additional bit-matching requirements, so len=5 is equivalent to len=4
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The predicate saturates at len=4 — any len > 4 is equivalent to len=4 because no additional constraints are added. If a caller passes len=5 expecting stricter matching (e.g., a 5-level paging extension like LA57), the predicate silently accepts it with no extra checks. This is a spec weakness for future extensibility.

### φ5: indexing_bits_match_ignores_high_bits
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `indexing_bits_match` only checks bits 12-47 — addresses differing only in bits 48-63 (non-canonical bits) are considered matching at all levels
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Bits 48-63 are not checked by `indexing_bits_match`. On x86-64, bits 48-63 must be a sign-extension of bit 47 for canonical addresses. Non-canonical addresses differing only in bits 48-63 would be considered matching, which could mask canonicality violations if the predicate is used without a separate canonical address check.

