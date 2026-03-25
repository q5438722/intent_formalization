# Adversarial Proof Test Results for `wf1.rs`

**Target**: `source-projects/anvil-library/verified/temporal_logic/wf1.rs`
**Date**: 2026-03-22

## Summary

All **13 tests** across 3 categories **failed verification as expected**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

| Category | Tests | All Rejected? |
|---|---|---|
| Boundary Tests | 5 | ✅ Yes |
| Behavioral Mutation Tests | 4 | ✅ Yes |
| Logical Tests | 4 | ✅ Yes |

**Conclusion**: The specification is **consistent** — no weaknesses detected. All semantic boundary queries were properly rejected.

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

Each test omits one precondition of a proof function and attempts to call it.

| # | Test | Omitted Precondition | Result |
|---|---|---|---|
| 1 | `test_wf1_missing_inductive` | Condition 1: `p ∧ next → p' ∨ q'` | ✅ Rejected (precondition not satisfied) |
| 2 | `test_wf1_missing_progress` | Condition 2: `p ∧ next ∧ forward → q'` | ✅ Rejected (precondition not satisfied) |
| 3 | `test_wf1_missing_enablement` | Condition 3: `p → enabled(forward)` | ✅ Rejected (precondition not satisfied) |
| 4 | `test_implies_apply_missing_p` | `p.satisfied_by(ex)` | ✅ Rejected (precondition not satisfied) |
| 5 | `test_execution_equality_no_pointwise` | Pointwise state equality | ✅ Rejected (precondition not satisfied) |

**Interpretation**: All preconditions are necessary and enforced. The spec correctly rejects calls with incomplete assumptions.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✅

Each test assumes a valid conclusion and asserts a mutated variant.

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_wrong_target` | `p ~> q` → assert `p ~> r` (wrong target) | ✅ Rejected (assertion failed) |
| 2 | `test_mutation_reversed_leads_to` | `p ~> q` → assert `q ~> p` (reversed) | ✅ Rejected (assertion failed) |
| 3 | `test_mutation_strengthen_to_always` | `spec ⊨ p ~> q` → assert `spec ⊨ □q` (strengthened) | ✅ Rejected (assertion failed) |
| 4 | `test_mutation_implies_wrong_conclusion` | `implies_apply` gives `q` → assert `r` (wrong conclusion) | ✅ Rejected (assertion failed) |

**Interpretation**: The spec precisely constrains its outputs. Mutated behaviors (wrong targets, reversals, strengthened conclusions) are all rejected.

---

## Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✅

Each test probes a logical property NOT guaranteed by the specification.

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_eventually_not_always` | `◇p → □p` (eventually implies always) | ✅ Rejected (assertion failed) |
| 2 | `test_logical_head_eq_not_exec_eq` | Head equality → execution equality | ✅ Rejected (assertion failed) |
| 3 | `test_logical_implies_not_symmetric` | `p → q` implies `q → p` (symmetry) | ✅ Rejected (assertion failed) |
| 4 | `test_logical_entails_not_strengthened` | `spec ⊨ q` implies `spec ⊨ (q ∧ r)` (strengthening) | ✅ Rejected (assertion failed) |

**Interpretation**: The spec does not admit unintended logical inferences. Temporal operators, implication, and entailment all behave correctly at the semantic boundaries.
