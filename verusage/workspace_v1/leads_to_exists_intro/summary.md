# Adversarial Test Summary: `leads_to_exists_intro`

## Target
`leads_to_exists_intro<T, A>(spec, a_to_p, q)`: proves that if each `a_to_p(a)` leads-to `q` under `spec`, then the existential `∃a. a_to_p(a)` also leads-to `q` under `spec`.

## Results

All **10 adversarial tests FAILED verification** as expected, confirming the specification correctly rejects invalid inputs, mutated behaviors, and unintended logical inferences.

### Boundary Tests (3/3 rejected ✅)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_missing_leads_to_precondition` | Calls `leads_to_exists_intro` without establishing `∀a. spec ⊨ a_to_p(a) ~> q` | **precondition error** ✅ |
| `test_boundary_implies_apply_missing_p` | Calls `implies_apply` without `p.satisfied_by(ex)` | **precondition error** ✅ |
| `test_boundary_spec_entails_tla_forall_partial` | Calls `spec_entails_tla_forall` without the universal quantifier (only partial evidence) | **precondition error** ✅ |

### Behavioral Mutation Tests (3/3 rejected ✅)
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_exists_to_forall` | Mutated conclusion from `tla_exists` to `tla_forall` (stronger claim) | **assertion failed** ✅ |
| `test_mutation_reverse_leads_to` | Reversed leads-to direction: `q ~> ∃a.p(a)` instead of `∃a.p(a) ~> q` | **assertion failed** ✅ |
| `test_mutation_leads_to_to_plain_entails` | Dropped temporal structure: asserted `spec ⊨ q` instead of `spec ⊨ (∃a.p(a) ~> q)` | **assertion failed** ✅ |

### Logical Tests (4/4 rejected ✅)
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_leads_to_does_not_imply_always` | `spec ⊨ (p ~> q)` does NOT imply `spec ⊨ □q` | **assertion failed** ✅ |
| `test_logical_leads_to_does_not_strengthen_to_always` | `spec ⊨ (p ~> q)` does NOT imply `spec ⊨ (p ~> □q)` | **assertion failed** ✅ |
| `test_logical_no_existence_from_leads_to` | `∀a. spec ⊨ (p(a) ~> q)` does NOT imply `spec ⊨ ∃a.p(a)` | **assertion failed** ✅ |
| `test_logical_entails_not_symmetric` | `spec ⊨ p` does NOT imply `p ⊨ spec` | **assertion failed** ✅ |

## Conclusion

The specification for `leads_to_exists_intro` and its supporting axioms (`implies_apply`, `tla_forall_leads_to_equality1`, `spec_entails_tla_forall`) are **consistent** with respect to all tested adversarial queries:
- **Preconditions** are enforced: invalid inputs are properly rejected.
- **Behavioral mutations** are detected: incorrect outputs cannot be derived from correct premises.
- **Logical boundaries** are sound: the spec does not entail properties beyond its intended scope.

No specification weaknesses were identified.
