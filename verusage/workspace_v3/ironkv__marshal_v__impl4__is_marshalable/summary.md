# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_v__impl4__is_marshalable/original.rs`
**Date:** 2026-03-25 04:06:06
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate property exposes a real platform-dependent spec weakness. Because Verus models `usize` as fitting within `u64::MAX` on 64-bit architectures, the `is_marshalable` constraint for `usize` is always trivially true, which makes the `forall` element check in `Vec<T>::is_marshalable` vacuous when `T = usize`. The entailment succeeds only because the precondition is contradictory—not because the spec genuinely permits non-marshalable elements. This is the same usize platform assumption previously identified in the marshal_v analysis.

## True Positives (Spec Issues)

### non_marshalable_element_ignored
- **Confidence:** high
- **Reasoning:** The `usize::is_marshalable` spec requires `*self as int <= u64::MAX`, but on 64-bit platforms (Verus's default model) no `usize` value can exceed `u64::MAX`, making the precondition `!v@[i].is_marshalable()` unsatisfiable. The proof is vacuously true, revealing that the per-element marshalability check for `Vec<usize>` is dead—it can never reject an element. This is a genuine spec gap: the `usize` marshalability guard is platform-dependent and provides no real protection on 64-bit targets.

## All Candidates

### φ1: empty_vec_not_marshalable
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty vector should trivially be marshalable; if the spec says otherwise, element iteration or base-case logic is wrong

### φ2: non_marshalable_element_ignored
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A vector containing a non-marshalable element must not be marshalable; if entailed, the per-element check is vacuous
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `usize::is_marshalable` spec requires `*self as int <= u64::MAX`, but on 64-bit platforms (Verus's default model) no `usize` value can exceed `u64::MAX`, making the precondition `!v@[i].is_marshalable()` unsatisfiable. The proof is vacuously true, revealing that the per-element marshalability check for `Vec<usize>` is dead—it can never reject an element. This is a genuine spec gap: the `usize` marshalability guard is platform-dependent and provides no real protection on 64-bit targets.

### φ3: marshalable_serialization_exceeds_max
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A marshalable vector's serialized size must fit in usize::MAX; exceeding it contradicts the size-bound invariant

### φ4: huge_vec_always_marshalable
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A Vec<u64> with length exceeding usize::MAX/8 would serialize to more than usize::MAX bytes; being marshalable would mean overflow is undetected

### φ5: marshalable_implies_element_not_marshalable
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a vector is marshalable then every element must individually be marshalable; the negation would indicate the forall quantifier in the spec is inverted

