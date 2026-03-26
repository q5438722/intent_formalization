# Spec Consistency Report

**Source:** `verusage/workspace_v4/memory-allocator__commit_mask__impl__next_run/original.rs`
**Date:** 2026-03-25T14:24:10Z

## Stats

- Candidates generated: 14
- Entailed (verified): 13
- Tautologies filtered: 0
- True positives: 13
- False positives: 0

## Summary

All 13 candidate properties are true positives targeting the exec function `next_run`. They collectively expose three interconnected spec incompleteness issues that the source code comments partially acknowledge: (1) **no completeness/liveness** — the spec is vacuously satisfied by (512,0), so it doesn't guarantee discovering any set bits (φ1, φ4, φ6, φ9, φ13); (2) **no forward-scan semantics** — the spec lacks `next_idx >= idx` and the commented-out "no skipped set bits" clause, permitting backward results and skipped runs (φ2, φ3, φ8, φ11, φ14); and (3) **no maximality** — the spec doesn't require count to cover the full contiguous run, allowing arbitrary under-reporting (φ5, φ7, φ10). While the function body correctly implements forward scanning with maximal runs, callers relying only on the spec cannot depend on any of these properties, making iterative scanning protocols potentially unsound or inefficient.

## True Positives

### always_returns_empty
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The postcondition is vacuously satisfied by (512,0) for any input, since the universal quantifier ranges over an empty set when count=0. This reveals the spec lacks any liveness/completeness guarantee — it doesn't promise to find set bits that exist. The source comments acknowledge missing clauses.

### next_idx_before_scan
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec has no `next_idx >= idx` constraint, so (0,0) satisfies the postcondition for any idx>0. The function body scans forward from idx, but callers cannot rely on this. This is a genuine spec gap affecting iteration correctness.

### skips_earlier_run
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec allows returning (200,10) while a closer run [10,20) exists at idx=5. The source code explicitly has the commented-out clause `forall |t| idx<=t<next_idx ==> !self@.contains(t)` that would prevent this, acknowledging the gap. Callers relying on nearest-run semantics are unprotected.

### set_bit_zero_count
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. Even when a set bit exists exactly at idx, the spec permits (512,0) via vacuous truth. The body would find it, but the spec doesn't promise it. This is a stronger witness of the same completeness gap as φ1, with a tighter precondition making the deficiency starker.

### non_maximal_count
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec only requires reported bits be set, not that count is maximal. Returning (idx,1) for a 10-bit run is spec-legal. The source comments acknowledge "we could have a condition that count is not smaller than necessary." Callers depending on full-run discovery are unprotected.

### boundary_511_empty
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. At the maximum boundary with bit 511 set, the spec still permits (512,0). This is a boundary-specific instance of the completeness gap. The body handles this correctly, but the spec doesn't guarantee the last set bit is discoverable.

### all_set_single_bit_run
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. With all 512 bits set, the spec allows count=1 — a gross under-report requiring 512 iterations instead of 1. This is an extreme case of the maximality gap, demonstrating that callers performing iterative scanning face worst-case blowup without a maximality guarantee.

### no_forward_progress
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. Same underlying issue as φ2: spec allows next_idx=0 < idx. Callers using an iteration pattern `idx = next_idx + count` risk infinite loops since forward progress is not guaranteed by the spec, only by the body.

### skipped_set_bits
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec allows (512,0) while a set bit at idx lies in the skipped range [idx, next_idx). This combines the completeness gap with the missing "no set bits skipped" clause, showing callers cannot trust that all reachable set bits are eventually reported.

### non_maximal_run
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec permits count=1 for a 10-bit run, and this φ explicitly witnesses that the bit at next_idx+count is still set — a direct maximality violation. This is a stronger form of φ5 since it provides an observable witness bit proving the run was cut short.

### zero_count_not_sentinel
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec allows (0,0) as a valid return — count=0 but next_idx≠512. Callers using `next_idx == COMMIT_MASK_BITS` as a "scan complete" sentinel cannot distinguish this from a legitimate interior empty result, breaking termination detection protocols.

