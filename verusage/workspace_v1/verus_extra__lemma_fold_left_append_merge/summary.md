# Adversarial Test Summary: `lemma_fold_left_append_merge`

## Specification Under Test

```
(s1 + s2).fold_left(empty, |acc, a| acc + f(a))
  == s1.fold_left(empty, |acc, a| acc + f(a)) + s2.fold_left(empty, |acc, a| acc + f(a))
```

No preconditions (`requires` is absent). The lemma states fold-left distributes over
sequence concatenation when the accumulator starts empty and the combining function appends `f(a)`.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes |
| `logical_tests.rs` | 3 | ✅ Yes |

**Total: 9/9 adversarial tests correctly rejected.**

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_nonempty_accumulator` | Distribution with non-empty init accumulator | ✅ FAILED (rejected) |
| 2 | `test_boundary_empty_fold_nonempty` | Fold of empty seq gives non-empty result | ✅ FAILED (rejected) |
| 3 | `test_boundary_nonempty_fold_gives_empty` | Fold of non-empty seq (with non-trivial f) gives empty | ✅ FAILED (rejected) |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Mutation Applied | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_reversed_order` | Swapped `fold(s1) + fold(s2)` → `fold(s2) + fold(s1)` | ✅ FAILED (rejected) |
| 2 | `test_mutation_missing_s2` | Dropped s2 contribution: `fold(s1+s2) == fold(s1)` | ✅ FAILED (rejected) |
| 3 | `test_mutation_drop_first_s1` | Off-by-one: used `s1[1..]` instead of `s1` | ✅ FAILED (rejected) |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_commutativity` | `fold(s1) + fold(s2) == fold(s2) + fold(s1)` | ✅ FAILED (rejected) |
| 2 | `test_logical_length_preservation` | `fold(s).len() == s.len()` | ✅ FAILED (rejected) |
| 3 | `test_logical_injectivity` | Equal folds ⟹ equal inputs | ✅ FAILED (rejected) |

---

## Conclusion

The specification is **consistent** with respect to all 9 adversarial queries:

- **Boundary**: The spec correctly scopes distribution to the empty initial accumulator and does not make false claims about fold results at edge cases.
- **Behavioral**: The spec rejects all mutated output relations (reversed order, dropped terms, off-by-one).
- **Logical**: The spec does not entail commutativity, length preservation, or injectivity—none of which are guaranteed by the lemma.

No specification weaknesses were detected.
