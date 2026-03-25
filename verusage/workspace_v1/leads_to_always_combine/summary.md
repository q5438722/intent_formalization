# Test Execution Summary: `leads_to_always_combine.rs`

## Overview

19 adversarial proof tests were generated across three categories to probe the semantic boundaries of the temporal logic specification. **All 19 tests failed verification as expected**, indicating the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## (1) Boundary Tests — `boundary_tests.rs`

| Test | Target | Violation | Result |
|------|--------|-----------|--------|
| `test_boundary_1` | `implies_apply` | p⇒q not satisfied (p=true, q=false) | ✅ FAILED (precondition) |
| `test_boundary_2` | `implies_apply` | p not satisfied (p=false) | ✅ FAILED (precondition) |
| `test_boundary_3` | `always_propagate_forwards` | always(p) not satisfied at ex | ✅ FAILED (precondition) |
| `test_boundary_4` | `eventually_proved_by_witness` | p not satisfied at witness | ✅ FAILED (precondition) |
| `test_boundary_5` | `execution_equality` | executions differ (0 ≠ 1) | ✅ FAILED (precondition) |
| `test_boundary_6` | `leads_to_always_combine` | second `requires` missing | ✅ FAILED (precondition) |
| `test_boundary_7` | `leads_to_always_combine` | first `requires` missing | ✅ FAILED (precondition) |

**Result: 7/7 correctly rejected.** All preconditions are properly enforced.

---

## (2) Behavioral Mutation Tests — `mutation_tests.rs`

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_1` | `implies_apply`: assert q at `ex.suffix(1)` instead of `ex` | ✅ FAILED (assertion) |
| `test_mutation_2` | `always_propagate_forwards`: strengthen to `always(p.and(q))` | ✅ FAILED (assertion) |
| `test_mutation_3` | `eventually_proved_by_witness`: strengthen `eventually(p)` → `always(p)` | ✅ FAILED (assertion) |
| `test_mutation_4` | `always_and_equality`: claim distribution over `implies` (not valid) | ✅ FAILED (assertion) |
| `test_mutation_5` | `leads_to_always_combine`: drop `leads_to`, assert `always(q∧r)` directly | ✅ FAILED (assertion) |
| `test_mutation_6` | `leads_to_always_combine`: add extra conjunct `s` to conclusion | ✅ FAILED (assertion) |

**Result: 6/6 correctly rejected.** The ensures clauses are precise — no incorrect behavioral mutations pass.

---

## (3) Logical Tests — `logical_tests.rs`

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_1` | `eventually(p) ⇒ always(p)` | ✅ FAILED (assertion) |
| `test_logical_2` | leads_to symmetry: `p~>q ⇒ q~>p` | ✅ FAILED (assertion) |
| `test_logical_3` | `p~>always(q) ⇒ always(q)` directly | ✅ FAILED (assertion) |
| `test_logical_4` | reverse propagation: `always(p)` at suffix ⇒ at original | ✅ FAILED (assertion) |
| `test_logical_5` | `valid(eventually(p)) ⇒ valid(always(p))` | ✅ FAILED (assertion) |
| `test_logical_6` | entails symmetry: `spec⊨q ⇒ q⊨spec` | ✅ FAILED (assertion) |

**Result: 6/6 correctly rejected.** The specification does not allow unintended logical inferences.

---

## Conclusion

| Category | Tests | Passed (FAILED verification) | Unexpected PASS |
|----------|-------|------------------------------|-----------------|
| Boundary | 7 | 7 | 0 |
| Behavioral Mutation | 6 | 6 | 0 |
| Logical | 6 | 6 | 0 |
| **Total** | **19** | **19** | **0** |

The specification for `leads_to_always_combine` and its supporting axioms is **consistent** with respect to all 19 adversarial queries:

- **Preconditions** are tight — no invalid inputs are accepted.
- **Postconditions** are precise — no incorrect behaviors pass.
- **Logical boundaries** are sound — no unintended properties are derivable.

No specification weaknesses were detected.
