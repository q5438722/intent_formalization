# Test Summary: `implies_apply_with_always`

## Target Specification
**□(p→q) ∧ □p → □q** — temporal logic modus ponens under the `always` operator.

## Results: All 15 adversarial tests FAIL verification ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (5/5 FAIL ✅)

| Test | What it probes | Failure type |
|------|---------------|--------------|
| `boundary_test_missing_implication` | Missing □(p→q), only □p given | precondition |
| `boundary_test_missing_antecedent` | Missing □p, only □(p→q) given | precondition |
| `boundary_test_non_always_implication` | (p→q) at one state, not □(p→q) | precondition |
| `boundary_test_non_always_p` | p at one state, not □p | precondition |
| `boundary_test_no_preconditions` | No preconditions at all | precondition |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | What it probes | Failure type |
|------|---------------|--------------|
| `mutation_test_global_equivalence` | □(p→q) ∧ □p ⊬ ∀ex. p(ex)↔q(ex) | postcondition |
| `mutation_test_arbitrary_conclusion` | □(p→q) ∧ □p ⊬ □r (r unrelated) | postcondition |
| `mutation_test_reversed_roles` | □(p→q) ∧ □q ⊬ □p | postcondition |
| `mutation_test_swapped_implication` | □(q→p) ∧ □p ⊬ □q | postcondition |
| `mutation_test_weaken_conclusion` | □(p→q) ∧ □p ⊬ □(p→r) | postcondition |

### Logical Tests (5/5 FAIL ✅)

| Test | What it probes | Failure type |
|------|---------------|--------------|
| `logical_test_transitivity_without_base` | □(p→q) ∧ □(q→r) ⊬ □r (no □p) | precondition |
| `logical_test_execution_transfer` | □q on ex₁ ⊬ □q on ex₂ | postcondition |
| `logical_test_converse_without_base` | □(p→q) ⊬ □(q→p) | postcondition |
| `logical_test_unfold_non_always` | p(ex) ⊬ ∀i. p(ex.suffix(i)) | precondition |
| `logical_test_arbitrary_predicate_always` | □p ⊬ □q (q unrelated) | postcondition |

---

### Notable Finding
An initial test for □(q→p) from □(p→q) ∧ □p **passed verification** — this is logically valid because □p makes the consequent of q→p always true. The test was replaced with `mutation_test_global_equivalence` (universal predicate equivalence) which correctly fails.

### Conclusion
The specification for `implies_apply_with_always` is **tight**: it requires both preconditions (□(p→q) and □p), rejects all tested output mutations, and does not admit unintended logical inferences. No spec weaknesses were found.
