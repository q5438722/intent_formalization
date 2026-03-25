# Adversarial Test Summary: `tla_forall_always_equality`

## Target
`tla_forall_always_equality.rs` — proves commutativity of `tla_forall` and `always`:
```
tla_forall(|a| always(a_to_p(a))) == always(tla_forall(a_to_p))
```

## Results: All 9 tests FAILED verification ✓

Every adversarial query was correctly rejected by the specification.

---

### Boundary Tests (3/3 rejected ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `always_unfold` without precondition | precondition not satisfied | ✓ REJECTED |
| 2 | `tla_forall_unfold` without precondition | precondition not satisfied | ✓ REJECTED |
| 3 | `temp_pred_equality` with one-directional entailment | precondition not satisfied (`q.entails(p)` missing) | ✓ REJECTED |

**Interpretation:** All three axioms properly guard their preconditions. Invalid inputs (missing `always(p)` satisfaction, missing `tla_forall` satisfaction, incomplete mutual entailment) are rejected.

---

### Behavioral Mutation Tests (3/3 rejected ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | Dropped inner `always` | `tla_forall(a_to_p) == always(tla_forall(a_to_p))` | ✓ REJECTED |
| 2 | Wrong entailment direction | `p.entails(always(p))` | ✓ REJECTED |
| 3 | Negated main theorem | `tla_forall(|a| always(a_to_p(a))) != always(tla_forall(a_to_p))` | ✓ REJECTED |

**Interpretation:** The specification correctly distinguishes the proven equality from structurally similar but semantically different claims. Dropping the inner `always`, reversing the entailment direction, and negating the equality are all rejected.

---

### Logical Tests (3/3 rejected ✓)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | Valid from single execution | `always(p).satisfied_by(ex) ⟹ valid(always(p))` | ✓ REJECTED |
| 2 | Always distributes over disjunction | `always(p ∨ q) ⟹ always(p) ∨ always(q)` | ✓ REJECTED |
| 3 | tla_forall promotes to always | `tla_forall(a_to_p)(ex) ⟹ tla_forall(|a| always(a_to_p(a)))(ex)` | ✓ REJECTED |

**Interpretation:** The specification does not admit unintended logical reasoning. It correctly refuses:
- Universalizing from a single execution to all executions
- Distributing `always` over disjunction (a classic invalid temporal-logic step)
- Promoting a point-in-time universal quantification to a temporal-universal one

---

## Conclusion

The specification for `tla_forall_always_equality` is **consistent** with respect to all tested adversarial queries. The axioms (`always_unfold`, `tla_forall_unfold`, `temp_pred_equality`) have appropriately guarded preconditions, the proven equality resists behavioral mutations, and no unintended logical inferences are derivable from the specification.
