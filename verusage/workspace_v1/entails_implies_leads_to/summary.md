# Test Results Summary: `entails_implies_leads_to`

**Target**: `source-projects/anvil-library/verified/temporal_logic/entails_implies_leads_to.rs`
**Date**: 2026-03-21

## Specification Under Test

```
entails_implies_leads_to(spec, p, q):
  requires: p.entails(q)          // valid(p ==> q)
  ensures:  spec.entails(p.leads_to(q))  // spec ==> always(p ==> eventually(q))
```

Supporting axioms: `valid_p_implies_always_p`, `always_implies_to_leads_to` (both `external_body`).

---

## Results: All 9 tests FAILED verification ✅

Every adversarial test was correctly rejected by the specification.

### Boundary Tests (3/3 rejected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_precondition_violated` | Call `entails_implies_leads_to` with `p=true, q=false` (p does NOT entail q) | ❌ precondition not satisfied |
| `test_boundary_valid_p_not_satisfied` | Call `valid_p_implies_always_p` with non-universally-valid `p` | ❌ precondition not satisfied |
| `test_boundary_always_implies_precondition_violated` | Call `always_implies_to_leads_to` with `p=true, q=false` (entailment of always(p⇒q) fails) | ❌ precondition not satisfied |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_reversed_leads_to` | Assert `spec.entails(q.leads_to(p))` — reversed direction | ❌ assertion failed |
| `test_mutation_converse_entails` | Assert `q.entails(p)` — converse of the precondition | ❌ assertion failed |
| `test_mutation_strengthen_to_always` | Assert `spec.entails(always(q))` — stronger than `leads_to` | ❌ assertion failed |

### Logical Tests (3/3 rejected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_leads_to_does_not_imply_eventually` | Assert `spec.entails(eventually(q))` — removes antecedent `p` from `leads_to` | ❌ assertion failed |
| `test_logical_unrelated_target` | Assert `spec.entails(p.leads_to(r))` where `r ≠ q` is unrelated | ❌ assertion failed |
| `test_logical_unrelated_global_property` | Assert `spec.entails(always(unrelated))` — cross-function structural claim | ❌ assertion failed |

---

## Assessment

The specification for `entails_implies_leads_to` is **consistent** with respect to all tested queries:

1. **Input boundary**: All three `requires` clauses (on the main function and both axioms) correctly reject calls with unsatisfied preconditions.
2. **Behavioral correctness**: The `ensures` clause is tight enough to reject reversed relations, converse entailments, and strengthened conclusions.
3. **Logical soundness**: No unintended reasoning was possible — leads_to does not leak into unconditional eventually, unrelated targets, or global structural properties.

**No specification weaknesses detected.**
