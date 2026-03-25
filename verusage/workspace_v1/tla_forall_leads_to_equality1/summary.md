# Adversarial Test Summary: `tla_forall_leads_to_equality1`

## Target Specification

The target proves that universal quantification distributes over `leads_to`:
```
∀a.(p(a) ~> q) == (∃a.p(a)) ~> q
```
using two external-body axioms: `tla_forall_always_equality_variant` (with precondition) and `tla_forall_implies_equality1` (unconditional).

---

## Results: All 10 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

### Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_contradictory_predicates` | Call axiom with `always(true)` vs `always(false)` — precondition violated | FAIL ✅ |
| `boundary_test_eventually_instead_of_always` | Pass `eventually(p)` where `always(p)` required — precondition violated | FAIL ✅ |
| `boundary_test_missing_always_wrapper` | Pass bare `p` where `always(p)` required — precondition violated | FAIL ✅ |

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `mutation_test_forall_replaces_exists_rhs` | RHS: `tla_exists(p)` → `tla_forall(p)` | FAIL ✅ |
| `mutation_test_exists_replaces_forall_lhs` | LHS: `tla_forall(…)` → `tla_exists(…)` | FAIL ✅ |
| `mutation_test_implies_replaces_leads_to_rhs` | RHS operator: `leads_to` → `implies` | FAIL ✅ |

### Logical Tests (4/4 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `logical_test_forall_equals_exists` | `∀ == ∃` (quantifier collapse) | FAIL ✅ |
| `logical_test_eventually_entails_always` | `◇p ⊨ □p` (stronger inequality) | FAIL ✅ |
| `logical_test_leads_to_symmetric` | `p ~> q == q ~> p` (symmetry) | FAIL ✅ |
| `logical_test_axiom_consistency` | `false` derivable from axioms (soundness) | FAIL ✅ |

---

## Conclusion

The specification is **tight**: it correctly rejects invalid preconditions (boundary), mutated postconditions (behavioral), and unintended logical consequences (logical). The axiom system is consistent (cannot derive `false`). No weaknesses detected.
