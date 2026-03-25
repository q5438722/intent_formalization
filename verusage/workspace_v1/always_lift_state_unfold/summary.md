# Adversarial Test Summary: `always_lift_state_unfold`

## Target Specification

- **`always_lift_state_unfold<T>(ex, p)`**: Unfolds `always(lift_state(p)).satisfied_by(ex)` into `∀i. p(ex.suffix(i).head())`
- Relies on trusted axiom `always_unfold` (`external_body`)

## Results: 9/9 tests FAILED verification ✅

All adversarial tests were correctly rejected, indicating the specification is **tight** for the properties tested.

### Boundary Tests (precondition violations)

| Test | Description | Result |
|------|-------------|--------|
| B1 | Call with no precondition | FAIL ✅ — precondition not satisfied |
| B2 | Call with only `p(ex.head())` | FAIL ✅ — head-only is insufficient for `always` |
| B3 | Call with finite prefix (3 positions) | FAIL ✅ — finite prefix insufficient for `always` |

### Behavioral Mutation Tests (mutated outputs)

| Test | Description | Result |
|------|-------------|--------|
| M1 | Assert negation `!p(ex.suffix(0).head())` | FAIL ✅ — contradicts postcondition |
| M2 | Assert result for unrelated predicate `q` | FAIL ✅ — only `p` is guaranteed |
| M3 | Assert result for different execution `ex2` | FAIL ✅ — only `ex1` is guaranteed |

### Logical Tests (unintended reasoning)

| Test | Description | Result |
|------|-------------|--------|
| L1 | Two executions with same `always` ⟹ same head | FAIL ✅ — no determinism guarantee |
| L2 | `always(lift_state(p))` ⟹ all states equal | FAIL ✅ — no structural collapse |
| L3 | `always(lift_state(p))` ⟹ all states > 0 | FAIL ✅ — no stronger property implied |

## Conclusion

The specification for `always_lift_state_unfold` correctly:
1. **Rejects invalid inputs** — precondition `always(lift_state(p)).satisfied_by(ex)` cannot be bypassed with weaker assumptions
2. **Rejects incorrect behaviors** — postcondition is precise; mutated relations (negation, wrong predicate, wrong execution) are all rejected
3. **Rejects unintended reasoning** — no spurious determinism, structural collapse, or stronger properties can be derived

No specification weaknesses were detected by these tests.
