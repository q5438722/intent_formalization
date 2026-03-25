# Adversarial Test Summary: `always_p_or_eventually_q.rs`

## Target Specification

The main theorem `always_p_or_eventually_q` proves a temporal logic property:
> If at every step `p ∧ next → later(p) ∨ later(q)`, and `next` always holds,
> then at every step `p → always(p) ∨ eventually(q)`.

It relies on 7 trusted (`external_body`) lemmas as axioms.

---

## Results: 14/14 tests FAILED verification ✅

All adversarial queries were correctly rejected by the specification.

### Boundary Tests (5/5 failed) — `boundary_tests.rs`

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| `test_boundary_missing_always_next` | Missing `always(next)` precondition | precondition not satisfied |
| `test_boundary_missing_transition` | Missing transition property precondition | precondition not satisfied |
| `test_boundary_always_unfold_without_always` | `always_unfold` called with only `p`, not `always(p)` | precondition not satisfied |
| `test_boundary_implies_apply_missing_antecedent` | `implies_apply` called without antecedent `p` | precondition not satisfied |
| `test_boundary_propagate_without_always` | `always_propagate_forwards` called without `always(p)` | precondition not satisfied |

### Behavioral Mutation Tests (4/4 failed) — `behavioral_mutation_tests.rs`

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `test_mutation_and_instead_of_or` | Strengthened OR to AND in conclusion | postcondition not satisfied |
| `test_mutation_always_p_unconditional` | Claimed `always(p)` unconditionally | postcondition not satisfied |
| `test_mutation_eventually_q_unconditional` | Claimed `eventually(q)` unconditionally | postcondition not satisfied |
| `test_mutation_swap_p_q_in_conclusion` | Swapped p and q roles in conclusion | postcondition not satisfied |

### Logical Tests (5/5 failed) — `logical_tests.rs`

| Test | Unintended Property Tested | Verus Error |
|------|---------------------------|-------------|
| `test_logical_always_implies_eventually` | `always(p) → eventually(q)` for unrelated p, q | postcondition not satisfied |
| `test_logical_always_q_from_premises` | Deriving `always(q)` from premises | postcondition not satisfied |
| `test_logical_partial_execution_equality` | `execution_equality` from single-point agreement | precondition not satisfied |
| `test_logical_wrong_execution` | Applying result to a different execution | postcondition not satisfied |
| `test_logical_rec_without_not_q` | Induction helper without ¬q constraint | precondition not satisfied |

---

## Conclusion

The specification is **consistent** with respect to all 14 adversarial queries:
- **Preconditions are tight**: every `requires` clause is necessary — removing any one causes verification failure.
- **Postconditions are precise**: the conclusion cannot be strengthened (AND vs OR), unconditionally asserted, or transferred to unrelated predicates/executions.
- **No unintended entailments**: the spec does not support cross-predicate reasoning, partial equality abuse, or constraint relaxation.
