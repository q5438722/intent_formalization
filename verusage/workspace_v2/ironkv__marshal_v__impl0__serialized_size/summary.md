# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl0__serialized_size.rs`
**Date:** 2026-03-24T09:27:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

Four of five candidates are false positives: fixed 8-byte u64 encoding is intentional, the serialization-fits-in-usize concern is trivially satisfied for u64, the usize::MAX marshalability biconditional is tautological, and the non-marshalable serialization collapse involves practically unsatisfiable preconditions. The one true positive is that `usize::serialized_size` is unnecessarily `external_body` — it could be verified like the u64 version since the serialization length is provably 8, but instead leaves an unverified trust assumption.

## True Positives (Spec Issues)

### usize_serialized_size_external_body_unchecked
- **Confidence:** medium
- **Reasoning:** `usize::serialized_size` is `external_body` with an `unimplemented!()` body, so its ensures clause (`res as int == self.ghost_serialize().len()`) is trusted without verification. Since the serialization length is provably always 8, this could be implemented and verified like the u64 version, but instead it's left as an unverified trust assumption.

## All Candidates

### φ1: u64_serialize_len_always_8
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Every u64 serializes to exactly 8 bytes regardless of value — small values like 0 waste 7 bytes, suggesting the format lacks any compact encoding
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Fixed-width 8-byte LE encoding for u64 is a standard, intentional design choice. This is the expected behavior of `spec_u64_to_le_bytes`, not a spec gap.

### φ2: usize_serialized_size_external_body_unchecked
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::serialized_size` is `external_body` but the serialization length is always 8 (same as u64) — the unverified exec implementation could return any value, breaking the ensures `res as int == ghost_serialize().len()` without detection
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `usize::serialized_size` is `external_body` with an `unimplemented!()` body, so its ensures clause (`res as int == self.ghost_serialize().len()`) is trusted without verification. Since the serialization length is provably always 8, this could be implemented and verified like the u64 version, but instead it's left as an unverified trust assumption.

### φ3: serialized_size_trait_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The trait's `serialized_size` returns `usize` and ensures `res as int == ghost_serialize().len()`, implicitly assuming serialized length fits in usize — but `ghost_serialize` at the trait level is external_body with no length bound, so future impls could have serializations exceeding usize::MAX
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For u64, the serialized length is provably 8, which trivially fits in usize. The property demonstrated here is a concrete fact about u64, not a gap. The theoretical concern about future impls exceeding usize::MAX is not a current spec issue.

### φ4: usize_max_is_marshalable
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Whether usize::MAX is marshalable depends on the platform — on 64-bit systems `usize::MAX == u64::MAX` so all usize values are marshalable, but the spec doesn't constrain the platform, creating a platform-dependent marshalability gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The biconditional `(usize::MAX).is_marshalable() <==> (usize::MAX as int <= u64::MAX)` is a direct tautological restatement of the `is_marshalable` definition. Platform-dependent behavior is the intentional design — the spec correctly captures the constraint.

### φ5: ghost_serialize_recommends_not_requires
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** For non-marshalable usize values (above u64::MAX), `ghost_serialize` delegates to `(*self as u64).ghost_serialize()` which truncates — distinct non-marshalable values could serialize identically, collapsing the serialization's injectivity without any warning since `is_marshalable` is only a `recommends`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This PHI's precondition requires `!(a.is_marshalable())` for usize, meaning `a as int > u64::MAX`. On any real platform, `usize` cannot exceed `u64::MAX` on 64-bit (and is smaller on 32-bit), so the precondition is unsatisfiable in practice. The `recommends` vs `requires` distinction is intentional — non-marshalable inputs are outside the specified domain.

