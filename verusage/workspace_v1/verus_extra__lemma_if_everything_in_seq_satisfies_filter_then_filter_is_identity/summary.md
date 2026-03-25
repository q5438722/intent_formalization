# Test Execution Summary

**Target**: `lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity`  
**Spec**: `requires ∀i. 0 ≤ i < s.len() ⟹ pred(s[i])` / `ensures s.filter(pred) == s`

## Results: 15/15 tests FAILED verification ✅ (all as expected)

| ID  | Category              | Description                              | Result         |
|-----|-----------------------|------------------------------------------|----------------|
| B1  | Boundary              | Some elements don't satisfy pred         | FAIL ✅        |
| B2  | Boundary              | No elements satisfy pred                 | FAIL ✅        |
| B3  | Boundary              | Single element fails pred                | FAIL ✅        |
| B4  | Boundary              | Only last element fails pred             | FAIL ✅        |
| B5  | Boundary              | Only first element fails pred            | FAIL ✅        |
| M1  | Behavioral Mutation   | Assert filter ≠ s (negate ensures)       | FAIL ✅        |
| M2  | Behavioral Mutation   | Assert filter produces empty seq         | FAIL ✅        |
| M3  | Behavioral Mutation   | Assert filter has shorter length         | FAIL ✅        |
| M4  | Behavioral Mutation   | Assert filter equals reordered seq       | FAIL ✅        |
| M5  | Behavioral Mutation   | Assert filter changes element value      | FAIL ✅        |
| L1  | Logical               | Conclusion without calling lemma         | FAIL ✅        |
| L2  | Logical               | Pred holds for values outside seq        | FAIL ✅        |
| L3  | Logical               | Apply conclusion to different seq        | FAIL ✅        |
| L4  | Logical               | Apply conclusion with different pred     | FAIL ✅        |
| L5  | Logical               | Filter is identity for ANY predicate     | FAIL ✅        |

## Assessment

The specification is **well-formed and consistent**:

- **Boundary**: The `requires` clause correctly rejects all invalid inputs — sequences with elements not satisfying the predicate are blocked at call sites.
- **Behavioral**: The `ensures` clause (`filter == s`) is tight — it rejects negated equality, wrong lengths, reordered results, and mutated elements.
- **Logical**: The spec does not leak unintended conclusions — it does not allow deriving pred for out-of-range values, transferring results across sequences/predicates, or proving universal filter-identity claims.

No specification weaknesses were detected.
