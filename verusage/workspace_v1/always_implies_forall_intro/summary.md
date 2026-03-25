# Adversarial Test Summary: `always_implies_forall_intro`

## Target Specification

The function `always_implies_forall_intro` proves a temporal logic distributivity law:
- **Requires**: `∀a. spec ⊨ □(p → q(a))`
- **Ensures**: `spec ⊨ □(p → ∀a. q(a))`

It relies on three `#[verifier::external_body]` axioms: `implies_apply`, `tla_forall_always_implies_equality2`, and `spec_entails_tla_forall`.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes |
| `logical_tests.rs` | 3 | ✅ Yes |

**Total: 9/9 tests correctly rejected by the verifier.**

---

## Boundary Tests (`boundary_tests.rs`)

All three tests violate preconditions and are correctly rejected:

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `boundary_test_partial_precondition` | Calls `always_implies_forall_intro` with entailment for a single `a=0` instead of `∀a` | ✅ FAIL (precondition not satisfied) |
| 2 | `boundary_test_implies_apply_missing_antecedent` | Calls `implies_apply` without `p.satisfied_by(ex)` (only provides implication, not antecedent) | ✅ FAIL (precondition not satisfied) |
| 3 | `boundary_test_spec_entails_tla_forall_partial` | Calls `spec_entails_tla_forall` with entailment for `a=true` only, missing `a=false` | ✅ FAIL (precondition not satisfied) |

**Conclusion**: Preconditions on all three functions are properly enforced.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All three tests mutate the conclusion and are correctly rejected:

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `mutation_reverse_implication` | Asserts `spec ⊨ □(∀a.q(a) → p)` (reversed direction) | ✅ FAIL (assertion failed) |
| 2 | `mutation_drop_always` | Asserts `spec ⊨ (p → ∀a.q(a))` (dropped □ operator) | ✅ FAIL (assertion failed) |
| 3 | `mutation_drop_p_guard` | Asserts `spec ⊨ □(∀a.q(a))` (dropped p guard) | ✅ FAIL (assertion failed) |

**Conclusion**: The specification is tight enough to reject all three behavioral mutations. The conclusion cannot be weakened, reversed, or structurally altered.

---

## Logical Tests (`logical_tests.rs`)

All three tests probe unintended logical consequences and are correctly rejected:

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `logical_valid_instead_of_entails` | Asserts `valid(□(p → ∀a.q(a)))` — universal validity, not just entailment by spec | ✅ FAIL (assertion failed) |
| 2 | `logical_entails_antecedent` | Asserts `spec ⊨ □(p)` — that the antecedent itself is entailed | ✅ FAIL (assertion failed) |
| 3 | `logical_single_instance_without_forall` | Assumes entailment for single `a=0` only, asserts universal conclusion | ✅ FAIL (assertion failed) |

**Conclusion**: The specification does not over-entail. It correctly distinguishes between entailment-by-spec vs. universal validity, does not conflate the antecedent with the conclusion, and requires full universality.

---

## Overall Assessment

**The specification is consistent.** All 9 adversarial queries — covering precondition violations, output mutations, and unintended logical consequences — were correctly rejected by Verus. The specification neither admits invalid inputs, nor permits incorrect behavioral mutations, nor supports unintended logical inferences.
