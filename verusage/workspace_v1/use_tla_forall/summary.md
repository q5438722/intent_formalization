# Adversarial Test Results — `use_tla_forall.rs`

## Overview

12 adversarial proof tests were generated across 3 categories to probe the semantic boundaries of the `use_tla_forall` specification. **All 12 tests failed verification as expected**, indicating the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (`boundary_tests.rs`) — 4/4 FAILED ✓

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| `test_boundary_1_no_precondition` | Call `use_tla_forall` with no precondition established | precondition not satisfied: `spec.entails(tla_forall(a_to_p))` |
| `test_boundary_2_missing_entails` | Call `entails_apply` with only `p.satisfied_by(ex)`, missing `p.entails(q)` | precondition not satisfied: `p.entails(q)` |
| `test_boundary_3_missing_satisfaction` | Call `entails_apply` with only `p.entails(q)`, missing `p.satisfied_by(ex)` | precondition not satisfied: `p.satisfied_by(ex)` |
| `test_boundary_4_instance_not_universal` | Concrete case: spec entails instances but not the universal | precondition not satisfied: `spec.entails(tla_forall(a_to_p))` |

**Conclusion**: The preconditions on both `use_tla_forall` and `entails_apply` are sufficient to reject all tested invalid input patterns.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✓

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `test_mutation_1_strengthen_to_valid` | Strengthen postcondition: `spec.entails(a_to_p(a))` → `valid(a_to_p(a))` | postcondition not satisfied |
| `test_mutation_2_wrong_mapping` | Substitute different mapping: conclude about `b_to_p` instead of `a_to_p` | postcondition not satisfied |
| `test_mutation_3_reverse_entailment` | Reverse entailment direction: `spec.entails(p)` → `p.entails(spec)` | postcondition not satisfied |
| `test_mutation_4_negate_conclusion` | Negate conclusion: assert `spec.entails(¬a_to_p(a))` | postcondition not satisfied |

**Conclusion**: The specification correctly distinguishes its postcondition from strengthened, substituted, reversed, and negated variants.

---

## Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✓

| Test | Unintended Property Tested | Verus Error |
|------|---------------------------|-------------|
| `test_logical_1_entails_not_symmetric` | Symmetry: `p.entails(q) ⟹ q.entails(p)` | postcondition not satisfied |
| `test_logical_2_entails_not_valid` | Drop hypothesis: `spec.entails(p) ⟹ valid(p)` | postcondition not satisfied |
| `test_logical_3_instance_to_universal` | Converse of `use_tla_forall`: single instance → universal | postcondition not satisfied |
| `test_logical_4_satisfaction_not_entailment` | Pointwise → global: one execution → all executions | postcondition not satisfied |

**Conclusion**: The specification does not admit unintended logical inferences. Entailment is correctly non-symmetric, conditional (not unconditional), and cannot be generalized from instances to universals.

---

## Final Assessment

The specification for `use_tla_forall` and its supporting definitions (`entails`, `valid`, `tla_forall`, `entails_apply`) is **consistent** across all tested semantic dimensions:

- **Preconditions** reject all invalid inputs (boundary completeness)
- **Postconditions** reject all mutated behaviors (behavioral precision)
- **Logical properties** are correctly bounded (no unintended entailments)

No specification weaknesses were detected.
