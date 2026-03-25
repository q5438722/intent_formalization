# Adversarial Test Summary: `always_distributed_by_and`

## Target Specification
Proves: `valid(always(p ∧ q) → (always(p) ∧ always(q)))` — the `always` operator distributes over conjunction.

## Results: All 9 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| boundary_test_1 | Call `always_unfold` without precondition | ❌ Rejected (precondition not satisfied) |
| boundary_test_2 | `valid(always(p))` for arbitrary `p` | ❌ Rejected (postcondition not satisfied) |
| boundary_test_3 | `∀i. p(ex.suffix(i))` from nothing | ❌ Rejected (postcondition not satisfied) |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| mutation_test_1 | `always(p) → always(p ∧ q)` (dropped hypothesis) | ❌ Rejected |
| mutation_test_2 | `always(p) → always(q)` (swapped predicates) | ❌ Rejected |
| mutation_test_3 | `always(p→q) → always(p) ∧ always(q)` (weakened hypothesis) | ❌ Rejected |

### Logical Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| logical_test_1 | `valid(false)` (vacuous inconsistency) | ❌ Rejected |
| logical_test_2 | `always(p→q) → always(p)` (antecedent extraction) | ❌ Rejected |
| logical_test_3 | `always(p) ∧ always(q) ⟹ p ≡ q` (predicate equality) | ❌ Rejected |

## Conclusion

The specification for `always_distributed_by_and` is **consistent** with respect to all tested adversarial properties:
- **Boundary**: Preconditions are enforced; arbitrary predicates cannot be shown always-valid.
- **Behavioral**: Mutated/weakened versions of the theorem are properly rejected.
- **Logical**: The spec does not admit false, does not confuse implication with conjunction, and does not collapse distinct predicates.

No specification weaknesses were detected.
