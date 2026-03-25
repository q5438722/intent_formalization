# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_finish_iter_walk_invalid_after_nonpos_write.rs`
**Date:** 2026-03-24T13:39:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `all_mb0_bits_are_zero` is an external_body predicate critical to PDE interpretation (though the test is trivially a tautology), and `Walk::result` falls through to `arbitrary()` for path.len() == 1 Page entries, leaving L0 page walk results undefined. Three false positives: flags combine commutativity is correct, the invalid-after-nonpos-write lemma is already proven in the source, and the US→is_supervisor inversion matches x86 hardware semantics.

## True Positives (Spec Issues)

### all_mb0_bits_are_zero_external_body
- **Confidence:** low
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with no spec, used in the critical `PDE::view` to determine entry validity. However, this specific φ only proves `P || !P` (tautology), so it doesn't actually demonstrate the external_body gap. The underlying trust assumption is real but the test is trivial.

### walk_result_path_len_1_arbitrary
- **Confidence:** medium
- **Reasoning:** `Walk::result` handles path lengths 2, 3, 4 but falls through to `arbitrary()` for path.len() == 1. Since `walk_next` at layer 0 can produce a `Page` entry (setting `complete: true`) with path.len() == 1, the result is unspecified. While x86-64 doesn't support L0 huge pages, the spec doesn't prevent this case and `arbitrary()` could yield `Valid` with garbage values.

## All Candidates

### φ1: all_mb0_bits_are_zero_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — completely opaque predicate used in `PDE::view` to determine validity, yet has no specification
- **Verdict:** TRUE_POSITIVE (low)
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with no spec, used in the critical `PDE::view` to determine entry validity. However, this specific φ only proves `P || !P` (tautology), so it doesn't actually demonstrate the external_body gap. The underlying trust assumption is real but the test is trivial.

### φ2: flags_combine_not_commutative_supervisor
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `combine` uses OR for `is_supervisor` and `disable_execute` — commutativity holds but the asymmetric AND for `is_writable` vs OR for supervisor could mask a precedence error
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` uses AND for `is_writable` and OR for `is_supervisor`/`disable_execute` — all three operations are commutative. This commutativity is correct and expected behavior.

### φ3: walk_result_path_len_1_arbitrary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Walk::result` handles path lengths 2, 3, 4 for Page entries but falls through to `arbitrary()` for path.len() == 1 — a 1-entry Page walk (L0 huge page) is undefined
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Walk::result` handles path lengths 2, 3, 4 but falls through to `arbitrary()` for path.len() == 1. Since `walk_next` at layer 0 can produce a `Page` entry (setting `complete: true`) with path.len() == 1, the result is unspecified. While x86-64 doesn't support L0 huge pages, the spec doesn't prevent this case and `arbitrary()` could yield `Valid` with garbage values.

### φ4: finish_iter_walk_nonpos_write_preserves_invalid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Clearing the P bit of any present entry preserves walk invalidity — but this assumes walks that were invalid before can't become valid by changing entries along the path, which depends on walk_next not creating new valid paths
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and already-proven lemma in the source code (`lemma_finish_iter_walk_invalid_after_nonpos_write`). Clearing a P bit (making an entry non-present) can only make walks more invalid, never less — an invalid walk stays invalid because clearing P bits removes directory references, it doesn't create new valid paths.

### φ5: flags_from_bits_supervisor_inverted
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `is_supervisor` is `!flag_US` — the inversion means US=1 (user allowed) maps to is_supervisor=false, which is correct but could confuse callers expecting US=1 to mean supervisor access
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The inversion `is_supervisor = !flag_US` correctly matches x86 semantics — the US bit being set (1) means user-mode access is allowed, which means the page is NOT supervisor-only. This is the standard Intel interpretation.

