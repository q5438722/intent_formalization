# Adversarial Test Summary: `some_differing_index_for_unequal_seqs`

## Specification Under Test

```
proof fn some_differing_index_for_unequal_seqs<A>(s1: Seq<A>, s2: Seq<A>) -> (i: int)
  requires s1 != s2, s1.len() == s2.len(),
  ensures  0 <= i < s1.len(), s1[i] != s2[i],
```

Given two unequal sequences of equal length, returns an index where they differ.

---

## Results Overview

| Category               | Tests | Failed (expected) | Passed (unexpected) |
|------------------------|-------|--------------------|----------------------|
| Boundary Tests         | 4     | 4 ✅               | 0                    |
| Behavioral Mutation    | 4     | 4 ✅               | 0                    |
| Logical Tests          | 4     | 4 ✅               | 0                    |
| **Total**              | **12**| **12**             | **0**                |

All 12 adversarial tests were **correctly rejected** by Verus.

---

## Boundary Tests (`boundary_tests.rs`) — 4/4 FAILED ✅

| # | Test                          | Violated Precondition       | Result        |
|---|-------------------------------|-----------------------------|---------------|
| 1 | Equal sequences `[1,2,3]`     | `s1 != s2`                  | FAIL ✅       |
| 2 | Different lengths `[1,2,3]` vs `[4,5]` | `s1.len() == s2.len()` | FAIL ✅  |
| 3 | Empty equal sequences         | `s1 != s2`                  | FAIL ✅       |
| 4 | One empty, one non-empty      | `s1 != s2` (checked first)  | FAIL ✅       |

**Finding:** All invalid inputs are properly rejected. The preconditions form a complete guard.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✅

| # | Test                                  | Mutated Property         | Result        |
|---|---------------------------------------|--------------------------|---------------|
| 1 | Assert `s1[i] == s2[i]`              | Negated `s1[i] != s2[i]` | FAIL ✅       |
| 2 | Assert `i >= s1.len()`               | Negated `i < s1.len()`   | FAIL ✅       |
| 3 | Assert `i < 0`                       | Negated `0 <= i`          | FAIL ✅       |
| 4 | Assert `i == 0` (only idx 2 differs) | Wrong specific index      | FAIL ✅       |

**Finding:** All incorrect output relationships are properly rejected. The postconditions correctly constrain the return value.

---

## Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✅

| # | Test                               | Unentailed Property       | Result        |
|---|------------------------------------|---------------------------|---------------|
| 1 | Assert minimum differing index     | Minimality                | FAIL ✅       |
| 2 | Assert only one position differs   | Uniqueness of difference  | FAIL ✅       |
| 3 | Assert `s1[i] > s2[i]`            | Stronger ordering         | FAIL ✅       |
| 4 | Assert commutativity of index      | Argument-order invariance | FAIL ✅       |

**Finding:** The specification does not entail any unintended stronger properties. It correctly provides only the guaranteed `!=` relationship at a valid index, without implying minimality, uniqueness, ordering, or commutativity.

---

## Conclusion

The specification for `some_differing_index_for_unequal_seqs` is **consistent** with respect to all tested adversarial queries:

1. **Input validation is sound** — preconditions reject all invalid inputs.
2. **Output constraints are tight** — mutated postconditions are properly rejected.
3. **No unintended entailments** — the spec does not over-commit; it guarantees exactly what it states (existence of a differing index) and nothing more.

The specification exhibits good semantic precision: it is strong enough to be useful (guarantees a valid differing index) while remaining minimal (does not impose unnecessary constraints like minimality or ordering).
