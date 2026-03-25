# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/log_append/append_L_tentatively_append_wrapping.rs`
**Date:** 2026-03-24T15:03:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives all stem from `lemma_tentatively_append` being `external_body`: the invariant preservation after writing, the no-outstanding-writes precondition, and the recovery-unreachability guarantee are all trusted without proof. These represent significant unverified trust assumptions in the crash-safe log implementation. One false positive: sequential append pending concatenation is correct by definition and sequence associativity.

## True Positives (Spec Issues)

### lemma_tentatively_append_external_body
- **Confidence:** high
- **Reasoning:** `lemma_tentatively_append` is `external_body` with `unimplemented!()` body. Its ensures clause asserts that writing bytes to the log area preserves `info_consistent_with_log_area` — a complex invariant involving circular buffer arithmetic, persistent memory state, and flush semantics. This entire proof is trusted without verification.

### no_outstanding_writes_from_external_body
- **Confidence:** high
- **Reasoning:** This property is derived from the same `external_body` lemma. The claim that the write target range has no outstanding writes follows from the invariant's guarantee about positions beyond `log_plus_pending_length`, but this reasoning is trusted, not verified. It's a distinct safety-critical sub-conclusion of the unverified lemma.

### unreachable_during_recovery_from_external_body
- **Confidence:** high
- **Reasoning:** Also derived from the same `external_body` lemma. The crash-safety property that write targets are unreachable during recovery is critical — if wrong, partial writes could corrupt recovered log state. This is trusted without proof.

## All Candidates

### φ1: lemma_tentatively_append_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_tentatively_append` is `external_body` with `unimplemented!()` — the non-wrapping append correctness proof is entirely trusted; if the write address calculation or invariant preservation is wrong, log corruption would go undetected
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_tentatively_append` is `external_body` with `unimplemented!()` body. Its ensures clause asserts that writing bytes to the log area preserves `info_consistent_with_log_area` — a complex invariant involving circular buffer arithmetic, persistent memory state, and flush semantics. This entire proof is trusted without verification.

### φ2: no_outstanding_writes_from_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The external_body lemma asserts no outstanding writes in the target range — this crash-safety precondition is trusted without proof; if the invariant doesn't actually guarantee this, writes could overwrite in-flight data
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This property is derived from the same `external_body` lemma. The claim that the write target range has no outstanding writes follows from the invariant's guarantee about positions beyond `log_plus_pending_length`, but this reasoning is trusted, not verified. It's a distinct safety-critical sub-conclusion of the unverified lemma.

### φ3: unreachable_during_recovery_from_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The external_body lemma asserts write targets are unreachable during recovery — this crash-safety property is trusted; if recovery could reach these offsets, a crash mid-write would expose partially written data
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Also derived from the same `external_body` lemma. The crash-safety property that write targets are unreachable during recovery is critical — if wrong, partial writes could corrupt recovered log state. This is trusted without proof.

### φ4: tentatively_append_pending_concat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Sequential tentative appends should concatenate pending bytes in order — if the associativity of sequence concatenation broke this, two successive appends would not equal a single combined append
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `tentatively_append` is defined as `Self { pending: self.pending + bytes, ..self }`. Chaining two appends gives `(self.pending + b1) + b2` which equals `self.pending + b1 + b2` by sequence concatenation associativity. Correct by construction.

### φ5: loginfo_tentatively_append_overflow
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `LogInfo::tentatively_append` casts to u64 — if `log_plus_pending_length + num_bytes` overflows u64, the field silently wraps around, making the log metadata inconsistent with the actual pending data length

