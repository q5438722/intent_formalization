# Adversarial Test Summary

**Target**: `true_pred_on_seq_implies_true_pred_on_filtered_seq.rs`

## Specification Under Test

- **Axiom** (`seq_filter_is_a_subset_of_original_seq`): `s.filter(pred)` is a subset of `s` — every element (and every indexed element) of the filtered sequence is contained in the original.
- **Lemma** (`true_pred_on_seq_implies_true_pred_on_filtered_seq`): If a predicate `pred` holds for all elements of `s`, then `pred` also holds for all elements of `s.filter(filter_pred)`.

## Results

| # | Category | Test | Verdict | Expected |
|---|----------|------|---------|----------|
| B1 | Boundary | Precondition violation (pred fails for some elements) | ❌ FAIL | ❌ FAIL ✅ |
| B2 | Boundary | Non-empty filter from empty sequence | ❌ FAIL | ❌ FAIL ✅ |
| B3 | Boundary | Containment at out-of-bounds index | ❌ FAIL | ❌ FAIL ✅ |
| M1 | Behavioral Mutation | Negate postcondition (∃ e in filter with ¬pred(e)) | ❌ FAIL | ❌ FAIL ✅ |
| M2 | Behavioral Mutation | Reverse postcondition (pred(e) ⟹ e ∈ filter) | ❌ FAIL | ❌ FAIL ✅ |
| M3 | Behavioral Mutation | Filter preserves length | ❌ FAIL | ❌ FAIL ✅ |
| L1 | Logical | Filter is identity (filter(s) =~= s) | ❌ FAIL | ❌ FAIL ✅ |
| L2 | Logical | Reverse subset (s ⊆ filter(s)) | ❌ FAIL | ❌ FAIL ✅ |
| L3 | Logical | Cross-sequence pred leakage | ❌ FAIL | ❌ FAIL ✅ |

**All 9/9 tests correctly rejected.** The specification does not entail any of the tested undesirable properties.

## Analysis

### Boundary Tests
- **B1**: Verus correctly rejects the call when the `requires` clause (`∀e ∈ s. pred(e)`) is not satisfied.
- **B2**: The spec does not allow deriving that an empty sequence has a non-empty filter result.
- **B3**: The axiom's index-based postcondition is properly guarded by `0 <= i < s.filter(pred).len()`, so out-of-bounds indices yield no useful information.

### Behavioral Mutation Tests
- **M1–M2**: The postcondition `∀e ∈ filter(s). pred(e)` is tight — neither its negation nor its converse is entailed.
- **M3**: The spec does not assert any relationship between `s.len()` and `s.filter(p).len()`, correctly preventing the false claim that filtering preserves length.

### Logical Tests
- **L1**: Asserting extensional equality between `s.filter(p)` and `s` with a selective predicate is correctly rejected.
- **L2**: The subset relationship is one-directional (filter ⊆ original); the reverse is not entailed.
- **L3**: Calling the lemma on `s1` does not leak `pred` information to an unrelated sequence `s2` whose elements violate `pred`.

## Notable Finding (During Development)

An initial version of logical test L1 asserted that `∀e ∈ s.filter(p). p(e)` (filtered elements satisfy the filter predicate). This **passed** verification despite NOT being stated in the `external_body` axiom. This is because Verus's **built-in** `Seq::filter` definition already axiomatizes this property internally. The `external_body` axiom's subset guarantee is therefore partially redundant with Verus's standard library. This is not a spec weakness — it reflects a correct layering where the external axiom supplements (but does not override) built-in semantics.

## Conclusion

The specification is **consistent** with respect to all tested properties. It correctly:
- Rejects invalid inputs (precondition enforcement)
- Rejects incorrect output mutations (postcondition tightness)
- Prevents unintended logical inferences (no cross-sequence leakage, no false structural claims)
