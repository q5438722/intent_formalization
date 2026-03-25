# Adversarial Test Summary: `always_implies_preserved_by_always`

## Target Specification

The theorem `always_implies_preserved_by_always` proves:
```
requires spec.entails(always(p.implies(q)))
ensures  spec.entails(always(always(p).implies(always(q))))
```
Meaning: if `p ⟹ q` holds at every step under spec, then `always(p) ⟹ always(q)` also holds at every step under spec.

Supported by three axioms: `always_unfold`, `implies_apply`, `execution_equality`.

---

## Results Overview

| File | Tests | All Failed? | Spec Weakness Found? |
|------|-------|-------------|---------------------|
| `boundary_tests.rs` | 4 | ✅ Yes (4/4 failed) | No |
| `behavioral_mutation_tests.rs` | 4 | ✅ Yes (4/4 failed) | No |
| `logical_tests.rs` | 4 | ✅ Yes (4/4 failed) | No |

**Total: 12/12 adversarial tests correctly rejected by Verus.**

---

## Boundary Tests (4/4 FAILED ✅)

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_no_precondition` | Conclusion holds without any precondition | FAILED ✅ |
| 2 | `test_missing_always_in_precondition` | `spec.entails(p ⟹ q)` suffices (missing `always`) | FAILED ✅ |
| 3 | `test_only_always_p` | `spec.entails(always(p))` suffices (wrong predicate) | FAILED ✅ |
| 4 | `test_converse_precondition` | `spec.entails(always(q ⟹ p))` suffices (converse) | FAILED ✅ |

**Conclusion:** The precondition `spec.entails(always(p.implies(q)))` is necessary — weaker, missing, or different preconditions are correctly rejected.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

| # | Test | Mutation Applied | Result |
|---|------|-----------------|--------|
| 1 | `test_swap_pq_in_conclusion` | Swapped p↔q: `always(always(q) ⟹ always(p))` | FAILED ✅ |
| 2 | `test_conclude_always_q` | Strengthened to `always(q)` unconditionally | FAILED ✅ |
| 3 | `test_p_implies_always_q` | Changed to `p ⟹ always(q)` (point-to-universal) | FAILED ✅ |
| 4 | `test_wrong_nesting_direction` | Changed to `always(q) ⟹ p` (wrong nesting/direction) | FAILED ✅ |

**Conclusion:** The postcondition correctly captures the exact relationship. Swapping arguments, strengthening, or changing nesting structure are all rejected.

---

## Logical Tests (4/4 FAILED ✅)

| # | Test | Logical Property Tested | Result |
|---|------|------------------------|--------|
| 1 | `test_drop_spec_to_valid` | `spec.entails(X)` implies `valid(X)` (drop spec guard) | FAILED ✅ |
| 2 | `test_different_suffixes_equal` | `ex.suffix(0) == ex.suffix(1)` (collapse distinct states) | FAILED ✅ |
| 3 | `test_entailment_symmetric` | `spec.entails(X)` implies `X.entails(spec)` (symmetry) | FAILED ✅ |
| 4 | `test_single_execution_to_valid` | `always(p)` on one execution implies `valid(p)` (universalize) | FAILED ✅ |

**Conclusion:** The spec does not allow unintended logical inferences. Entailment is not conflated with validity, suffix identity is preserved, entailment is not symmetric, and single-execution facts are not universalized.

---

## Overall Assessment

The specification for `always_implies_preserved_by_always` is **robust against all 12 adversarial queries**:

- **Preconditions are tight** — no weaker alternative suffices.
- **Postconditions are precise** — mutated behaviors are rejected.
- **Logical boundaries are sound** — no unintended reasoning is permitted by the axioms.

No specification weaknesses were detected.
