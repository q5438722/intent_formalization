# Adversarial Test Summary: `seq_unequal_preserved_by_add_auto`

## Specification Under Test

```
seq_unequal_preserved_by_add<A>(s1, s2, suffix)
  requires: s1 != s2
  ensures:  s1 + suffix != s2 + suffix

seq_unequal_preserved_by_add_auto<A>(suffix)
  ensures: forall |s1, s2| s1 != s2 ==> s1 + suffix != s2 + suffix
```

The specification states that appending the same suffix to two unequal sequences preserves their inequality.

---

## Results Summary

| # | Test | Type | Expected | Actual | Status |
|---|------|------|----------|--------|--------|
| 1 | `test_boundary_equal_empty_sequences` | Boundary | FAIL | FAIL (precondition) | ✅ |
| 2 | `test_boundary_equal_nonempty_sequences` | Boundary | FAIL | FAIL (precondition) | ✅ |
| 3 | `test_boundary_structurally_equal_sequences` | Boundary | FAIL | FAIL (precondition) | ✅ |
| 4 | `test_boundary_auto_with_equal_sequences` | Boundary | FAIL | FAIL (assertion) | ✅ |
| 5 | `test_mutation_negated_postcondition` | Behavioral | FAIL | FAIL (assertion) | ✅ |
| 6 | `test_mutation_wrong_length_relation` | Behavioral | FAIL | FAIL (assertion) | ✅ |
| 7 | `test_mutation_equality_different_suffix` | Behavioral | FAIL | FAIL (assertion) | ✅ |
| 8 | `test_mutation_wrong_element_relation` | Behavioral | FAIL | FAIL (assertion) | ✅ |
| 9 | `test_logical_length_inequality_from_seq_inequality` | Logical | FAIL | FAIL (assertion) | ✅ |
| 10 | `test_logical_first_element_inequality` | Logical | FAIL | FAIL (assertion) | ✅ |
| 11 | `test_logical_suffix_elements_differ` | Logical | FAIL | FAIL (assertion) | ✅ |
| 12 | `test_logical_all_same_length_unequal` | Logical | FAIL | FAIL (assertion) | ✅ |

**All 12/12 tests FAILED as expected.**

---

## Analysis

### Boundary Tests (4/4 rejected)
The precondition `requires s1 != s2` correctly rejects:
- Equal empty sequences
- Equal non-empty sequences
- Structurally identical sequences constructed independently
- Attempts to derive false inequality for equal sequences via the auto lemma

### Behavioral Mutation Tests (4/4 rejected)
The postcondition correctly rejects:
- Negated postcondition (asserting equality instead of inequality after concatenation)
- Wrong length relationships (asserting lengths differ when they don't)
- Incorrect transfer to different suffixes (trying to conclude equality with a suffix not covered by the call)
- Wrong element-level claims (asserting first elements differ when sequences share a prefix)

### Logical Tests (4/4 rejected)
The specification does not allow:
- Deriving length inequality from sequence inequality (false: [1] ≠ [2] but len([1]) == len([2]))
- Deriving first-element inequality from sequence inequality (false: [1,2] ≠ [1,3] but first elements match)
- Deriving element-wise inequality at suffix positions (suffix portions are identical)
- Deriving that all same-length sequences are unequal (absurd universal claim)

## Conclusion

The specification is **consistent** with respect to all tested adversarial queries. It correctly:
1. **Rejects invalid inputs** via the `requires s1 != s2` precondition
2. **Rejects incorrect behaviors** — the postcondition is precise and non-vacuous
3. **Does not entail unintended logical properties** — stronger claims about lengths, elements, or universality are all rejected

No specification weaknesses were identified.
