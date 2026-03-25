# Adversarial Test Summary: `verus_extra__map_fold_finite`

## Target Specification

```rust
proof fn map_fold_finite<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    requires s.finite()
    ensures map_fold(s, f).finite()
```

The specification guarantees that applying `map_fold` (image of a set under a function) to a finite set produces a finite set. `map_fold` is opaque (not `open`), so only the `ensures` clause is available for reasoning.

---

## Results: All 9 tests FAILED verification ✅

Every adversarial test was correctly rejected, meaning the specification does not entail any of the undesirable properties tested.

### Boundary Tests (3/3 failed as expected)

| Test | Property Probed | Failure Mode | Result |
|------|----------------|--------------|--------|
| `test_missing_finite_precondition` | Call without `s.finite()` | Precondition not satisfied | ✅ FAIL |
| `test_postcondition_wrong_set` | Prove for `s1`, claim for `s2` | Postcondition not satisfied | ✅ FAIL |
| `test_stronger_than_postcondition` | Claim `len() == 0` for non-empty input | Postcondition not satisfied | ✅ FAIL |

**Analysis**: The precondition `s.finite()` is correctly enforced. The postcondition is scoped precisely to the proven set and does not leak stronger guarantees.

### Behavioral Mutation Tests (3/3 failed as expected)

| Test | Property Probed | Failure Mode | Result |
|------|----------------|--------------|--------|
| `test_negated_postcondition` | `!map_fold(s, f).finite()` | Postcondition not satisfied | ✅ FAIL |
| `test_wrong_cardinality_mutation` | `len() == s.len() + 1` | Postcondition not satisfied | ✅ FAIL |
| `test_nonempty_maps_to_empty` | `map_fold(s, f) =~= Set::empty()` | Postcondition not satisfied | ✅ FAIL |

**Analysis**: Negating the postcondition is correctly rejected (the spec is not vacuously true). Mutated cardinality and wrong-value claims are also rejected. The opaqueness of `map_fold` prevents deriving any concrete structural information beyond finiteness.

### Logical Tests (3/3 failed as expected)

| Test | Property Probed | Failure Mode | Result |
|------|----------------|--------------|--------|
| `test_cardinality_preservation` | `map_fold(s, f).len() == s.len()` | Postcondition not satisfied | ✅ FAIL |
| `test_element_containment` | `s.contains(x) ⟹ map_fold(s, f).contains(f(x))` | Postcondition not satisfied | ✅ FAIL |
| `test_distributivity_over_union` | `map_fold(s1 ∪ s2, f) =~= map_fold(s1, f) ∪ map_fold(s2, f)` | Postcondition not satisfied | ✅ FAIL |

**Analysis**: The specification correctly does not entail cardinality preservation (which is false in general when `f` is not injective), element containment, or distributivity over union. These are properties that would require separate proofs with additional lemmas or an `open` definition of `map_fold`.

---

## Specification Assessment

The specification `map_fold_finite` is **tight within its scope**:

- **Precondition**: `s.finite()` is strictly enforced and cannot be bypassed.
- **Postcondition**: Only `map_fold(s, f).finite()` is derivable — no unintended structural, cardinality, or element-membership properties leak.
- **Opacity**: `map_fold` being opaque (`spec fn` without `open`) provides strong information hiding, preventing the SMT solver from deriving anything beyond what the proof function explicitly guarantees.

**Note**: The opacity of `map_fold` means the specification is intentionally minimal. Properties like element containment (`s.contains(x) ⟹ map_fold(s, f).contains(f(x))`) are *semantically true* but not entailed — they would need separate lemmas. Whether this incompleteness is intentional depends on the broader usage context.
