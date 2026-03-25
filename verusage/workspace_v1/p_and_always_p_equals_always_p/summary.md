# Adversarial Test Summary: `p_and_always_p_equals_always_p`

## Target Specification

The target proves `p.and(always(p)) == always(p)` using two axioms:
- `always_to_current`: `always(p).satisfied_by(ex) ⟹ p.satisfied_by(ex)`
- `temp_pred_equality`: mutual entailment implies equality

## Results

**All 12 adversarial tests FAILED verification as expected** — the specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (4/4 failed ✓)

| Test | Property Challenged | Result |
|------|-------------------|--------|
| `test_boundary_p_implies_always_p` | `p` at one point ⟹ `always(p)` | FAIL ✓ |
| `test_boundary_always_wrong_predicate` | `always(p)` ⟹ arbitrary `q` | FAIL ✓ |
| `test_boundary_suffix_confusion` | `suffix(2)` ⟹ `suffix(1)` | FAIL ✓ |
| `test_boundary_always_from_nothing` | `always(p)` from no assumptions | FAIL ✓ |

### Behavioral Mutation Tests (4/4 failed ✓)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| `test_mutation_and_always_equals_p` | `p.and(always(p)) == p` (wrong RHS) | FAIL ✓ |
| `test_mutation_always_equals_p` | `always(p) == p` (drop always) | FAIL ✓ |
| `test_mutation_conjunction_from_p_only` | `p ⟹ p.and(always(p))` (missing always) | FAIL ✓ |
| `test_mutation_always_to_shifted_conjunction` | `p` at suffix(1) ⟹ conjunction at suffix(1) | FAIL ✓ |

### Logical Tests (4/4 failed ✓)

| Test | Unintended Property | Result |
|------|-------------------|--------|
| `test_logical_p_entails_always_p` | `p` entails `always(p)` | FAIL ✓ |
| `test_logical_arbitrary_valid` | Any `p` is universally valid | FAIL ✓ |
| `test_logical_p_equals_always_p` | `p == always(p)` (stronger than theorem) | FAIL ✓ |
| `test_logical_false_equality` | Arbitrary `p == q` (axiom misuse) | FAIL ✓ |

## Conclusion

The specification is **well-bounded**: it does not entail any of the 12 adversarial properties tested. The axioms (`always_to_current`, `temp_pred_equality`) are not overpowered — they cannot be exploited to derive false equalities, invalid temporal inferences, or unsound reasoning from insufficient preconditions.
