# Test Results Summary: `range_consistent_subset`

## Target Specification

`DelegationMap::range_consistent_subset` proves that if all keys in range `[x, y)` map to `dst`, and `[x_inner, y_inner) ⊆ [x, y)`, then all keys in `[x_inner, y_inner)` also map to `dst`.

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes (3/3) |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes (3/3) |
| `logical_tests.rs` | 3 | ✅ Yes (3/3) |

**Total: 9/9 tests correctly rejected by Verus.**

---

## Boundary Tests (Precondition Violations)

| Test | Violation | Result |
|------|-----------|--------|
| `bt_missing_range_consistent` | Omits `range_consistent(x, y, dst)` precondition | ❌ FAILED (correct) |
| `bt_inner_below_outer_lo` | Uses `x_inner < x` instead of `x_inner >= x` | ❌ FAILED (correct) |
| `bt_inner_above_outer_hi` | Uses `y_inner > y` instead of `y_inner <= y` | ❌ FAILED (correct) |

**Interpretation:** The spec correctly rejects all invalid inputs. Each precondition is necessary — removing or negating any one prevents the postcondition from being provable.

---

## Behavioral Mutation Tests (Output/Relation Mutations)

| Test | Mutation | Result |
|------|----------|--------|
| `bmt_wrong_destination` | Concludes consistency for a different `dst2 ≠ dst` | ❌ FAILED (correct) |
| `bmt_superset_from_subset` | Reverses direction: tries outer from inner consistency | ❌ FAILED (correct) |
| `bmt_extended_range` | Extends upper bound beyond original: `[x, y_ext)` where `y_ext > y` | ❌ FAILED (correct) |

**Interpretation:** The spec correctly rejects all mutated behaviors. The destination, range direction, and range bounds are all tightly constrained.

---

## Logical Tests (Unintended Inferences)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `lt_range_nonemptiness` | `range_consistent` implies range is non-empty (`x < y`) | ❌ FAILED (correct) |
| `lt_destination_uniqueness` | Two consistent destinations for same range must be equal | ❌ FAILED (correct) |
| `lt_universal_mapping` | `range_consistent` for `[x,y)` implies ALL keys map to `dst` | ❌ FAILED (correct) |

**Interpretation:** The spec does not support unintended logical inferences:
- Empty ranges are properly handled (vacuous truth)
- Destination uniqueness is not falsely guaranteed
- Range-local properties don't leak to global scope

---

## Conclusion

The specification for `range_consistent_subset` is **well-formed and tight**:
- **No missing preconditions**: All three `requires` clauses are necessary.
- **No behavioral over-permissiveness**: Incorrect outputs/relations are rejected.
- **No unintended logical entailments**: The spec does not admit vacuous or overly strong reasoning.

No specification weaknesses were detected.
