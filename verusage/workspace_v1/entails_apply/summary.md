# Adversarial Proof Test Summary: `entails_apply.rs`

## Target Specification

The file defines a temporal logic framework with `TempPred<T>` (temporal predicates over executions) and two proof functions:
- **`implies_apply`** (axiom): modus ponens at a single execution — `p ⟹ q` satisfied at `ex` ∧ `p` at `ex` → `q` at `ex`
- **`entails_apply`** (proved): universal modus ponens — `p` entails `q` ∧ `p` at `ex` → `q` at `ex`

## Results: All 15 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (5/5 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_missing_entails` | Missing `p.entails(q)` precondition | ✅ Rejected |
| `boundary_test_missing_satisfaction` | Missing `p.satisfied_by(ex)` precondition | ✅ Rejected |
| `boundary_test_implies_missing_implication` | Missing `p.implies(q).satisfied_by(ex)` | ✅ Rejected |
| `boundary_test_implies_missing_satisfaction` | Missing `p.satisfied_by(ex)` for `implies_apply` | ✅ Rejected |
| `boundary_test_unsatisfiable_predicate` | Always-false predicate (contradictory preconditions) | ✅ Rejected |

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `mutation_test_entails_implies_valid` | `q.satisfied_by(ex)` → `valid(q)` (universally valid) | ✅ Rejected |
| `mutation_test_negated_conclusion` | `q.satisfied_by(ex)` → `!q.satisfied_by(ex)` | ✅ Rejected |
| `mutation_test_reversed_entailment` | Conclude `q.entails(p)` instead of `q.satisfied_by(ex)` | ✅ Rejected |
| `mutation_test_implies_to_valid` | Strengthen `implies_apply` postcondition to `valid(q)` | ✅ Rejected |
| `mutation_test_satisfaction_transfers` | Conclude `q.satisfied_by(ex2)` for a different execution | ✅ Rejected |

### Logical Tests (5/5 failed ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `logical_test_entails_symmetry` | Entailment is symmetric | ✅ Rejected |
| `logical_test_satisfaction_implies_validity` | Single satisfaction → universal validity | ✅ Rejected |
| `logical_test_entails_without_satisfaction` | Entailment alone → satisfaction | ✅ Rejected |
| `logical_test_entails_implies_valid_q` | `p.entails(q)` → `valid(q)` without `valid(p)` | ✅ Rejected |
| `logical_test_satisfaction_not_transferable` | Satisfaction transfers between executions | ✅ Rejected |

---

## Conclusion

The specification for `entails_apply` and `implies_apply` is **well-constrained**:
- **Preconditions are tight**: No invalid inputs are accepted.
- **Postconditions are precise**: No incorrect behavioral mutations pass.
- **Logical boundaries are sound**: No unintended properties (symmetry, transferability, universalization) are derivable.

No specification weaknesses were identified.
