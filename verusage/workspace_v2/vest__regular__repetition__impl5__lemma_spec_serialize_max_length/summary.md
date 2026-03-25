# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__repetition__impl5__lemma_spec_serialize_max_length.rs`
**Date:** 2026-03-24T15:41:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: serialize_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Serializing an empty sequence should produce empty bytes — if fold_left injected phantom data for zero elements, downstream combinators would be corrupted

### φ2: serialize_ignores_count_field
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `spec_serialize` uses `fold_left` over `vs` without referencing `self.1` — the count field is completely ignored during serialization

### φ3: wf_helper_equals_wf
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `wf` and `wf_helper` should be equivalent when `n == self.1` — if they diverged, the roundtrip proof using `wf_helper` would not transfer to the `wf`-based trait contract

### φ4: serialize_append_splits
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Serialization of a sequence should equal serialization of all-but-last concatenated with serialization of the last element — if fold_left didn't decompose this way, the inductive roundtrip proof structure would be unsound

### φ5: max_length_lemma_individual_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If the total serialization fits in usize, each individual element's serialization must also fit — if any individual exceeded usize::MAX while the total didn't, the addition would have overflowed

