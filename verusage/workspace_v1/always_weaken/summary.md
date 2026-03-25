# Adversarial Test Summary: `always_weaken.rs`

## Target Specification

The `always_weaken` lemma proves: if `valid(p → q)` and `spec ⊨ □p`, then `spec ⊨ □q`.
It relies on two trusted axioms: `implies_apply` (modus ponens) and `entails_preserved_by_always` (□-monotonicity).

## Results: All 9 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no inconsistencies found.

| # | Category | Test | Failure Mode | Result |
|---|----------|------|-------------|--------|
| 1 | Boundary | `boundary_test_1_missing_implies` | Missing `valid(p→q)` precondition | ❌ precondition not satisfied |
| 2 | Boundary | `boundary_test_2_missing_entails` | Missing `spec⊨□p` precondition | ❌ precondition not satisfied |
| 3 | Boundary | `boundary_test_3_reversed_implies` | Reversed direction: `valid(q→p)` instead of `valid(p→q)` | ❌ precondition not satisfied |
| 4 | Mutation | `mutation_test_1_reverse_weakening` | Reverse weakening: derive `□p` from `□q` | ❌ postcondition not satisfied |
| 5 | Mutation | `mutation_test_2_flipped_entailment` | Flipped entailment: `□q⊨spec` instead of `spec⊨□q` | ❌ postcondition not satisfied |
| 6 | Mutation | `mutation_test_3_extra_conclusion` | Extra unjustified conclusion: `q⊨spec` | ❌ postcondition not satisfied |
| 7 | Logical  | `logical_test_1_converse` | Converse: `valid(p→q)` ⟹ `valid(q→p)` | ❌ postcondition not satisfied |
| 8 | Logical  | `logical_test_2_transitivity` | Unwarranted transitivity to arbitrary `r` | ❌ postcondition not satisfied |
| 9 | Logical  | `logical_test_3_always_converse` | Converse of □-monotonicity: `□p⊨□q` ⟹ `p⊨q` | ❌ postcondition not satisfied |

## Analysis

- **Boundary tests (1–3):** The specification correctly requires BOTH preconditions and rejects reversed implications. Verus enforces precondition checking at call sites.
- **Behavioral mutation tests (4–6):** Mutated conclusions (reversed direction, flipped entailment, extra claims) are all rejected. The spec is tight enough to prevent incorrect behavioral inferences.
- **Logical tests (7–9):** The specification does not admit unintended logical properties — the converse of implication, unwarranted transitivity, and the converse of the □-monotonicity axiom are all correctly rejected.

## Conclusion

The `always_weaken` specification is **consistent** with respect to all tested adversarial queries. Both preconditions are necessary and sufficient, the postcondition is precise (no over- or under-specification detected), and the trusted axioms do not enable unintended reasoning within the tested scope.
