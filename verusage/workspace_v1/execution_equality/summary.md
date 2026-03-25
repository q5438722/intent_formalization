# Adversarial Test Results: `execution_equality.rs`

**Target**: `execution_equality<T>` — extensional equality for `Execution<T>` structs  
**Spec**: If two executions agree pointwise (`∀i. ex1(i) == ex2(i)`), then `ex1 == ex2`.

---

## Results Summary

| Category | Test | Expected | Actual | Result |
|----------|------|----------|--------|--------|
| Boundary | Partial equality (all but index 42) | FAIL | precondition not satisfied | ✅ PASS |
| Boundary | No relationship (arbitrary inputs) | FAIL | precondition not satisfied | ✅ PASS |
| Boundary | Finite range agreement (i < 100) | FAIL | precondition not satisfied | ✅ PASS |
| Behavioral | Assert inequality after valid proof | FAIL | assertion failed | ✅ PASS |
| Behavioral | Wrong state value (0 ≠ 1) | FAIL | assertion failed | ✅ PASS |
| Behavioral | Equality with unrelated third execution | FAIL | assertion failed | ✅ PASS |
| Logical | Universal equality (no precondition) | FAIL | postcondition not satisfied | ✅ PASS |
| Logical | Self-equality implies constant function | FAIL | postcondition not satisfied | ✅ PASS |
| Logical | Existence of zero state | FAIL | postcondition not satisfied | ✅ PASS |

**Overall: 9/9 adversarial tests correctly rejected.**

---

## Analysis

### Boundary Tests (3/3 rejected)
The specification correctly requires **full pointwise agreement** across all natural number indices. Partial agreement (all-but-one), no agreement, and finite-range agreement are all properly rejected. The precondition `∀i: nat. ex1(i) == ex2(i)` is strict and cannot be weakened.

### Behavioral Mutation Tests (3/3 rejected)
The spec rejects mutated conclusions:
- **Negated output**: `ex1 != ex2` contradicts the proven `ex1 == ex2`.
- **Wrong state value**: Pointwise agreement + known value at index 0 correctly propagates through equality.
- **Unrelated third party**: Equality of `ex1` and `ex2` does not extend to arbitrary `ex3`.

### Logical Tests (3/3 rejected)
The spec does not entail unintended logical properties:
- **Universal equality**: Cannot prove arbitrary executions are equal (no vacuous truth leak).
- **Constant function**: Self-equality (`ex == ex`) does not imply the execution is constant.
- **Existence of zero**: No existential state properties are smuggled in.

---

## Conclusion

The specification for `execution_equality` is **tight and well-constrained**. It admits exactly the intended reasoning (extensional equality from pointwise agreement) and rejects all tested adversarial queries across boundary, behavioral, and logical dimensions. No specification weaknesses were detected.
