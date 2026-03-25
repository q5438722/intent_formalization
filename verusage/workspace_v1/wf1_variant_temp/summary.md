# Adversarial Test Results: `wf1_variant_temp.rs`

**Target**: `source-projects/anvil-library/verified/temporal_logic/wf1_variant_temp.rs`  
**Date**: 2026-03-22  
**Result**: ✅ All 16/16 tests **failed verification** as expected — specification is consistent.

---

## Summary

| Category | Tests | All Failed? | Verdict |
|---|---|---|---|
| Boundary Tests | 6 | ✅ Yes | Preconditions correctly enforced |
| Behavioral Mutation Tests | 5 | ✅ Yes | Incorrect behaviors rejected |
| Logical Tests | 5 | ✅ Yes | Unintended reasoning blocked |

---

## (1) Boundary Tests — `boundary_tests.rs` (6/6 failed ✅)

| Test | Violation | Error Type |
|---|---|---|
| `boundary_missing_stuttering_preservation` | Dropped 1st requires (stuttering: p∧next → ○p∨○q) | precondition not satisfied |
| `boundary_missing_forward_progress` | Dropped 2nd requires (forward progress: p∧next∧forward → ○q) | precondition not satisfied |
| `boundary_missing_always_next` | Dropped 3rd requires (□next) | precondition not satisfied |
| `boundary_missing_liveness` | Dropped 4th requires (□p ↝ forward) | precondition not satisfied |
| `boundary_implies_apply_no_antecedent` | Called `implies_apply` without `p.satisfied_by(ex)` | precondition not satisfied |
| `boundary_eventually_no_witness` | Called `eventually_proved_by_witness` without witness condition | precondition not satisfied |

**Conclusion**: Every precondition of `wf1_variant_temp` and its helper lemmas is essential and correctly enforced. Dropping any single one causes verification failure.

---

## (2) Behavioral Mutation Tests — `behavioral_mutation_tests.rs` (5/5 failed ✅)

| Test | Mutation | Error Type |
|---|---|---|
| `mutation_swapped_leads_to` | Conclusion swapped: `q ↝ p` instead of `p ↝ q` | postcondition not satisfied |
| `mutation_strengthen_to_always` | Strengthened to `spec ⊨ □q` | postcondition not satisfied |
| `mutation_stronger_conjunction` | Strengthened to `p ↝ (p∧q)` | postcondition not satisfied |
| `mutation_instantaneous_implication` | Dropped temporality: `spec ⊨ p→q` (instantaneous) | postcondition not satisfied |
| `mutation_unrelated_always_forward` | Unrelated: `spec ⊨ □forward` | postcondition not satisfied |

**Conclusion**: The specification precisely entails `spec ⊨ p ↝ q` and nothing stronger, swapped, or unrelated. All behavioral mutations are correctly rejected.

---

## (3) Logical Tests — `logical_tests.rs` (5/5 failed ✅)

| Test | Property Tested | Error Type |
|---|---|---|
| `logical_consistency_check` | Can `false` be derived from WF1 preconditions? | postcondition not satisfied |
| `logical_eventually_does_not_imply_always` | ◇p ⟹ □p (invalid direction) | postcondition not satisfied |
| `logical_leads_to_not_symmetric` | p↝q ⟹ q↝p (symmetry) | postcondition not satisfied |
| `logical_always_no_or_distribution` | □(p∨q) ⟹ □p ∨ □q (invalid distribution) | postcondition not satisfied |
| `logical_partial_execution_equality` | ex₁(0)=ex₂(0) ⟹ ex₁=ex₂ (partial equality) | postcondition not satisfied |

**Conclusion**: The axiom system is consistent (cannot derive `false`). Temporal logic operators maintain correct semantics — `eventually` does not imply `always`, `leads_to` is not symmetric, `always` does not distribute over `or`, and execution equality requires full extensional equality.

---

## Overall Assessment

The specification of `wf1_variant_temp` and its supporting axioms is **well-formed and tight**:

1. **No missing preconditions**: All four WF1 preconditions are necessary.
2. **No over-approximation**: The conclusion `p ↝ q` cannot be strengthened or mutated.
3. **No logical inconsistencies**: The axiom system does not admit unintended reasoning.
