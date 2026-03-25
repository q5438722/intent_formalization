# Adversarial Test Results Summary

**Target**: `tla_forall_always_implies_equality2.rs`  
**Spec under test**: `tla_forall(|a| always(p.implies(a_to_q(a)))) == always(p.implies(tla_forall(a_to_q)))`

## Overall Result: All 11 tests correctly FAILED ✅

The specification correctly rejects all adversarial properties — no weaknesses detected.

---

## Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| B1: Missing `always` wrapper | Call axiom with `a_to_always = a_to_p` (no `always`), violating `a_to_p(a) ⊨ always(a_to_p(a))` | precondition not satisfied |
| B2: Derive false | Use main function's postcondition to derive `false` | postcondition not satisfied |
| B3: One-directional entailment | Call axiom with `a_to_always(a) = always(q)→always(p)` where only backward entailment holds | precondition not satisfied |

**Analysis**: The `requires` clause of `tla_forall_always_equality_variant` correctly enforces bidirectional entailment. Invalid inputs (B1, B3) are rejected at the call site. The equality postcondition does not introduce contradictions (B2).

---

## Behavioral Mutation Tests (4/4 failed ✅)

| Test | Mutation | Verus Error |
|------|----------|-------------|
| M1: Assert inequality | `LHS != RHS` instead of `LHS == RHS` | postcondition not satisfied |
| M2: Drop `always` from LHS | `tla_forall(\|a\| p→q(a))` instead of `tla_forall(\|a\| always(p→q(a)))` | postcondition not satisfied |
| M3: Swap implication | `q(a)→p` instead of `p→q(a)` inside `always` | postcondition not satisfied |
| M4: Specific value for ∀ | `a_to_q(specific_a)` instead of `tla_forall(a_to_q)` on RHS | postcondition not satisfied |

**Analysis**: The spec precisely characterizes the equality. Removing the temporal operator (M2), reversing logical direction (M3), weakening universal to existential (M4), or negating the result (M1) are all correctly rejected.

---

## Logical Tests (4/4 failed ✅)

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| L1: Derive `valid(p)` | Spec does not make `p` universally true | postcondition not satisfied |
| L2: `p ⊨ always(p)` | Holding now ≠ holding forever | postcondition not satisfied |
| L3: `□(p→q) ⊨ p→□q` | Pointwise implication ≠ global consequence | postcondition not satisfied |
| L4: `∀a.q(a) ⊨ □∀a.q(a)` | Quantification at one state ≠ persistence | postcondition not satisfied |

**Analysis**: The spec does not leak unintended temporal or logical consequences. The `always` operator is not conflated with single-state truth (L2), pointwise properties don't collapse into global ones (L3), and quantification over parameters doesn't imply persistence over time (L4).

---

## Conclusion

The specification for `tla_forall_always_implies_equality2` is **consistent** with respect to all tested semantic boundaries:
- **Preconditions** correctly guard the axiom against invalid invocations
- **Behavioral mutations** (negation, operator removal, direction swap, quantifier weakening) are all rejected
- **Logical overreach** (deriving validity, temporal persistence, or distributivity not stated) is blocked

No spec weaknesses were identified across 11 adversarial tests covering 11 distinct failure modes.
