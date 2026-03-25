# Adversarial Test Summary: `leads_to_shortcut_temp`

**Target**: `source-projects/anvil-library/verified/temporal_logic/leads_to_shortcut_temp.rs`  
**Date**: 2026-03-22  
**Result**: All 15 adversarial tests **correctly rejected** (15/15 failed verification as expected)

---

## Specification Under Test

The file defines temporal logic primitives (`always`, `eventually`, `leads_to`, `entails`) and three axioms:
- `temp_pred_equality`: bidirectional entailment ⟹ equality
- `leads_to_trans`: transitivity of leads-to
- `leads_to_framed_by_or`: framing leads-to with disjunction

The main theorem `leads_to_shortcut_temp` proves:
> Given spec ⊨ p ~> (q ∨ s) and spec ⊨ q ~> (r ∨ s), then spec ⊨ p ~> (r ∨ s)

---

## Boundary Tests (5/5 FAILED as expected)

| Test | Violation | Error Type |
|------|-----------|------------|
| `boundary_missing_first_precond` | Missing `p ~> (q∨s)` | precondition not satisfied |
| `boundary_missing_second_precond` | Missing `q ~> (r∨s)` | precondition not satisfied |
| `boundary_missing_both_preconds` | Missing both preconditions | precondition not satisfied |
| `boundary_trans_missing_first` | Missing `p ~> q` for transitivity | precondition not satisfied |
| `boundary_framed_or_missing_precond` | Missing `p ~> q` for framing | precondition not satisfied |

**Conclusion**: All preconditions are enforced. Invalid inputs are properly rejected.

---

## Behavioral Mutation Tests (5/5 FAILED as expected)

| Test | Mutation | Error Type |
|------|----------|------------|
| `mutation_drop_disjunct` | Conclude `p ~> r` instead of `p ~> (r∨s)` | postcondition not satisfied |
| `mutation_wrong_target` | Conclude `p ~> q` instead of `p ~> (r∨s)` | postcondition not satisfied |
| `mutation_reverse_direction` | Conclude `(r∨s) ~> p` (reversed) | postcondition not satisfied |
| `mutation_trans_reverse` | From `p ~> q, q ~> r`, conclude `r ~> p` | postcondition not satisfied |
| `mutation_framed_or_drop_frame` | From `p ~> q`, conclude `(p∨r) ~> q` (dropped frame) | postcondition not satisfied |

**Conclusion**: All output mutations are rejected. The spec does not allow deriving incorrect behavioral conclusions.

---

## Logical Tests (5/5 FAILED as expected)

| Test | Unintended Property | Error Type |
|------|---------------------|------------|
| `logical_leads_to_not_symmetric` | `p ~> q` ⟹ `q ~> p` (symmetry) | postcondition not satisfied |
| `logical_or_elimination` | `p ~> (q∨s)` ⟹ `p ~> q` (disjunct elimination) | postcondition not satisfied |
| `logical_strengthen_shortcut` | Shortcut premises ⟹ `p ~> r` (strengthened conclusion) | postcondition not satisfied |
| `logical_leads_to_not_immediate` | `p ~> q` ⟹ `always(p ⟹ q)` (drop "eventually") | postcondition not satisfied |
| `logical_entails_not_distribute_or` | `spec ⊨ (p∨q)` ⟹ `spec ⊨ p` (disjunction distribution) | postcondition not satisfied |

**Conclusion**: The spec does not entail any unintended logical properties. The temporal "eventually" semantics, disjunction non-eliminability, and asymmetry of leads-to are all properly preserved.

---

## Overall Assessment

The specification for `leads_to_shortcut_temp` is **consistent** across all three dimensions:
1. **Preconditions** are necessary — removing any causes verification failure
2. **Postconditions** are tight — no stronger conclusion can be derived
3. **Logical boundaries** are sound — no unintended semantic inferences are permitted
