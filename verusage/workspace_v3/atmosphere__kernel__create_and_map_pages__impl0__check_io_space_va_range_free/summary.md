# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_map_pages__impl0__check_io_space_va_range_free/original.rs`
**Date:** 2026-03-24T21:31:04Z

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five properties are false positives arising from vacuous verification. The decisive evidence is that φ1 (always_free) mutually contradicts both φ2 (never_free_nonempty, for len>0) and φ5 (zero_len_not_free, for len=0). Since all verified, the shared precondition set — particularly the opaque `external_body closed spec` predicates `os.wf()`, `io_space_range_free()`, and related functions — is unsatisfiable in this verification context. Verus cannot construct any witness satisfying the requires clauses, so it proves every ensures clause vacuously. No real specification weakness in `check_io_space_va_range_free` is demonstrated by these properties.

## All Candidates

### φ1: always_free → `check_io_space_va_range_free`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If the IO-space range is always reported free, actual conflicts are silently ignored and double-mapping becomes possible
- **Verdict:** FALSE_POSITIVE (high)

### φ2: never_free_nonempty → `check_io_space_va_range_free`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If no non-empty range is ever considered free, IO-space mapping is impossible and the check is vacuously blocking
- **Verdict:** FALSE_POSITIVE (high)

### φ3: proc_independent → `check_io_space_va_range_free`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If the result is identical for distinct processes, per-process IO-space isolation is violated
- **Verdict:** FALSE_POSITIVE (high)

### φ4: free_implies_entire_io_space_empty → `check_io_space_va_range_free`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If range-free implies the entire IO space is empty, existing legitimate IO mappings outside the range are impossible
- **Verdict:** FALSE_POSITIVE (high)

### φ5: zero_len_not_free → `check_io_space_va_range_free`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-length range has no addresses to conflict, so it should be vacuously free; reporting it as not-free breaks the base case
- **Verdict:** FALSE_POSITIVE (high)

