# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__mem_util__impl0__get_address_space_va_range_none/original.rs`
**Date:** 2026-03-25 04:04:08
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One candidate property was analyzed. `vacuous_empty_range` is a true positive at medium confidence: it exposes that `get_address_space_va_range_none` (an executable function) vacuously succeeds for zero-length VA ranges, meaning any caller that doesn't separately validate `va_range.len > 0` could incorrectly conclude a region is free. This is a boundary-condition spec gap — the function's contract should either require `va_range.len > 0` or callers should be aware that empty ranges are trivially "unmapped." Whether this is exploitable depends on upstream call sites, but the spec as written permits a degenerate bypass.

## True Positives (Spec Issues)

### vacuous_empty_range
- **Confidence:** medium
- **Reasoning:** The property demonstrates that `get_address_space_va_range_none` trivially returns `true` for any zero-length `VaRange4K`, regardless of whether the address space has mapped pages. This is a real spec weakness: callers relying on this function to validate that a VA range is unmapped could pass a degenerate zero-length range and bypass the check entirely. The ensures clause on the executable function `get_address_space_va_range_none` vacuously holds when `va_range.len == 0`, which could mask bugs in callers that don't independently validate range length.

## All Candidates

### φ1: map_entry_always_writable
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the spec fails to preserve the write permission bit, silently granting write access to read-only pages

### φ2: map_entry_addr_zeroed
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the spec discards the physical address, meaning all map entries alias to address zero regardless of input

### φ3: map_entry_perms_swapped
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, write and execute_disable are transposed in the mapping, creating executable-but-unwritable entries from writable-executable inputs

### φ4: vacuous_empty_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If provable, a zero-length range vacuously succeeds even with mapped VAs, letting callers skip real emptiness checks on degenerate inputs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The property demonstrates that `get_address_space_va_range_none` trivially returns `true` for any zero-length `VaRange4K`, regardless of whether the address space has mapped pages. This is a real spec weakness: callers relying on this function to validate that a VA range is unmapped could pass a degenerate zero-length range and bypass the check entirely. The ensures clause on the executable function `get_address_space_va_range_none` vacuously holds when `va_range.len == 0`, which could mask bugs in callers that don't independently validate range length.

### φ5: off_by_one_range_check
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the spec's upper bound is effectively len-1 instead of len, missing the last VA and allowing the function to return true despite an existing mapping at the range boundary

