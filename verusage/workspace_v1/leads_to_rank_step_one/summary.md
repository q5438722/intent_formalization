# Test Execution Summary: `leads_to_rank_step_one`

## Target Specification

The function `leads_to_rank_step_one` is an induction lemma for temporal logic:
- **Requires**: ∀ n > 0, spec ⊨ p(n) ↝ p(n−1)
- **Ensures**: ∀ n, spec ⊨ p(n) ↝ p(0)

## Results Overview

| Category | Test | Expected | Actual | Status |
|----------|------|----------|--------|--------|
| **Boundary** | No precondition | FAIL | FAIL (precondition not satisfied) | ✅ |
| **Boundary** | Partial precondition (n=1 only) | FAIL | FAIL (precondition not satisfied) | ✅ |
| **Boundary** | Reversed direction precondition | FAIL | FAIL (precondition not satisfied) | ✅ |
| **Behavioral** | Reverse leads_to: p(0) ↝ p(5) | FAIL | FAIL (assertion failed) | ✅ |
| **Behavioral** | Wrong target: p(5) ↝ p(1) | FAIL | FAIL (assertion failed) | ✅ |
| **Behavioral** | Universal wrong target: ∀n>0, p(n) ↝ p(1) | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | Spec is universally valid | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | p(0) always holds under spec | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | Bounded liveness (p(3)@0 ⟹ p(0)@3) | FAIL | FAIL (assertion failed) | ✅ |

**Total: 9/9 tests correctly rejected ✅**

## Analysis

The specification is **consistent** across all three query dimensions:

1. **Boundary**: The precondition requires a *universal* quantifier over *all* n > 0 with the *correct* direction. Partial, missing, or reversed preconditions are all rejected.

2. **Behavioral**: The postcondition precisely guarantees p(n) ↝ p(0) and nothing more. It does not allow deriving the reverse direction (p(0) ↝ p(n)) or intermediate targets (p(n) ↝ p(1)).

3. **Logical**: The specification does not leak unintended properties:
   - It does not imply spec is universally valid.
   - It does not strengthen leads_to into always.
   - It does not provide bounded step guarantees (eventual ≠ bounded).

## Conclusion

No specification weaknesses detected. The `leads_to_rank_step_one` specification correctly constrains the semantic boundary: it entails what it should and rejects what it shouldn't.
