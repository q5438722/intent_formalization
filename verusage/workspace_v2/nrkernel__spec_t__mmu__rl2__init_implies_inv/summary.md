# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__init_implies_inv.rs`
**Date:** 2026-03-24T13:34:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `inv_unmapping__core_vs_writer_reads` and `inv_unmapping__notin_nonpos` are both `external_body` predicates with no spec bodies — completely opaque trust assumptions in the unmapping invariant. Two false positives: the vacuous invariant on unhappy states is a standard UB modeling pattern, and empty TSO implying no pending maps correctly captures the relationship between store buffer drainage and map completion.

## True Positives (Spec Issues)

### inv_unmapping_core_vs_writer_reads_external_body
- **Confidence:** medium
- **Reasoning:** `inv_unmapping__core_vs_writer_reads` is `external_body` with `unimplemented!()` and no spec body — it's a completely opaque predicate included as part of the unmapping invariant. Its actual content is unknown and unverified, making it an unauditable trust assumption.

### inv_unmapping_notin_nonpos_external_body
- **Confidence:** medium
- **Reasoning:** `inv_unmapping__notin_nonpos` is `external_body` with `unimplemented!()` and no spec body — another completely opaque predicate in the unmapping invariant. Its semantics are unknown and unverifiable, representing a trust gap in the invariant.

## All Candidates

### φ1: walk_next_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `walk_next` is `external_body` with no spec — completely opaque, meaning Verus treats identical inputs as producing identical outputs only by coincidence

### φ2: inv_unmapping_core_vs_writer_reads_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `inv_unmapping__core_vs_writer_reads` is `external_body` with no spec — the unmapping invariant includes a completely opaque predicate that could be anything
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `inv_unmapping__core_vs_writer_reads` is `external_body` with `unimplemented!()` and no spec body — it's a completely opaque predicate included as part of the unmapping invariant. Its actual content is unknown and unverified, making it an unauditable trust assumption.

### φ3: inv_unmapping_notin_nonpos_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `inv_unmapping__notin_nonpos` is `external_body` with no spec — another completely opaque unmapping invariant component trusted without any specification
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `inv_unmapping__notin_nonpos` is `external_body` with `unimplemented!()` and no spec body — another completely opaque predicate in the unmapping invariant. Its semantics are unknown and unverifiable, representing a trust gap in the invariant.

### φ4: inv_vacuous_when_unhappy
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv` is guarded by `self.happy` — once `happy` becomes false, all invariants are vacuously satisfied, allowing any state to be considered valid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional design — `happy` serves the same role as `sound` in the hlspec layer. Once the system enters an unhappy/unsound state, invariant preservation becomes vacuous. This is a standard pattern for modeling undefined behavior in state machine verification.

### φ5: mapping_empty_tso_implies_no_pending_maps
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When in Mapping polarity with empty TSO set, all pending maps must be empty — this means if the store buffer is drained before completing a map, the pending map history is lost
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by design. When the TSO set is empty, all store buffer writes have been drained and become globally visible — meaning any pending maps have been committed to `pt_mem` and are no longer "pending." The invariant correctly captures that pending maps only exist while writes are still buffered.

