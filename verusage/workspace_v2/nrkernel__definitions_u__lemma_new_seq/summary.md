# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__lemma_new_seq.rs`
**Date:** 2026-03-24T12:22:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. They test correct and intended properties of the fully verified `new_seq` function and its `lemma_new_seq` lemma — length, constant fill, base case, and recursive structure. No external_body or trust gaps exist in this file.

## All Candidates

### φ1: new_seq_length
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The sequence length equals `i` exactly — tests the core length property
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the first ensures clause of the fully verified `lemma_new_seq`. Correct by inductive proof.

### φ2: new_seq_all_same
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Every element equals `e` — the sequence is a constant function, cannot represent varying initialization
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the second ensures clause of `lemma_new_seq`. Every element being `e` is the intended semantics of a constant-fill sequence constructor.

### φ3: new_seq_zero_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `new_seq(0, _)` produces an empty sequence — tests the base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Directly follows from the open spec base case `i == 0 → seq![]`. Correct by definition.

### φ4: new_seq_one_singleton
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `new_seq(1, e)` is a singleton — tests the minimal non-trivial case

### φ5: new_seq_push_extends
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The recursive step is directly exposed — tests that the open spec definition unfolds as expected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec recursive case. This is the definition itself, not a spec gap.

