# Adversarial Test Results: `leads_to_rank_step_one_usize`

## Target Specification

The function proves: if ∀n>0, `spec ⊨ p(n) ↝ p(n-1)`, then ∀n, `spec ⊨ p(n) ↝ p(0)`.
(Inductive rank-step-down for temporal leads-to over bounded `usize`.)

---

## Results Summary

| Test File | Tests | Failed (as expected) | Passed (unexpected) |
|-----------|-------|---------------------|---------------------|
| boundary_tests.rs | 3 | 3 ✅ | 0 |
| behavioral_mutation_tests.rs | 3 | 3 ✅ | 0 |
| logical_tests.rs | 3 | 3 ✅ | 0 |
| **Total** | **9** | **9 ✅** | **0** |

All 9 adversarial tests were correctly **rejected** by the specification.

---

## Boundary Tests (3/3 rejected ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| `test_boundary_missing_precondition` | Calls function with no precondition at all | Precondition not satisfied |
| `test_boundary_partial_precondition` | Only assumes p(1)↝p(0), not ∀n>0 | Precondition not satisfied |
| `test_boundary_skip_base_step` | Assumes step-down for n>1 only, missing n=1→0 | Precondition not satisfied |

**Conclusion**: The `requires` clause correctly demands the **full** universal step-down hypothesis. Partial or absent preconditions are rejected.

---

## Behavioral Mutation Tests (3/3 rejected ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| `test_mutation_reverse_leads_to` | Asserts p(0) ↝ p(n) (reverse direction) | Assertion failed |
| `test_mutation_p0_leads_to_p1` | Asserts p(0) ↝ p(1) (upward step) | Assertion failed |
| `test_mutation_valid_without_spec` | Asserts `valid(p(n)↝p(0))` instead of `spec⊨(p(n)↝p(0))` | Assertion failed |

**Conclusion**: The specification correctly rejects:
- Reverse-direction reasoning (leads-to is not symmetric)
- Upward steps from p(0) (only downward steps are guaranteed)
- Removing the spec context (entailment under spec ≠ universal validity)

---

## Logical Tests (3/3 rejected ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| `test_logical_always_p0` | Asserts `spec ⊨ □p(0)` (leads-to ≠ always) | Assertion failed |
| `test_logical_direct_entailment` | Asserts `p(n) ⊨ p(0)` directly (temporal ≠ immediate) | Assertion failed |
| `test_logical_spec_universally_valid` | Asserts `valid(spec)` (spec entailment ≠ spec validity) | Assertion failed |

**Conclusion**: The specification does **not** allow unintended logical inferences:
- Leads-to (◇) is not conflated with always (□)
- Temporal entailment is not conflated with direct/immediate entailment
- `spec.entails(X)` does not imply `valid(spec)`

---

## Overall Assessment

The specification for `leads_to_rank_step_one_usize` is **consistent** across all three tested dimensions:

1. **Input boundary**: Invalid/insufficient preconditions are properly rejected.
2. **Behavioral correctness**: Mutated output claims (reverse, wrong target, context removal) are rejected.
3. **Logical soundness**: Unintended logical over-claims (always, direct entailment, universal validity) are rejected.

No specification weaknesses were discovered.
