# Adversarial Test Summary: `init_invariant_rec`

## Target Specification

`init_invariant_rec` proves that an inductive invariant `inv` holds at position `i` of an execution trace, given:
1. `init(ex.head())` — initial state satisfies `init`
2. `∀ idx. next(ex[idx], ex[idx+1])` — all transitions satisfy `next`
3. `∀ idx. init(ex[idx]) ⟹ inv(ex[idx])` — `init` implies `inv`
4. `∀ idx. inv(ex[idx]) ∧ next(ex[idx], ex[idx+1]) ⟹ inv(ex[idx+1])` — `inv` is preserved by `next`

**Ensures:** `inv(ex.suffix(i).head())`

---

## Results Overview

| Category | Tests | All Failed (Expected) | Spec Weakness Found |
|---|---|---|---|
| Boundary | 4 | ✅ Yes | None |
| Behavioral Mutation | 3 | ✅ Yes | None |
| Logical | 4 | ✅ Yes | None |
| **Total** | **11** | **11/11 ✅** | **None** |

---

## Boundary Tests (boundary_tests.rs) — 4/4 FAILED ✅

Each test drops one precondition and attempts to call `init_invariant_rec`.

| Test | Dropped Precondition | Failure Mode |
|---|---|---|
| `test_missing_init` | `init(ex.head())` | Precondition not satisfied |
| `test_missing_next` | `∀ next(...)` | Precondition not satisfied |
| `test_missing_init_implies_inv` | `init ⟹ inv` | Precondition not satisfied |
| `test_missing_inductive_step` | `inv ∧ next ⟹ inv'` | Precondition not satisfied |

**Conclusion:** All four preconditions are independently necessary. The spec correctly rejects calls with any missing precondition.

---

## Behavioral Mutation Tests (behavioral_mutation_tests.rs) — 3/3 FAILED ✅

Each test provides full valid preconditions but asserts a mutated postcondition.

| Test | Mutation | Failure Mode |
|---|---|---|
| `test_ensures_init_instead_of_inv` | Replace `inv` with `init` in ensures | Postcondition not satisfied |
| `test_ensures_negated` | Negate `inv(...)` in ensures | Postcondition not satisfied |
| `test_ensures_wrong_execution` | Apply ensures to unrelated `ex2` | Postcondition not satisfied |

**Conclusion:** The spec precisely guarantees `inv` (not `init`) at the specified position of the specified execution. Incorrect output relations are rejected.

---

## Logical Tests (logical_tests.rs) — 4/4 FAILED ✅

Each test asserts a property not explicitly guaranteed by the specification.

| Test | Unentailed Property | Failure Mode |
|---|---|---|
| `test_init_propagates_everywhere` | `init` holds at all positions | Postcondition not satisfied |
| `test_inv_holds_for_arbitrary_state` | `inv(s)` for arbitrary `s: T` | Postcondition not satisfied |
| `test_execution_uniqueness` | Same predicates ⟹ same execution states | Postcondition not satisfied |
| `test_inv_next_implies_init` | `inv ∧ next ⟹ init'` (reversed direction) | Postcondition not satisfied |

**Conclusion:** The spec does not admit unintended logical inferences:
- `init` is not confused with `inv` (no init propagation)
- `inv` is execution-local, not universal
- Executions are not assumed deterministic
- The `init ⟹ inv` direction is not reversible

---

## Overall Assessment

The specification of `init_invariant_rec` is **well-constrained**:
- **No boundary weakness:** Every precondition is independently necessary.
- **No behavioral weakness:** Only the correct postcondition is derivable.
- **No logical weakness:** No unintended properties are entailed.

The specification faithfully encodes the inductive invariant principle without admitting extraneous reasoning.
