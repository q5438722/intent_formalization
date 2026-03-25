# Adversarial Test Summary: `not_eventually_by_always_not`

## Target Specification

The file encodes the temporal logic theorem **□¬p ⟹ ¬◇p** ("always not p" implies "not eventually p").

- `always_unfold`: axiom (`external_body`) that unfolds `always(p)` into `∀i. p(ex.suffix(i))`
- `not_eventually_by_always_not`: proves `always(not(p)) ⟹ not(eventually(p))`

## Results Overview

| Category              | Total | Failed (expected) | Passed (unexpected) |
|-----------------------|-------|--------------------|----------------------|
| Boundary Tests        | 5     | 5 ✅               | 0                    |
| Behavioral Mutations  | 5     | 5 ✅               | 0                    |
| Logical Tests         | 5     | 5 ✅               | 0                    |
| **Total**             | **15**| **15 ✅**          | **0**                |

**All 15 adversarial tests were correctly rejected by Verus.**

---

## Boundary Tests (5/5 FAILED ✅)

| Test | Description | Failure Mode |
|------|-------------|--------------|
| `boundary_no_precondition` | Call `not_eventually_by_always_not` with no assumptions | Precondition not satisfied |
| `boundary_only_initial_not` | Only `not(p)` at initial state, not `always(not(p))` | Precondition not satisfied |
| `boundary_opposite_precondition` | `always(p)` instead of `always(not(p))` | Precondition not satisfied |
| `boundary_eventually_not_instead_of_always_not` | `eventually(not(p))` — weaker than `always(not(p))` | Precondition not satisfied |
| `boundary_always_unfold_no_precondition` | Call `always_unfold` without `always(p)` | Precondition not satisfied |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| Test | Description | Failure Mode |
|------|-------------|--------------|
| `mutation_assert_opposite_postcondition` | Assert `eventually(p)` — opposite of guaranteed `not(eventually(p))` | Postcondition not satisfied |
| `mutation_assert_p_holds` | Assert `p` holds at initial state | Postcondition not satisfied |
| `mutation_assert_always_p` | Assert `always(p)` — completely contradictory | Postcondition not satisfied |
| `mutation_negate_precondition_as_postcondition` | Assert `not(always(not(p)))` — negates the precondition | Postcondition not satisfied |
| `mutation_weaker_postcondition` | Assert `not(always(p))` — different from `not(eventually(p))` | Postcondition not satisfied |

## Logical Tests (5/5 FAILED ✅)

| Test | Description | Failure Mode |
|------|-------------|--------------|
| `logical_invalid_duality` | ¬□p ⟹ □¬p — invalid direction of the modal duality | Postcondition not satisfied |
| `logical_stronger_always_not_eventually` | □¬p ⟹ □¬◇p — stronger lifting not supported by spec | Postcondition not satisfied |
| `logical_unfold_eventually_as_always` | Misuse `always_unfold` on `eventually(p)` | Precondition not satisfied |
| `logical_double_negation` | □¬¬p ⟹ □p — double negation elimination across modal operators | Postcondition not satisfied |
| `logical_always_not_implies_not_always` | □¬p ⟹ ¬□p — cross-operator reasoning not derivable | Postcondition not satisfied |

---

## Conclusion

The specification for `not_eventually_by_always_not` is **well-constrained**:

1. **Preconditions are enforced**: All boundary violations (missing, partial, opposite, or weakened preconditions) are correctly rejected.
2. **Postconditions are precise**: No incorrect behavioral mutations pass — the spec does not admit wrong output relations.
3. **Logical boundaries are tight**: The spec does not leak unintended reasoning — invalid dualities, stronger liftings, cross-operator inferences, and double negation elimination are all correctly rejected.

**Note**: An initial test for the converse (¬◇p ⟹ □¬p) was verified by Verus because it is a definitional equivalence — `not(eventually(p))` and `always(not(p))` reduce to the same formula `∀i. ¬p(ex.suffix(i))` upon unfolding. This is expected and not a spec weakness.
