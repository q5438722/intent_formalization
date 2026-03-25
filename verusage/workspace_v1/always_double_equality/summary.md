## Adversarial Test Summary — `always_double_equality.rs`

### Target Specification
Proves `always(always(p)) == always(p)` using two axioms:
- `execution_equality`: pointwise-equal executions are structurally equal
- `temp_pred_equality`: mutually-entailing predicates are equal

---

### Results: All 9 tests correctly FAILED verification ✅

| # | File | Test | Type | Failure Mode | Result |
|---|------|------|------|-------------|--------|
| 1 | `boundary_tests.rs` | `test_execution_equality_no_precondition` | Boundary | `execution_equality` called without pointwise equality precondition | ✅ FAIL (precondition not satisfied) |
| 2 | `boundary_tests.rs` | `test_temp_pred_equality_one_direction` | Boundary | `temp_pred_equality` called with only one entailment direction | ✅ FAIL (precondition not satisfied) |
| 3 | `boundary_tests.rs` | `test_suffix_composition_multiply` | Boundary | Suffix composition with `i*j` instead of correct `i+j` | ✅ FAIL (precondition not satisfied) |
| 4 | `behavioral_mutation_tests.rs` | `test_always_over_collapse` | Mutation | Mutated conclusion to `always(always(p)) == p` (over-collapse) | ✅ FAIL (postcondition not satisfied) |
| 5 | `behavioral_mutation_tests.rs` | `test_always_is_identity` | Mutation | Mutated `always` to be the identity: `always(p) == p` | ✅ FAIL (postcondition not satisfied) |
| 6 | `behavioral_mutation_tests.rs` | `test_always_ignores_argument` | Mutation | Mutated `always` to ignore input: `always(p) == always(q)` | ✅ FAIL (postcondition not satisfied) |
| 7 | `logical_tests.rs` | `test_p_entails_always_p` | Logical | Attempted promotion: `p ⊨ □p` (converse of always-elim) | ✅ FAIL (postcondition not satisfied) |
| 8 | `logical_tests.rs` | `test_suffix_injectivity` | Logical | Claimed suffix is injective on positions | ✅ FAIL (postcondition not satisfied) |
| 9 | `logical_tests.rs` | `test_always_valid_for_any` | Logical | Claimed `valid(□p)` for arbitrary `p` | ✅ FAIL (postcondition not satisfied) |

---

### Interpretation

The specification is **consistent** with respect to all tested semantic queries:

- **Boundary**: The axiom preconditions are precise — invalid inputs (missing pointwise equality, one-directional entailment, wrong arithmetic on suffix positions) are correctly rejected.
- **Behavioral**: Mutations to the theorem's conclusion (over-collapsing `always(always(p))` to `p`, treating `always` as identity, or treating `always` as a constant function) are all rejected.
- **Logical**: Unintended inferences (promoting `p` to `always(p)`, suffix injectivity, universal validity of arbitrary predicates) are correctly disallowed.

No specification weaknesses were detected.
