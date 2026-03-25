# Adversarial Proof Test Results — `lemma_seq_fold_left_append_len_int`

## Target Specification

The lemma has **no preconditions** and proves:
```
s.fold_left(prefix, |sb, a| sb + f(a)).len() == s.fold_left(prefix.len(), |i, a| i + f(a).len())
```
i.e., the length of sequence-fold equals the integer-fold of lengths.

---

## Results Summary

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes (3 errors) |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes (3 errors) |
| `logical_tests.rs` | 3 | ✅ Yes (3 errors) |

**Total: 9/9 tests correctly rejected by the specification.**

---

## Boundary Tests (boundary_tests.rs)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_1_empty_seq_off_by_one` | Assert fold of empty seq has length `prefix.len() + 1` instead of `prefix.len()` | ✅ FAILED |
| 2 | `test_boundary_2_nonempty_prefix_wrong_zero` | Assert fold of empty seq over non-empty prefix has length 0 | ✅ FAILED |
| 3 | `test_boundary_3_fold_shorter_than_prefix` | Assert fold result is strictly shorter than prefix | ✅ FAILED |

## Behavioral Mutation Tests (behavioral_mutation_tests.rs)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_1_plus_one` | Mutate postcondition: `fold_seq.len() == fold_int + 1` | ✅ FAILED |
| 2 | `test_mutation_2_minus_one` | Mutate postcondition: `fold_seq.len() == fold_int - 1` | ✅ FAILED |
| 3 | `test_mutation_3_wrong_initial_value` | Replace `prefix.len()` with `0` as fold base | ✅ FAILED |

## Logical Tests (logical_tests.rs)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_1_independent_of_s` | Assert fold length always equals `prefix.len()` (s-independence) | ✅ FAILED |
| 2 | `test_logical_2_f_independence` | Assert two different `f` functions yield same fold length | ✅ FAILED |
| 3 | `test_logical_3_strict_inequality` | Assert fold length is strictly greater than `prefix.len()` | ✅ FAILED |

---

## Conclusion

The specification correctly rejects all 9 adversarial properties:
- **Boundary**: Invalid edge-case claims (off-by-one, zero-length, shrinking fold) are rejected.
- **Behavioral mutations**: Mutated postconditions (+1, -1, wrong base) are rejected.
- **Logical over-claims**: Unentailed properties (s-independence, f-independence, strict inequality) are rejected.

The specification appears **consistent** — it does not entail any of the tested undesirable properties.
