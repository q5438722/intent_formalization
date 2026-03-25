# Adversarial Test Results: `entails_and_temp`

**Target**: `source-projects/anvil-library/verified/temporal_logic/entails_and_temp.rs`
**Function under test**: `entails_and_temp<T>(spec, p, q)` — if `spec.entails(p)` and `spec.entails(q)`, then `spec.entails(p.and(q))`

---

## Summary

| Category                | Tests | Failed (expected) | Passed (unexpected) |
|-------------------------|-------|--------------------|----------------------|
| Boundary Tests          | 4     | 4 ✅               | 0                    |
| Behavioral Mutation     | 4     | 4 ✅               | 0                    |
| Logical Tests           | 4     | 4 ✅               | 0                    |
| **Total**               | **12**| **12**             | **0**                |

**Verdict**: The specification is **consistent** — all 12 adversarial properties were correctly rejected.

---

## Boundary Tests (4/4 FAILED ✅)

All tests violate preconditions and are correctly rejected:

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `boundary_missing_first_precondition` | Missing `spec.entails(p)` | precondition not satisfied ✅ |
| 2 | `boundary_missing_second_precondition` | Missing `spec.entails(q)` | precondition not satisfied ✅ |
| 3 | `boundary_no_preconditions` | Both preconditions absent | precondition not satisfied ✅ |
| 4 | `boundary_implies_apply_missing_implication` | `implies_apply` called without implication holding | precondition not satisfied ✅ |

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

All tests start from valid preconditions but assert mutated/incorrect postconditions:

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `mutation_valid_instead_of_entails` | Asserts `valid(p.and(q))` (universal validity) instead of conditional entailment | assertion failed ✅ |
| 2 | `mutation_reversed_entailment` | Asserts `p.and(q).entails(spec)` (reversed direction) | assertion failed ✅ |
| 3 | `mutation_extra_conjunct` | Asserts `spec.entails(p.and(q).and(r))` with unentailed `r` | assertion failed ✅ |
| 4 | `mutation_individual_validity` | Asserts `valid(p)` (promotes entailment to universal validity) | assertion failed ✅ |

---

## Logical Tests (4/4 FAILED ✅)

All tests probe unintended logical consequences that the spec does not guarantee:

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `logical_entails_not_symmetric` | Symmetry: `p.entails(q) → q.entails(p)` | assertion failed ✅ |
| 2 | `logical_no_arbitrary_implication` | `spec.entails(p) → spec.entails(p.implies(q))` without knowing `q` | assertion failed ✅ |
| 3 | `logical_and_entails_unrelated` | `spec.entails(p.and(q)) → spec.entails(r)` for unrelated `r` | assertion failed ✅ |
| 4 | `logical_no_universal_entailment` | Arbitrary `p.entails(q)` with no assumptions | assertion failed ✅ |

---

## Conclusion

The specification for `entails_and_temp` correctly:
- **Rejects invalid inputs**: All precondition violations are caught (boundary tests).
- **Rejects incorrect behaviors**: Mutated postconditions (stronger claims, reversed directions, extra conjuncts) are all rejected (behavioral mutation tests).
- **Rejects unintended reasoning**: The spec does not allow symmetry of entailment, promotion from entailment to validity, or entailment of unrelated predicates (logical tests).

No specification weaknesses were detected.
