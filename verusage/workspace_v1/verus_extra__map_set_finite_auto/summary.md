# Test Summary: `verus_extra__map_set_finite_auto`

## Target Specification

Two proof functions:
- `map_finite(s, f)`: **requires** `s.finite()`, **ensures** `s.map(f).finite()` (external axiom)
- `map_set_finite_auto()`: **ensures** `∀ s, f. s.finite() ⟹ s.map(f).finite()` (derived)

## Results Overview

| Category | Tests | Failed (as expected) | Passed (spec weakness) |
|---|---|---|---|
| Boundary | 4 | 4 | 0 |
| Behavioral Mutation | 4 | 4 | 0 |
| Logical | 4 | 4 | 0 |
| **Total** | **12** | **12** | **0** |

**All 12 adversarial tests were correctly rejected by the verifier.**

---

## Boundary Tests (4/4 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_boundary_no_finite_precondition` | Call `map_finite` on `Set::full()` without finiteness | FAIL ✓ — precondition not satisfied |
| 2 | `test_boundary_infinite_set_map_finite` | Use `map_set_finite_auto` then assert finiteness on `Set::full().map(f)` | FAIL ✓ — assertion failed |
| 3 | `test_boundary_complement_singleton_finite` | Assert finiteness on `Set::full().remove(0).map(f)` | FAIL ✓ — assertion failed |
| 4 | `test_boundary_constant_fn_infinite_set` | Constant function on infinite set — precondition still required | FAIL ✓ — precondition not satisfied |

## Behavioral Mutation Tests (4/4 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_mutation_negate_postcondition` | Assert `!s.map(f).finite()` on a finite set | FAIL ✓ — assertion failed |
| 2 | `test_mutation_map_result_is_empty` | Assert `s.map(id) == ∅` for non-empty `s` | FAIL ✓ — assertion failed |
| 3 | `test_mutation_map_equals_original` | Assert `s.map(f) == s` for non-identity `f` | FAIL ✓ — assertion failed |
| 4 | `test_mutation_swap_function` | Establish finiteness with `f`, assert equality with `g ≠ f` | FAIL ✓ — assertion failed |

## Logical Tests (4/4 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_logical_converse_finite` | Converse: `s.map(f).finite() ⟹ s.finite()` | FAIL ✓ — assertion failed |
| 2 | `test_logical_cardinality_preserved` | Cardinality: `s.map(f).len() == s.len()` (non-injective `f`) | FAIL ✓ — assertion failed |
| 3 | `test_logical_map_injective` | Injectivity: `s1.map(f) == s2.map(f) ⟹ s1 == s2` | FAIL ✓ — assertion failed |
| 4 | `test_logical_finiteness_transfers_across_functions` | Cross-function: `s.map(f).finite() ⟹ s.map(g).finite()` | FAIL ✓ — assertion failed |

## Conclusion

The specification for `map_finite` / `map_set_finite_auto` is **consistent** with respect to all 12 adversarial queries:

- **Boundary**: Invalid inputs (infinite sets) are properly rejected by the precondition.
- **Behavioral**: Incorrect postconditions (negation, wrong result sets, function swaps) are rejected.
- **Logical**: Unentailed properties (converse, cardinality preservation, map injectivity, cross-function transfer) are correctly not derivable.

No spec weaknesses were detected.
