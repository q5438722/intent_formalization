# Adversarial Test Summary: `leads_to_weaken`

## Target
`source-projects/anvil-library/verified/temporal_logic/leads_to_weaken.rs`

The function `leads_to_weaken` implements a standard temporal-logic weakening rule:
given `always(p2⇒p1)`, `always(q1⇒q2)`, and `p1 ~> q1`, conclude `p2 ~> q2`.

---

## Results

All **17 adversarial tests** correctly **FAILED** verification, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (6/6 failed ✓)

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| BT1 | Missing `always(p2⇒p1)` precondition | precondition not satisfied |
| BT2 | Missing `always(q1⇒q2)` precondition | precondition not satisfied |
| BT3 | Missing `p1 ~> q1` precondition | precondition not satisfied |
| BT4 | Reversed antecedent implication `p1⇒p2` | precondition not satisfied |
| BT5 | Reversed consequent implication `q2⇒q1` | precondition not satisfied |
| BT6 | All preconditions missing | precondition not satisfied |

**Conclusion**: All three preconditions are necessary; the spec correctly rejects reversed implications.

### Behavioral Mutation Tests (5/5 failed ✓)

| Test | Mutation | Verus Error |
|------|----------|-------------|
| BM1 | Reversed conclusion: `q2 ~> p2` | postcondition not satisfied |
| BM2 | Stronger conclusion: `always(p2⇒q2)` | postcondition not satisfied |
| BM3 | Swapped predicates: `q1 ~> p2` | postcondition not satisfied |
| BM4 | Wrong predicates: `q2 ~> p1` | postcondition not satisfied |
| BM5 | Overly strong: `always(q2)` | postcondition not satisfied |

**Conclusion**: The spec does not over-promise; mutated conclusions are all rejected.

### Logical Tests (6/6 failed ✓)

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| LT1 | Symmetry: `p ~> q` ⇒ `q ~> p` | postcondition not satisfied |
| LT2 | `p ~> q` ⇒ `always(q)` | postcondition not satisfied |
| LT3 | Identity weakening strengthens to `always(p⇒q)` | postcondition not satisfied |
| LT4 | Transitivity without second link | precondition not satisfied |
| LT5 | Entailment is symmetric | postcondition not satisfied |
| LT6 | Unrelated predicates inherit leads_to | postcondition not satisfied |

**Conclusion**: No unintended logical properties leak through the specification. Leads-to is not symmetric, does not collapse to always-implies, and cannot be fabricated for unrelated predicates.

---

## Overall Assessment

The specification of `leads_to_weaken` is **well-constrained**:
- **Preconditions are tight**: every required premise is necessary (no redundancy), and direction matters.
- **Postcondition is precise**: only `p2 ~> q2` is derivable — no stronger or mutated conclusions pass.
- **No logical leakage**: the temporal logic operators maintain proper semantic boundaries.

No specification weaknesses were detected.
