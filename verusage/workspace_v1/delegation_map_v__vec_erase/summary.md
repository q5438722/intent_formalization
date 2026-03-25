# Adversarial Test Summary — `delegation_map_v__vec_erase`

## Target Specification

```rust
pub fn vec_erase<A>(v: &mut Vec<A>, start: usize, end: usize)
    requires start <= end <= old(v).len(),
    ensures  v@ == old(v)@.subrange(0, start as int)
                 + old(v)@.subrange(end as int, old(v)@.len() as int),
```

Erases elements in the range `[start, end)` from vector `v`.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes |
| `logical_tests.rs` | 3 | ✅ Yes |

**Total: 9/9 tests correctly rejected by the specification.**

---

## Boundary Tests (precondition violations)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_start_greater_than_end` | `start=3 > end=1` violates `start <= end` | ✅ FAIL |
| 2 | `test_boundary_end_exceeds_length` | `end=5 > v.len()=3` violates `end <= v.len()` | ✅ FAIL |
| 3 | `test_boundary_empty_vec_nonzero_range` | `end=1 > v.len()=0` on empty vec | ✅ FAIL |

**Conclusion:** The precondition `0 <= start <= end <= v.len()` correctly rejects all invalid input combinations.

---

## Behavioral Mutation Tests (incorrect output assertions)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_length_unchanged` | Claimed `r.len() == 5` after erasing 2 elements (correct: 3) | ✅ FAIL |
| 2 | `test_mutation_erased_element_present` | Claimed erased element `20` at `r[1]` (correct: `40`) | ✅ FAIL |
| 3 | `test_mutation_result_equals_original` | Claimed `r == v` after erasing `[0, 2)` | ✅ FAIL |

**Conclusion:** The postcondition (exact sequence equality) correctly rejects all mutated output claims.

---

## Logical Tests (non-entailed properties)

| # | Test | False Claim | Result |
|---|------|-------------|--------|
| 1 | `test_logical_noop_erase_shortens` | No-op erase (`start==end`) shortens the vector | ✅ FAIL |
| 2 | `test_logical_different_inputs_same_output` | Different inputs with same erase range produce identical results | ✅ FAIL |
| 3 | `test_logical_idempotency` | Erasing `[1,3)` twice is idempotent | ✅ FAIL |

**Conclusion:** The spec does not admit unintended logical inferences—false structural claims (stronger inequalities, cross-input equivalence, idempotency) are all rejected.

---

## Overall Assessment

The specification of `vec_erase` is **tight and well-formed**:

- **Preconditions** properly guard against invalid index ranges.
- **Postcondition** fully determines the result via exact sequence equality (`v.subrange(0, start) + v.subrange(end, v.len())`), leaving no room for ambiguity.
- No unintended logical consequences were admitted.

The spec's use of exact sequence equality (rather than weaker properties like length or multiset equality) makes it particularly robust against both behavioral mutations and logical over-reasoning.