### no_completeness
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. Equivalent to φ4 — (512,0) is valid despite a set bit at idx. This directly demonstrates the spec is consistent with a trivial implementation that never finds any bits, confirming the completeness gap from a different angle.

### interior_run_start
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Targets exec function `next_run`. The spec allows returning (15,3) for a contiguous run [10,20), starting mid-run rather than at bit 10. This combines the missing "nearest first set bit" guarantee with the maximality gap — callers miss committed slices both before and after the reported sub-range.

## All Candidates

### φ1: always_returns_empty → `next_run`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** A trivial implementation returning (512,0) satisfies the postcondition via vacuous truth over the empty range, defeating the purpose of scanning for set-bit runs.
- **Verdict:** TRUE_POSITIVE (high)

### φ2: next_idx_before_scan → `next_run`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** The spec has no constraint next_idx >= idx, so (0, 0) satisfies the postcondition for any idx > 0, breaking callers that assume forward progress.
- **Verdict:** TRUE_POSITIVE (high)

### φ3: skips_earlier_run → `next_run`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** The spec allows returning (200,10) while skipping the earlier run at [10,20), since the commented-out clause forall |t| idx<=t<next_idx ==> !self@.contains(t) is not enforced.
- **Verdict:** TRUE_POSITIVE (high)

### φ4: set_bit_zero_count → `next_run`
- **Type:** boundary | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** Despite a set bit at the scan start, (512,0) is a valid return via vacuous truth, so callers relying on next_run to discover set regions may miss them entirely.
- **Verdict:** TRUE_POSITIVE (high)

### φ5: non_maximal_count → `next_run`
- **Type:** logical | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** The spec only requires reported bits to be set, not that count is maximal; returning (idx,1) for a 10-bit run satisfies the postcondition but under-reports the contiguous region.
- **Verdict:** TRUE_POSITIVE (high)

### φ6: boundary_511_empty → `next_run`
- **Type:** boundary | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** At the maximum boundary idx=511 with bit 511 set, the spec still permits the degenerate (512,0) return, failing to detect the last set bit.
- **Verdict:** TRUE_POSITIVE (high)

### φ7: all_set_single_bit_run → `next_run`
- **Type:** logical | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** For an all-ones mask the spec allows count=1, grossly under-reporting the 512-bit contiguous region; iteration would need 512 calls instead of 1.
- **Verdict:** TRUE_POSITIVE (high)

### φ8: no_forward_progress → `next_run`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** The spec allows next_idx=0 < idx, so callers iterating with increasing idx cannot assume forward progress, risking infinite loops.
- **Verdict:** TRUE_POSITIVE (high)

### φ9: skipped_set_bits → `next_run`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** The spec allows (512,0) while a set bit at idx lies in [idx, next_idx), so the scanning semantics (all skipped positions are unset) is not enforced.
- **Verdict:** TRUE_POSITIVE (high)

### φ10: non_maximal_run → `next_run`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** The spec permits count=1 for a 10-bit run, with the bit right after the reported run still set, showing maximality is not enforced.
- **Verdict:** TRUE_POSITIVE (high)

### φ11: zero_count_not_sentinel → `next_run`
- **Type:** boundary | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** The spec allows (0,0) as a valid return with count==0 but next_idx!=512, so callers using next_idx==512 as a termination sentinel cannot distinguish it from an interior empty result.
- **Verdict:** TRUE_POSITIVE (high)

### φ12: positive_count_high_idx → `next_run`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ❌
- **Why flagged:** This gap is actually already closed by the spec: next_idx+count<=512 with count>0 implies next_idx<512 arithmetically, so this proof fn should NOT verify.

### φ13: no_completeness → `next_run`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** Even with a set bit at idx, (512,0) satisfies the postcondition, so the spec is consistent with a trivial implementation that never finds any set bits.
- **Verdict:** TRUE_POSITIVE (high)

### φ14: interior_run_start → `next_run`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** The spec allows returning (15,3) for a run at [10,20), starting at an interior point rather than the true run beginning at 10, causing callers to miss committed-but-unreported slices.
- **Verdict:** TRUE_POSITIVE (high)

