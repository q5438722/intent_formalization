# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__next_run/original.rs`
**Date:** 2026-03-25 04:06:33
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four properties are true positives exposing genuine spec weaknesses in `next_run`. The ensures clause only guarantees bounds and that returned positions are set bits — it lacks three key properties the authors themselves identified in comments: (1) completeness (if bits exist, they must be found), (2) forward-scan (result must be at or after idx), and (3) maximality (count must be the full contiguous run length). These gaps mean callers cannot rely on the function actually scanning forward or returning useful results, even though the implementation does behave correctly.

## True Positives (Spec Issues)

### trivial_not_found
- **Confidence:** high
- **Reasoning:** The spec allows returning (512, 0) for any input, satisfying the postcondition vacuously since the forall over an empty range is trivially true. This means the function could always claim "no run found" without actually scanning. The spec lacks a completeness guarantee (e.g., count == 0 implies no set bits from idx onward).

### run_before_search
- **Confidence:** high
- **Reasoning:** The ensures clause places no constraint relating `next_idx` to `idx`. The spec allows returning a run starting before the search position, violating the implicit forward-scan contract. The source comments even acknowledge this: `//forall |t| idx <= t < next_idx ==> !self@.contains(t)` is commented out.

### non_maximal_run
- **Confidence:** high
- **Reasoning:** The spec guarantees that returned bits are set but not that the count is maximal. A conforming implementation could return count=1 when the actual contiguous run is longer. The source comments explicitly acknowledge this: "Likewise we could have a condition that `count` is not smaller than necessary."

### skipped_earlier_run
- **Confidence:** high
- **Reasoning:** Since the spec doesn't require `next_idx` to be the first set bit at or after `idx`, it allows skipping an earlier run entirely. This is the same root cause as run_before_search — the commented-out `forall |t| idx <= t < next_idx ==> !self@.contains(t)` would prevent this.

## All Candidates

### φ1: trivial_not_found
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Spec allows always returning (512, 0) regardless of bit state, making the function vacuously correct but useless
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The spec allows returning (512, 0) for any input, satisfying the postcondition vacuously since the forall over an empty range is trivially true. This means the function could always claim "no run found" without actually scanning. The spec lacks a completeness guarantee (e.g., count == 0 implies no set bits from idx onward).

### φ2: run_before_search
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Spec allows returning a run at position 0 when searching from position 256, violating the forward-scan contract
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The ensures clause places no constraint relating `next_idx` to `idx`. The spec allows returning a run starting before the search position, violating the implicit forward-scan contract. The source comments even acknowledge this: `//forall |t| idx <= t < next_idx ==> !self@.contains(t)` is commented out.

### φ3: zero_count_implies_empty
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, count==0 would force the entire mask to be empty, over-constraining callers who expect partial scans

### φ4: non_maximal_run
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Spec allows returning count=1 when the actual contiguous run is at least 2, so callers cannot trust count is maximal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The spec guarantees that returned bits are set but not that the count is maximal. A conforming implementation could return count=1 when the actual contiguous run is longer. The source comments explicitly acknowledge this: "Likewise we could have a condition that `count` is not smaller than necessary."

### φ5: skipped_earlier_run
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Spec allows skipping a set bit at idx and reporting a later run, so callers cannot rely on getting the first available run
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Since the spec doesn't require `next_idx` to be the first set bit at or after `idx`, it allows skipping an earlier run entirely. This is the same root cause as run_before_search — the commented-out `forall |t| idx <= t < next_idx ==> !self@.contains(t)` would prevent this.

