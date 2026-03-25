# Adversarial Proof Test Summary: `leads_to_by_borrowing_inv`

## Target Specification

The lemma `leads_to_by_borrowing_inv` states: if `spec ⊨ (p∧inv) ↝ q` and `spec ⊨ □inv`, then `spec ⊨ p ↝ q`. This is a standard temporal logic rule for "borrowing" an invariant into a leads-to proof.

## Results

**All 13 adversarial tests FAILED verification as expected.** The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

### Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✓

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_missing_always_inv` | Missing `spec.entails(always(inv))` | precondition not satisfied ✓ |
| 2 | `test_boundary_missing_leads_to` | Missing `spec.entails(p.and(inv).leads_to(q))` | precondition not satisfied ✓ |
| 3 | `test_boundary_no_preconditions` | Both preconditions missing | precondition not satisfied ✓ |
| 4 | `test_boundary_always_no_spec` | `instantiate_entailed_always` without `spec.satisfied_by(ex)` | precondition not satisfied ✓ |
| 5 | `test_boundary_leads_to_no_implies` | `instantiate_entailed_leads_to` without `spec.implies(...)` | precondition not satisfied ✓ |

**Conclusion:** All preconditions are properly enforced. Omitting any required condition is caught.

---

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✓

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_reversed_leads_to` | Conclude `q ↝ p` instead of `p ↝ q` | postcondition not satisfied ✓ |
| 2 | `test_mutation_always_q` | Conclude `□q` instead of `p ↝ q` | postcondition not satisfied ✓ |
| 3 | `test_mutation_wrong_source` | Conclude `inv ↝ q` instead of `p ↝ q` | postcondition not satisfied ✓ |
| 4 | `test_mutation_conjunction_target` | Conclude `p ↝ (q∧p)` instead of `p ↝ q` | postcondition not satisfied ✓ |

**Conclusion:** The postcondition is precise. No mutated output relation is accepted.

---

### Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✓

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_symmetry` | `p ↝ q ⟹ q ↝ p` (symmetry) | postcondition not satisfied ✓ |
| 2 | `test_logical_conjunction` | `p ↝ q ∧ p ↝ r ⟹ p ↝ (q∧r)` | postcondition not satisfied ✓ |
| 3 | `test_logical_eventually_to_always` | `p ↝ q ⟹ p ↝ □q` (eventually→always) | postcondition not satisfied ✓ |
| 4 | `test_logical_invariant_sufficiency` | `□inv` alone implies `p ↝ q` | postcondition not satisfied ✓ |

**Conclusion:** The specification does not admit unintended logical inferences. Key temporal logic distinctions (symmetry, conjunction, eventually vs. always) are preserved.

---

## Overall Assessment

The specification for `leads_to_by_borrowing_inv` is **consistent**: it correctly rejects all 13 adversarial queries across boundary, behavioral, and logical dimensions. No weaknesses were detected.
