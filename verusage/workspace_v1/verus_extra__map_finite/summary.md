# Adversarial Test Summary: `verus_extra__map_finite`

## Target Specification

- **`map_finite<A, B>`**: `requires s.finite()` → `ensures s.map(f).finite()`
- Helper axioms: `map_fold_ok` (equivalence) and `map_fold_finite` (finiteness of fold)

## Results: ALL 9 tests FAILED verification ✅ (as expected)

### Boundary Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `boundary_test_infinite_set` | Call `map_finite` with `Set::new(\|i\| true)` | ❌ Precondition `s.finite()` rejected |
| `boundary_test_predicate_set_not_provably_finite` | Call `map_finite` with `Set::new(\|i\| 0<=i && i<10)` | ❌ Precondition `s.finite()` rejected |
| `boundary_test_map_fold_finite_infinite` | Call `map_fold_finite` with `Set::new(\|i\| i>=0)` | ❌ Precondition `s.finite()` rejected |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `mutation_negate_finiteness` | Assert `!s.map(f).finite()` after `map_finite` | ❌ Assertion failed |
| `mutation_map_fold_not_equal_to_map` | Assert `!(map_fold(s,f) =~= s.map(f))` after `map_fold_ok` | ❌ Assertion failed |
| `mutation_map_fold_not_finite` | Assert `!map_fold(s,f).finite()` after `map_fold_finite` | ❌ Assertion failed |

### Logical Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `logical_cardinality_not_preserved` | `s.map(f).len() == s.len()` for non-injective f | ❌ Assertion failed |
| `logical_cross_function_equivalence` | `s.map(f) =~= s.map(g)` for f≠g | ❌ Assertion failed |
| `logical_finiteness_not_transferable` | `s2.map(g).finite()` for infinite s2 after proving finiteness for unrelated s1 | ❌ Assertion failed |

## Conclusion

The specification is **well-calibrated**:
- **Preconditions** correctly guard against infinite/non-provably-finite inputs.
- **Postconditions** are tight enough to reject negated outputs.
- **No unintended entailments** detected: cardinality preservation, cross-function equivalence, and finiteness transfer are all correctly rejected.

No spec weaknesses found within the tested semantic boundary.
