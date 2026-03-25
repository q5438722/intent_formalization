# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/util/util__page_ptr_util_u__page_index2page_ptr.rs`
**Date:** 2026-03-24T08:20:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive was identified: `lemma_usize_u64` is an unconditionally-trusted external_body axiom that asserts a u64-to-usize roundtrip identity for all u64 values without any precondition, which is unsound on 32-bit platforms and represents an unverified platform assumption even on 64-bit. The other two candidates are false positives — page index 0 mapping to address 0 is a mathematical tautology (null-pointer exclusion is a separate concern), and the MAX_USIZE bound on page pointers is arithmetically valid given the constants.

## True Positives (Spec Issues)

### lemma_usize_u64_enables_unsound_cast
- **Confidence:** high
- **Reasoning:** `lemma_usize_u64` is an external_body axiom with no precondition, asserting `x as usize as u64 == x` for all u64 values. On a 32-bit platform this is false for any `x > u32::MAX`. Even on 64-bit platforms, this is an unverified assumption that should at minimum carry a precondition like `x as int <= usize::MAX` rather than being universally quantified.

## All Candidates

### φ1: lemma_usize_u64_enables_unsound_cast
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_usize_u64` is external_body with no precondition — it asserts `x as usize as u64 == x` for ALL u64 values, but on a 32-bit platform where `usize` is 32 bits, values above `u32::MAX` would truncate and the roundtrip would fail
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_usize_u64` is an external_body axiom with no precondition, asserting `x as usize as u64 == x` for all u64 values. On a 32-bit platform this is false for any `x > u32::MAX`. Even on 64-bit platforms, this is an unverified assumption that should at minimum carry a precondition like `x as int <= usize::MAX` rather than being universally quantified.

### φ2: page_index2ptr_no_overflow_check
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The spec casts `i * 4096` to `usize` without verifying the product fits — with `NUM_PAGES = 2M`, the max product is `(2M-1)*4096 ≈ 8G` which exceeds 32-bit `usize::MAX` but the spec silently truncates via `as usize`

### φ3: max_usize_constant_incorrect
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `MAX_USIZE` is declared as `u64` with value 31GB but named as if it represents `usize::MAX` — on 64-bit platforms `usize::MAX` is ~18EB, and this constant is used in `lemma_usize_u64` proofs to justify arithmetic that may not actually hold

### φ4: page_ptr_zero_valid
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Page index 0 maps to page pointer 0 (null pointer) — if page pointers are used as memory addresses, a null page pointer being valid could mask null-dereference bugs or confuse null-check logic
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** `spec_page_index2page_ptr(0) == 0` is a direct arithmetic consequence of `0 * 4096 == 0`. Whether page index 0 should be excluded is a design decision external to this spec — the spec defines a mathematical mapping from indices to pointers, and null-pointer concerns are an implementation-level issue handled elsewhere (e.g., by starting valid page allocation at index 1).

### φ5: page_index_valid_not_linked_to_max_usize
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `page_index_valid` only checks `i < NUM_PAGES` but `MAX_USIZE` (31GB) is smaller than the maximum page pointer value `(NUM_PAGES-1)*4096 ≈ 8GB` — wait, 8GB < 31GB so this holds, but the relationship is never formally established in the spec, relying entirely on the unverified `lemma_usize_u64`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property that `spec_page_index2page_ptr(i) <= MAX_USIZE` for valid page indices is arithmetically correct: max pointer is `(2M - 1) * 4096 ≈ 8GB` which is well under `MAX_USIZE = 31GB`. While the proof path goes through `lemma_usize_u64`, the arithmetic relationship itself holds and is a desirable property confirming that page pointers fit within the system's memory bounds.

