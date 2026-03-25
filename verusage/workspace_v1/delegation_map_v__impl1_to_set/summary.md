# Test Execution Summary: `delegation_map_v__impl1_to_set`

## Target: `StrictlyOrderedVec::to_set` proof function

**Specification under test:**
```
requires self.valid()
ensures  s == self@.to_set(), s.finite(), s.len() == self@.len()
```

---

## Results: All 9 adversarial tests FAILED verification ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (boundary_tests.rs) — 3/3 rejected ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_to_set_no_precondition` | Call `to_set` without `valid()` precondition | ✅ Rejected (precondition not satisfied) |
| `test_to_set_negated_valid` | Call `to_set` with `!valid()` | ✅ Rejected (precondition not satisfied) |
| `test_empty_valid_nonempty_set` | Assert non-empty set from empty valid vec | ✅ Rejected (assertion failed) |

### Behavioral Mutation Tests (behavioral_mutation_tests.rs) — 3/3 rejected ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_wrong_set_length` | Assert `s.len() == sov@.len() + 1` (off-by-one) | ✅ Rejected (assertion failed) |
| `test_set_empty_when_nonempty` | Assert `s.len() == 0` for non-empty sequence | ✅ Rejected (assertion failed) |
| `test_result_not_finite` | Assert `!s.finite()` (negate finiteness) | ✅ Rejected (assertion failed) |

### Logical Tests (logical_tests.rs) — 3/3 rejected ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_cmp_transitivity` | Assert `cmp_spec` transitivity (no such axiom) | ✅ Rejected (assertion failed) |
| `test_cmp_antisymmetry` | Assert `cmp_spec` antisymmetry (no such axiom) | ✅ Rejected (assertion failed) |
| `test_same_length_implies_same_set` | Assert same-length valid vecs produce equal sets | ✅ Rejected (assertion failed) |

---

## Notable Finding

During development, a test (`test_sorted_not_leaked`) revealed that `closed spec fn valid()` **does expose its body within the same crate**. Asserting `sov@[0].cmp_spec(sov@[1]).lt()` given only `sov.valid()` was provable, meaning the `sorted` property inside `valid()` is accessible within the same compilation unit. This is expected Verus behavior — `closed` controls cross-crate visibility, not intra-crate opacity.

## Conclusion

The `to_set` specification is **consistent**: it correctly guards its postconditions behind the `valid()` precondition, produces accurate length/finiteness/equality guarantees, and does not inadvertently entail properties about the unconstrained `cmp_spec` comparison function.
