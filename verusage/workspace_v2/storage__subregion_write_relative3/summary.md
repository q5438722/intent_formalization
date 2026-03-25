# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_subregion/subregion_write_relative3.rs`
**Date:** 2026-03-24T15:28:27Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The address translation roundtrip is trivial integer arithmetic, and the double-write behavior at the spec level is intentional — the `no_outstanding_writes_in_range` precondition on the exec-level `write` prevents this scenario from occurring in verified code.

## All Candidates

### φ1: pm_write_external_body_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The write spec sets outstanding_write to Some(byte) for each written position — if the map function had an off-by-one in `bytes[pos - addr]`, written bytes would be shifted relative to their intended addresses

### φ2: write_relative_writable_check_absolute_translation
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The `is_writable_relative_addr` translates via `addr + self.start()` while `write_relative` uses `relative_addr + self.start_` — if the translation between relative and absolute addressing were inconsistent, the writability check would guard the wrong range
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(relative_addr + start) - start == relative_addr` is basic integer arithmetic. This confirms the address translation is consistent, which is a correct and expected property.

### φ3: write_preserves_views_differ_outside_writable
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** After writing within the writable subregion, non-writable addresses must still match the initial view — if the views_differ invariant weren't preserved through writes, the opaque_inv would break after the first mutation

### φ4: write_double_outstanding_silently_overwrites
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The write spec overwrites any existing outstanding_write without checking — a second write before flush silently discards the first written value, even though the `no_outstanding_writes_in_range` precondition on `PersistentMemoryRegion::write` is supposed to prevent this; if the precondition check were bypassed, data loss would occur
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `PersistentMemoryByte::write` spec intentionally sets `outstanding_write: Some(byte)` regardless of existing state — this is the correct definition. The `no_outstanding_writes_in_range` precondition on `PersistentMemoryRegion::write` prevents double writes at the exec level. The spec-level function merely defines what would happen; the precondition guards actual usage.

### φ5: write_relative_no_outstanding_forwarding
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `write_relative` function checks no-outstanding-writes on the subregion view but calls `pm.write` which requires it on absolute addresses — if the subrange indexing didn't correctly forward this property, the precondition of the underlying write would not be met

