# Adversarial Test Summary: `or_leads_to_combine`

## Target Specification

`or_leads_to_combine<T>(spec, p, q, r)`:
- **Requires**: `spec ⊨ (p ~> r)` AND `spec ⊨ (q ~> r)`
- **Ensures**: `spec ⊨ ((p ∨ q) ~> r)`

## Results

**All 9 tests FAILED verification as expected.** The specification is strong enough to reject every adversarial query.

### Boundary Tests (`boundary_tests.rs`) — 3/3 FAILED ✓

| Test | Description | Result |
|------|-------------|--------|
| `test_boundary_missing_first_precond` | Omits `spec ⊨ (p ~> r)` | FAILED (precondition not satisfied) |
| `test_boundary_missing_second_precond` | Omits `spec ⊨ (q ~> r)` | FAILED (precondition not satisfied) |
| `test_boundary_wrong_leads_to_target` | Provides `spec ⊨ (p ~> q)` instead of `spec ⊨ (p ~> r)` | FAILED (precondition not satisfied) |

**Conclusion**: Both preconditions are necessary and correctly enforced. Substituting the leads_to target is also rejected.

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3/3 FAILED ✓

| Test | Description | Result |
|------|-------------|--------|
| `test_mutation_wrong_target` | Asserts `spec ⊨ ((p ∨ q) ~> p)` instead of `~> r` | FAILED (postcondition not satisfied) |
| `test_mutation_reverse_direction` | Asserts `spec ⊨ (r ~> (p ∨ q))` — reversed direction | FAILED (postcondition not satisfied) |
| `test_mutation_always_instead_of_leads_to` | Asserts `spec ⊨ □(p∨q → r)` — immediate, not eventual | FAILED (postcondition not satisfied) |

**Conclusion**: The postcondition precisely characterizes the output. Mutating the target, reversing direction, or strengthening from "eventually" to "always" are all rejected.

### Logical Tests (`logical_tests.rs`) — 3/3 FAILED ✓

| Test | Description | Result |
|------|-------------|--------|
| `test_logical_transitivity_misuse` | Derives `spec ⊨ (p ~> q)` from shared target r | FAILED (postcondition not satisfied) |
| `test_logical_spec_independence` | Derives `valid((p∨q) ~> r)` without spec guard | FAILED (postcondition not satisfied) |
| `test_logical_leads_to_not_symmetric` | Derives `spec ⊨ (r ~> p)` from `spec ⊨ (p ~> r)` | FAILED (postcondition not satisfied) |

**Conclusion**: The spec does not admit unintended logical inferences. leads_to is not treated as symmetric or transitive through shared targets, and spec-conditional results cannot be lifted to universal validity.

## Overall Assessment

The specification for `or_leads_to_combine` is **consistent**: it rejects all tested adversarial queries spanning precondition violations, output mutations, and unintended logical reasoning. No weaknesses were detected.
