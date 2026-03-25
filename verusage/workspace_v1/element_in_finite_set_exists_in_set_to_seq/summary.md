# Adversarial Test Summary: `element_in_finite_set_exists_in_set_to_seq`

## Specification Under Test

```
proof fn element_in_finite_set_exists_in_set_to_seq<A>(s: Set<A>, e: A)
    requires s.finite(), s.contains(e),
    ensures  s.to_seq().contains(e),
```

Proves: if element `e` is in finite set `s`, then `e` appears in `s.to_seq()`.

---

## Results Summary

| Category | Tests | All Failed? | Spec Weakness Found? |
|---|---|---|---|
| Boundary | 4 | ✅ Yes (4/4 failed) | No |
| Behavioral Mutation | 4 | ✅ Yes (4/4 failed) | No |
| Logical | 5 | ✅ Yes (5/5 failed) | No |
| **Total** | **13** | **✅ All 13 failed** | **No** |

---

## Boundary Tests (4/4 FAILED ✅)

| Test | Failure Mode | Result |
|---|---|---|
| `test_boundary_infinite_set` | Infinite set violates `s.finite()` | ✅ precondition not satisfied |
| `test_boundary_element_not_in_set` | Element 99 not in {1,2,3} violates `s.contains(e)` | ✅ precondition not satisfied |
| `test_boundary_empty_set` | Empty set violates `s.contains(e)` | ✅ precondition not satisfied |
| `test_boundary_both_preconditions_violated` | Infinite set + element outside range | ✅ precondition not satisfied |

**Conclusion**: Both preconditions (`s.finite()` and `s.contains(e)`) are properly enforced. Invalid inputs are correctly rejected.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

| Test | Mutation | Result |
|---|---|---|
| `test_mutation_negate_postcondition` | Assert `!s.to_seq().contains(e)` | ✅ assertion failed |
| `test_mutation_absent_element_in_seq` | Assert `s.to_seq().contains(99)` for s={1,2,3} | ✅ assertion failed |
| `test_mutation_seq_is_empty` | Assert `s.to_seq().len() == 0` for non-empty set | ✅ assertion failed |
| `test_mutation_wrong_element_value` | Assert `s.to_seq()[0] == 20` for s={10} | ✅ assertion failed |

**Conclusion**: Incorrect behavioral claims are properly rejected. The spec does not allow wrong elements, empty sequences, or negated postconditions.

---

## Logical Tests (5/5 FAILED ✅)

| Test | Unintended Property | Result |
|---|---|---|
| `test_logical_seq_longer_than_set` | `to_seq().len() > s.len()` (stronger inequality) | ✅ assertion failed |
| `test_logical_ordering_guarantee` | `to_seq()[0] == 1` (deterministic ordering) | ✅ assertion failed |
| `test_logical_postcondition_without_lemma` | Conclusion without calling the lemma | ✅ assertion failed |
| `test_logical_cross_set_transfer` | Lemma on s1 implies property on s2 | ✅ assertion failed |
| `test_logical_no_duplicate_in_seq` | Uniqueness of elements in sequence | ✅ assertion failed |

**Conclusion**: The spec does not entail unintended logical properties. It does not leak ordering guarantees, cross-set reasoning, or stronger-than-stated inequalities. The postcondition is only available after explicitly invoking the lemma on the specific set.

---

## Overall Assessment

**The specification is consistent.** All 13 adversarial tests were correctly rejected by Verus:
- Preconditions guard against invalid inputs (infinite sets, missing elements, empty sets).
- The postcondition is tight — it only establishes `s.to_seq().contains(e)` and nothing more.
- No unintended logical consequences were discovered (no ordering, no uniqueness, no cross-set transfer).

The specification precisely captures its stated intent without admitting any of the tested undesirable properties.
