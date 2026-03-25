# Adversarial Proof Test Summary: `init_invariant.rs`

## Target Specification

The `init_invariant` theorem proves that if:
1. `init(s) ==> inv(s)` (init implies invariant)
2. `inv(s) && next(s, s') ==> inv(s')` (invariant is inductive)
3. `spec ⊨ lift_state(init)` (spec entails initial condition)
4. `spec ⊨ □(lift_action(next))` (spec entails always-next)

Then: `spec ⊨ □(lift_state(inv))` (spec entails the invariant always holds)

## Results Summary

| Test File | Tests | All Failed? | Verdict |
|-----------|-------|-------------|---------|
| boundary_tests.rs | 4 | ✅ Yes (4/4 failed) | Spec correctly rejects precondition violations |
| mutation_tests.rs | 3 | ✅ Yes (3/3 failed) | Spec correctly rejects mutated behaviors |
| logical_tests.rs  | 3 | ✅ Yes (3/3 failed) | Spec correctly rejects unintended reasoning |

**Overall: 10/10 adversarial tests correctly rejected.**

---

## Boundary Tests (4 tests)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| B1 | `boundary_init_not_implies_inv` | `init(s) ==> inv(s)` — counterexample s=0 | ✅ FAILED |
| B2 | `boundary_inv_not_inductive` | `inv(s) && next(s,s') ==> inv(s')` — counterexample s=4,s'=5 | ✅ FAILED |
| B3 | `boundary_spec_too_permissive` | `spec.entails(lift_state(init))` — trivial spec cannot entail init | ✅ FAILED |
| B4 | `boundary_action_not_enforced` | `spec.entails(always(lift_action(next)))` — constant execution violates next | ✅ FAILED |

## Behavioral Mutation Tests (3 tests)

| # | Test | Mutation Applied | Result |
|---|------|------------------|--------|
| M1 | `mutation_wrong_inv_constant` | inv value: `s == 1` instead of `s == 0` | ✅ FAILED |
| M2 | `mutation_off_by_one_inv` | inv bound: `s > 0` instead of `s >= 0` | ✅ FAILED |
| M3 | `mutation_wrong_action_relation` | action: `s' == s + 1` instead of `s' == s` | ✅ FAILED |

## Logical Tests (3 tests)

| # | Test | Unintended Property Tested | Result |
|---|------|---------------------------|--------|
| L1 | `logical_inv_does_not_imply_init` | Reverse: `inv(s) ==> init(s)` | ✅ FAILED |
| L2 | `logical_next_not_deterministic` | Determinism: unique successor | ✅ FAILED |
| L3 | `logical_always_inv_not_implies_init` | Reverse entailment: `□inv ==> init` | ✅ FAILED |

---

## Conclusion

The specification of `init_invariant` (and its helper axioms) is **consistent** with respect to all tested adversarial queries:

- **Precondition boundaries are tight**: Each `requires` clause is necessary — removing any one allows invalid reasoning.
- **Behavioral mutations are rejected**: Wrong invariant values, off-by-one errors, and wrong action relations are all caught.
- **No unintended logical consequences**: The spec does not imply reverse entailment, determinism, or reverse precondition directions.

No specification weaknesses were detected in this test suite.
