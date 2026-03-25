# Adversarial Test Summary: `injective_finite_map_implies_dom_len_is_equal_to_values_len`

## Target Specification

```
requires: m.dom().finite(), m.is_injective()
ensures:  m.dom().len() == m.values().len()
```

## Results: All 9 tests FAILED verification ✅ (as expected)

| # | File | Test | Failure Mode | Result |
|---|------|------|-------------|--------|
| 1 | boundary_tests.rs | `test_boundary_non_injective` | Violates `m.is_injective()` — two keys map to same value | ✅ FAILED (precondition not satisfied) |
| 2 | boundary_tests.rs | `test_boundary_infinite_injective` | Violates `m.dom().finite()` — infinite domain | ✅ FAILED (precondition not satisfied) |
| 3 | boundary_tests.rs | `test_boundary_both_violated` | Violates both preconditions | ✅ FAILED (precondition not satisfied) |
| 4 | behavioral_mutation_tests.rs | `test_mutation_dom_strictly_greater` | Mutates `==` to `>` | ✅ FAILED (assertion failed) |
| 5 | behavioral_mutation_tests.rs | `test_mutation_not_equal` | Mutates `==` to `!=` | ✅ FAILED (assertion failed) |
| 6 | behavioral_mutation_tests.rs | `test_mutation_off_by_one` | Mutates `len` to `len + 1` | ✅ FAILED (assertion failed) |
| 7 | logical_tests.rs | `test_logical_holds_without_injectivity` | Omits injectivity requirement | ✅ FAILED (assertion failed) |
| 8 | logical_tests.rs | `test_logical_equal_len_not_equal_sets` | Equal cardinality ≠ equal sets | ✅ FAILED (assertion failed) |
| 9 | logical_tests.rs | `test_logical_nonemptiness_not_guaranteed` | Spec doesn't guarantee non-emptiness | ✅ FAILED (assertion failed) |

## Analysis

The specification is **well-formed and tight**:

- **Boundary control**: Both preconditions (`finite`, `injective`) are enforced — invalid inputs are correctly rejected.
- **Behavioral correctness**: The postcondition `dom.len == values.len` is precise — mutated relations (`>`, `!=`, off-by-one) are all rejected.
- **Logical tightness**: The spec does not allow unintended reasoning:
  - It does not over-generalize to non-injective maps.
  - It does not conflate equal cardinality with set equality.
  - It does not implicitly assume non-emptiness.

**Conclusion**: No spec weaknesses detected. All adversarial queries were correctly rejected.
