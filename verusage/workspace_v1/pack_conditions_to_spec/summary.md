# Adversarial Test Summary: `pack_conditions_to_spec`

## Target Specification

```
pack_conditions_to_spec<T>(spec, c, p, q)
  requires spec.entails(p.and(c).leads_to(q))
  ensures  spec.and(always(c)).entails(p.leads_to(q))
```

Semantics: If `spec` guarantees that `p ∧ c` leads-to `q`, then `spec ∧ □c` guarantees that `p` alone leads-to `q`.

## Results: 9/9 tests correctly FAILED verification

The specification rejects all adversarial queries — no inconsistencies found.

---

### Boundary Tests (3/3 FAIL ✓) — `boundary_tests.rs`

| Test | Violation | Verus Error |
|------|-----------|-------------|
| `boundary_no_precondition` | No `requires` clause; calls `pack_conditions_to_spec` without establishing any precondition | precondition not satisfied |
| `boundary_swapped_pq_precondition` | Provides `spec ⊨ (q∧c) ~> p` instead of `spec ⊨ (p∧c) ~> q` | precondition not satisfied |
| `boundary_unrelated_precondition` | Provides `spec ⊨ □p` (irrelevant) instead of the required leads-to entailment | precondition not satisfied |

**Finding:** The precondition `spec.entails(p.and(c).leads_to(q))` is properly enforced. Invalid or unrelated inputs are correctly rejected.

---

### Behavioral Mutation Tests (3/3 FAIL ✓) — `behavioral_mutation_tests.rs`

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `mutation_drop_always_c` | Removes `always(c)` from conclusion: claims `spec ⊨ p ~> q` | postcondition not satisfied |
| `mutation_swap_pq_conclusion` | Swaps p and q: claims `spec ∧ □c ⊨ q ~> p` | postcondition not satisfied |
| `mutation_strengthen_to_always_q` | Strengthens to □q: claims `spec ∧ □c ⊨ □q` | postcondition not satisfied |

**Finding:** The postcondition correctly discriminates the exact result. Dropping the `always(c)` guard, reversing the leads-to direction, and strengthening from temporal liveness to safety are all rejected.

---

### Logical Tests (3/3 FAIL ✓) — `logical_tests.rs`

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| `logical_implies_from_leads_to` | Derives pointwise `p ⇒ q` from temporal `p ~> q` (conflates temporal/pointwise) | assertion failed |
| `logical_converse_unpacking` | Reverses the theorem: from `spec ∧ □c ⊨ p ~> q`, derives `spec ⊨ (p∧c) ~> q` | postcondition not satisfied |
| `logical_always_p_implies_always_q` | Claims `□p ⊨ □q` as a structural consequence of leads-to | assertion failed |

**Finding:** The specification does not admit unintended logical reasoning. Temporal-to-pointwise conflation, converse unpacking, and unsound global inferences are all rejected.

---

## Conclusion

The specification of `pack_conditions_to_spec` is **consistent** with respect to all 9 adversarial queries across three categories:

- **Input boundaries** are enforced: the precondition rejects invalid, swapped, and unrelated inputs.
- **Output correctness** is precise: mutations to the conclusion (dropping guards, swapping arguments, strengthening) are rejected.
- **Logical entailment** is controlled: the spec does not enable unintended reasoning (pointwise conflation, converse direction, global structural claims).

No specification weaknesses were identified.
