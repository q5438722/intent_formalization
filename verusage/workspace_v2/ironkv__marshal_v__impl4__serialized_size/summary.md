# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl4__serialized_size.rs`
**Date:** 2026-03-24T09:41:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

All five candidates are true positives identifying unnecessarily `external_body` declarations. Three (φ1–3) flag `serialized_size` exec implementations that are `unimplemented!()` stubs with trusted ensures — the spec properties they claim are correct but the exec bodies panic at runtime. Two (φ4–5) flag sequence fold lemmas that are mathematically correct but entirely trusted without proof, forming the unverified foundation of the Vec<T>::serialized_size verification. The common theme is provable properties left as trusted axioms.

## True Positives (Spec Issues)

### u64_serialized_size_external_body
- **Confidence:** medium
- **Reasoning:** `u64::serialized_size` is `external_body` with `unimplemented!()` — the exec body panics at runtime while the spec-exec ensures is trusted without verification. The spec property itself (8-byte length) is correct and provable, but the exec implementation is missing.

### usize_serialized_size_external_body
- **Confidence:** medium
- **Reasoning:** Same pattern — `usize::serialized_size` is `external_body` with `unimplemented!()`. The ensures `res as int == self.ghost_serialize().len()` is trusted, and the exec body will panic. Could be implemented by delegating to u64's serialized_size.

### vec_u8_serialized_size_external_body
- **Confidence:** medium
- **Reasoning:** `Vec<u8>::serialized_size` is `external_body` with `unimplemented!()` — the exec body panics at runtime. The spec property (length = length-prefix size + data length) is a correct definitional consequence, but the missing exec implementation means the ensures is unverified.

### fold_left_sum_right_external_body_unsound
- **Confidence:** medium
- **Reasoning:** `lemma_seq_fold_left_sum_right` is `external_body` — the fold decomposition property (splitting last element from prefix) is a correct mathematical fact but is entirely trusted without proof. It underpins the Vec<T>::serialized_size loop correctness.

### fold_left_append_len_monotone_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_seq_fold_left_append_len_int_le` is `external_body` — prefix fold monotonicity is trusted without proof. This is used to establish that partial sums don't exceed usize::MAX in the Vec<T>::serialized_size loop. The property is mathematically correct (assuming non-negative lengths) but unverified.

## All Candidates

### φ1: u64_serialized_size_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `u64::serialized_size` is `external_body` with `unimplemented!()` — the exec function never runs correctly at runtime despite the spec ensuring it returns `ghost_serialize().len()`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `u64::serialized_size` is `external_body` with `unimplemented!()` — the exec body panics at runtime while the spec-exec ensures is trusted without verification. The spec property itself (8-byte length) is correct and provable, but the exec implementation is missing.

### φ2: usize_serialized_size_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::serialized_size` is `external_body` with `unimplemented!()` — the ensures `res as int == self.ghost_serialize().len()` is trusted without verification, and the exec body will panic at runtime
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern — `usize::serialized_size` is `external_body` with `unimplemented!()`. The ensures `res as int == self.ghost_serialize().len()` is trusted, and the exec body will panic. Could be implemented by delegating to u64's serialized_size.

### φ3: vec_u8_serialized_size_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<u8>::serialized_size` is `external_body` with `unimplemented!()` — the spec-exec correspondence is entirely trusted, and the exec body panics at runtime
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Vec<u8>::serialized_size` is `external_body` with `unimplemented!()` — the exec body panics at runtime. The spec property (length = length-prefix size + data length) is a correct definitional consequence, but the missing exec implementation means the ensures is unverified.

### φ4: fold_left_sum_right_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_seq_fold_left_sum_right` is `external_body` — the decomposition of fold_left into prefix + last element is entirely trusted, and if incorrect would silently corrupt all Vec<T> serialized_size calculations
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_seq_fold_left_sum_right` is `external_body` — the fold decomposition property (splitting last element from prefix) is a correct mathematical fact but is entirely trusted without proof. It underpins the Vec<T>::serialized_size loop correctness.

### φ5: fold_left_append_len_monotone_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_seq_fold_left_append_len_int_le` is `external_body` — the monotonicity of prefix fold sums is trusted without proof, and if wrong would allow the Vec<T>::serialized_size loop invariant to accept values exceeding usize::MAX
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_seq_fold_left_append_len_int_le` is `external_body` — prefix fold monotonicity is trusted without proof. This is used to establish that partial sums don't exceed usize::MAX in the Vec<T>::serialized_size loop. The property is mathematically correct (assuming non-negative lengths) but unverified.

