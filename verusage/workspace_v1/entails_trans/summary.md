# Adversarial Proof Test Summary: `entails_trans.rs`

## Target Specification

The file defines temporal predicate entailment (`TempPred<T>.entails`) and proves **transitivity of entailment** (`entails_trans`), using an axiomatized modus ponens helper (`implies_apply`).

## Results: All 15 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no spec weakness detected.

---

### Boundary Tests (5/5 failed as expected)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| B1 | `entails_trans` without `p.entails(q)` | Precondition not satisfied |
| B2 | `entails_trans` without `q.entails(r)` | Precondition not satisfied |
| B3 | `entails_trans` with no preconditions | Precondition not satisfied |
| B4 | `implies_apply` without implication premise | Precondition not satisfied |
| B5 | `implies_apply` without satisfaction premise | Precondition not satisfied |

**Conclusion**: Both functions correctly guard against missing preconditions.

---

### Behavioral Mutation Tests (5/5 failed as expected)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| M1 | Reversed conclusion: `r.entails(p)` | Postcondition not satisfied |
| M2 | Partially reversed: `q.entails(p)` | Postcondition not satisfied |
| M3 | Strengthen to `valid(q)` from `p.entails(q)` | Postcondition not satisfied |
| M4 | Strengthen chain to `valid(r)` | Postcondition not satisfied |
| M5 | Reverse last step: `r.entails(q)` | Postcondition not satisfied |

**Conclusion**: Mutated outputs/relations are all correctly rejected. The spec does not over-promise.

---

### Logical Tests (5/5 failed as expected)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| L1 | Symmetry: `p⊨q → q⊨p` | Postcondition not satisfied |
| L2 | Common cause: `p⊨q ∧ p⊨r → q⊨r` | Postcondition not satisfied |
| L3 | Decomposition: `p⊨r → p⊨q` for arbitrary q | Postcondition not satisfied |
| L4 | Backward validity: `p⊨q → valid(p)` | Postcondition not satisfied |
| L5 | Wrong chaining: `p⊨q ∧ r⊨q → p⊨r` | Postcondition not satisfied |

**Conclusion**: The spec does not entail any unintended logical properties — no symmetry, no spurious decomposition, no backward validity inference.

---

## Overall Assessment

The `entails_trans` specification is **consistent**: it correctly rejects all 15 adversarial queries across boundary violations, behavioral mutations, and unintended logical inferences. The preconditions are tight (both required), postconditions are precise (only transitivity, nothing stronger), and no unintended reasoning paths were found.
