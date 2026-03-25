# Adversarial Proof Test Summary: `tla_exists_equality.rs`

## Target Specification

The file defines a TLA-style temporal logic framework with:
- `temp_pred_equality`: axiom (external_body) — bidirectional entailment implies structural equality
- `tla_exists_equality`: proof — `lift_state(|t| ∃a. f(a,t)) == tla_exists(|a| lift_state(|t| f(a,t)))`

## Results: All 9 tests FAILED verification ✅

Every adversarial test was correctly rejected by the specification.

---

### Boundary Tests (`boundary_tests.rs`) — 3/3 rejected

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_one_direction_entailment` | Calls `temp_pred_equality(⊥, ⊤)` — vacuous forward entailment but reverse fails | ❌ precondition `q.entails(p)` violated |
| `test_boundary_no_entailment` | Calls `temp_pred_equality` with disjoint predicates (head>0 vs head<0) | ❌ precondition `p.entails(q)` violated |
| `test_boundary_strict_vs_nonstrict` | Calls `temp_pred_equality` with head≥0 vs head>0 — off-by-one at boundary | ❌ precondition `p.entails(q)` violated |

**Assessment**: The `requires` clauses on `temp_pred_equality` correctly reject all invalid inputs.

---

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3/3 rejected

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_negate_equality` | Asserts `p != q` after `tla_exists_equality` proves `p == q` | ❌ assertion failed |
| `test_mutation_forall_replaces_exists` | Replaces `∃` with `∀` on LHS of the equality | ❌ assertion failed |
| `test_mutation_different_function` | Uses `f1(a,t) = a==t ∧ t>5` on LHS, `f2(a,t) = a==t ∧ t<0` on RHS | ❌ assertion failed |

**Assessment**: The specification correctly rejects mutated outputs — negated results, quantifier mutations, and function mismatches are all caught.

---

### Logical Tests (`logical_tests.rs`) — 3/3 rejected

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_quantifier_swap` | Claims `∃a. ∀ex. a == ex.head()` (swapped quantifier order) | ❌ assertion failed |
| `test_logical_wrong_monotonicity` | Claims `tla_exists(f_weak)` entails `tla_exists(f_strong)` | ❌ assertion failed |
| `test_logical_tla_exists_over_false` | Claims `valid(tla_exists(|a| lift_state(|t| false)))` | ❌ assertion failed |

**Assessment**: The specification does not admit unintended logical inferences — quantifier swaps, wrong monotonicity, and vacuous existentials are all rejected.

---

## Conclusion

The specification for `tla_exists_equality` is **consistent** with respect to all 9 adversarial queries tested:
- **Boundary**: preconditions correctly guard `temp_pred_equality`
- **Behavioral**: postconditions are precise enough to reject mutated outputs
- **Logical**: the spec does not entail unwarranted properties (quantifier swaps, wrong monotonicity, false existentials)

No specification weaknesses were detected.
