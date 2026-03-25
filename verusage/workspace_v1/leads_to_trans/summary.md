# Adversarial Test Summary: `leads_to_trans.rs`

## Target
Temporal logic `leads_to_trans` theorem: transitivity of the leads-to relation (`p ~> q ∧ q ~> r ⟹ p ~> r`) under a specification context.

## Results: 13/13 tests FAILED verification (as expected)

All adversarial queries were correctly rejected by the specification.

---

### Boundary Tests (5/5 failed ✅)

| Test | What it probes | Failure mode |
|------|---------------|--------------|
| B1: `missing_p_leads_to_q` | Remove first precondition `spec ⊨ (p ~> q)` | Precondition of `leads_to_trans` not satisfied |
| B2: `missing_q_leads_to_r` | Remove second precondition `spec ⊨ (q ~> r)` | Precondition of `leads_to_trans` not satisfied |
| B3: `no_preconditions` | Remove both preconditions | Precondition of `leads_to_trans` not satisfied |
| B4: `implies_no_antecedent` | Call `implies_apply` without `p.satisfied_by(ex)` | Precondition of `implies_apply` not satisfied |
| B5: `entails_no_entailment` | Call `entails_apply` without `p.entails(q)` | Precondition of `entails_apply` not satisfied |

**Conclusion:** All preconditions are necessary; none can be dropped.

---

### Behavioral Mutation Tests (4/4 failed ✅)

| Test | Mutation | Failure mode |
|------|----------|--------------|
| M1: `reversed_conclusion` | Conclude `r ~> p` instead of `p ~> r` | Postcondition not satisfied |
| M2: `drop_eventually` | Conclude `always(p ⟹ r)` instead of `always(p ⟹ ◇r)` | Postcondition not satisfied |
| M3: `drop_spec` | Conclude `valid(p ~> r)` instead of `spec ⊨ (p ~> r)` | Postcondition not satisfied |
| M4: `conclude_always` | Conclude `spec ⊨ □r` instead of `spec ⊨ (p ~> r)` | Postcondition not satisfied |

**Conclusion:** The specification precisely characterizes the output; no incorrect behavioral variant is admitted.

---

### Logical Tests (4/4 failed ✅)

| Test | Unwarranted property | Failure mode |
|------|---------------------|--------------|
| L1: `symmetry` | `p ~> q` implies `q ~> p` | Postcondition not satisfied |
| L2: `eventually_to_always` | `◇p` implies `□p` | Postcondition not satisfied |
| L3: `leads_to_no_antecedent` | `p ~> q` satisfied implies `◇q` without `p` holding | Postcondition not satisfied |
| L4: `always_not_valid` | `□p` at one execution implies `valid(p)` across all executions | Postcondition not satisfied |

**Conclusion:** The specification does not entail any of the tested unwarranted logical properties. Temporal modalities (always, eventually, leads_to) maintain correct semantic distinctions.

---

## Overall Assessment

The specification for `leads_to_trans` and its supporting axioms is **consistent** with respect to all 13 adversarial queries:
- **Preconditions are tight** — every `requires` clause is necessary.
- **Postconditions are precise** — no behavioral mutation is admitted.
- **No unintended logical entailments** — the temporal logic semantics are correctly captured.
