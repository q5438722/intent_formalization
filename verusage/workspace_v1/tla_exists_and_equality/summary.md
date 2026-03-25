# Adversarial Test Summary: `tla_exists_and_equality`

## Target
`source-projects/anvil-library/verified/temporal_logic/tla_exists_and_equality.rs`

Functions tested: `tla_exists_proved_by_witness`, `temp_pred_equality`, `tla_exists_and_equality`, `tla_exists`, `valid`, `entails`

---

## Results: All 9 tests FAILED verification ✅

All adversarial queries were correctly rejected by the specification.

### Boundary Tests (3/3 failed as expected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_invalid_witness` | Calls `tla_exists_proved_by_witness` with witness=0 where `a_to_p(0)` is always-false | ✅ precondition rejected |
| `test_boundary_one_direction_entailment` | Calls `temp_pred_equality` with only p→q (always-false → always-true), missing q→p | ✅ precondition rejected |
| `test_boundary_contradictory_predicates` | Calls `temp_pred_equality` with always-true vs always-false (p→q fails) | ✅ precondition rejected |

### Behavioral Mutation Tests (3/3 failed as expected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_drop_and_q` | Asserts `∃a. P(a)∧Q == ∃a. P(a)` — drops `.and(q)` from RHS | ✅ assertion rejected |
| `test_mutation_reverse_witness` | From `∃a. P(a)` satisfied, concludes `P(0)` satisfied for specific a=0 | ✅ assertion rejected |
| `test_mutation_strengthen_to_valid` | From existential at one execution, concludes validity for all executions | ✅ assertion rejected |

### Logical Tests (3/3 failed as expected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_entails_not_symmetric` | Asserts `q.entails(p)` from only `p.entails(q)` — entailment is not symmetric | ✅ assertion rejected |
| `test_logical_single_satisfied_not_valid` | Asserts `valid(∃a.P(a))` from satisfaction at a single execution | ✅ assertion rejected |
| `test_logical_equality_without_lemma` | Asserts `p == q` from mutual entailment without calling `temp_pred_equality` axiom | ✅ assertion rejected |

---

## Conclusion

The specification is **consistent** with respect to all tested adversarial queries:
- **Boundary**: Invalid inputs to proof functions are properly guarded by preconditions.
- **Behavioral**: Mutated outputs (dropped conjuncts, reversed witnesses, strengthened conclusions) are all rejected.
- **Logical**: Unintended inferences (symmetry of entailment, single-exec-to-valid lifting, equality without axiom) are not derivable.

No specification weaknesses were detected.
