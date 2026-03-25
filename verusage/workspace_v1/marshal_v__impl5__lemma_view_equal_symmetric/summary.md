# Test Summary: `marshal_v__impl5__lemma_view_equal_symmetric`

## Target Specification

- **Trait** `Marshalable`: abstract `view_equal` + axiom `lemma_view_equal_symmetric` (symmetry: `self.view_equal(other) == other.view_equal(self)`)
- **Tuple impl** `(T, U)`: `view_equal` = conjunction of both components; proof delegates to component lemmas

## Results Overview

| Category | Tests | Failed (as expected) | Unexpected Passes |
|---|---|---|---|
| Boundary | 5 | 5 | 0 |
| Behavioral Mutation | 5 | 5 | 0 |
| Logical | 6 | 6 | 0 |
| **Total** | **16** | **16** | **0** |

All 16 adversarial tests were correctly **rejected** by the verifier.

## Boundary Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|---|---|---|
| `test_boundary_tuple_partial_first_only` | Tuple view_equal with only 1st component matching | FAIL ✅ |
| `test_boundary_tuple_partial_second_only` | Tuple view_equal with only 2nd component matching | FAIL ✅ |
| `test_boundary_tuple_neither_component` | Tuple view_equal with neither component matching | FAIL ✅ |
| `test_boundary_arbitrary_always_equal` | Arbitrary values always view_equal | FAIL ✅ |
| `test_boundary_arbitrary_always_unequal` | Arbitrary values always NOT view_equal | FAIL ✅ |

## Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|---|---|---|
| `test_mutation_symmetry_negated` | Negate symmetry: a~=b ⇒ ¬(b~=a) | FAIL ✅ |
| `test_mutation_anti_symmetric` | Assert anti-symmetry alongside symmetry | FAIL ✅ |
| `test_mutation_tuple_equal_negated` | Negate tuple view_equal when components match | FAIL ✅ |
| `test_mutation_tuple_symmetry_negated` | Negate tuple-level symmetry | FAIL ✅ |
| `test_mutation_components_true_tuple_false` | Components true ⇒ tuple false OR component false | FAIL ✅ |

## Logical Tests (6/6 FAIL ✅)

| Test | Property Queried | Result |
|---|---|---|
| `test_logical_reflexivity` | Reflexivity: a~=a | FAIL ✅ |
| `test_logical_transitivity` | Transitivity: a~=b ∧ b~=c ⇒ a~=c | FAIL ✅ |
| `test_logical_view_equal_implies_eq` | view_equal ⇒ structural equality (==) | FAIL ✅ |
| `test_logical_common_neighbor` | a~=b ∧ a~=c ⇒ b~=c | FAIL ✅ |
| `test_logical_tuple_reflexivity` | Tuple reflexivity: (a,b)~=(a,b) | FAIL ✅ |
| `test_logical_tuple_view_equal_implies_component_eq` | Tuple view_equal ⇒ component structural equality | FAIL ✅ |

## Conclusion

The specification is **well-scoped** for its stated purpose. It correctly:

1. **Rejects invalid inputs**: Partial component matches and arbitrary claims are rejected (boundary).
2. **Rejects incorrect behaviors**: Negations and mutations of the symmetry property and tuple conjunction semantics are rejected (behavioral mutation).
3. **Does not over-entail**: Properties not guaranteed by symmetry alone (reflexivity, transitivity, structural equality, equivalence closure) are correctly unprovable (logical).

**No spec weaknesses detected** — the specification entails exactly symmetry of `view_equal` and the conjunctive semantics of tuple `view_equal`, nothing more.
