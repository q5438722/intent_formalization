# Adversarial Test Summary: `leads_to_framed_by_or`

## Target Specification

```
leads_to_framed_by_or<T>(spec, p, q, r)
  requires: spec.entails(p.leads_to(q))
  ensures:  spec.entails(p.or(r).leads_to(q.or(r)))
```

A temporal logic framing rule: if `p` leads-to `q` under `spec`, then `p∨r` leads-to `q∨r` under `spec`.

---

## Results Overview

| File | Tests | All Failed? | Verdict |
|------|-------|-------------|---------|
| `boundary_tests.rs` | 4 | ✅ Yes (4 errors) | Spec correctly rejects invalid inputs |
| `mutation_tests.rs` | 4 | ✅ Yes (4 errors) | Spec correctly rejects mutated behaviors |
| `logical_tests.rs`  | 4 | ✅ Yes (4 errors) | Spec correctly rejects unintended reasoning |

**Total: 12/12 tests failed verification as expected.**

---

## Boundary Tests (4/4 FAILED ✅)

| # | Test | Failure Mode | Error Type |
|---|------|-------------|------------|
| 1 | `test_boundary_1_no_precondition` | Assert conclusion without any precondition | postcondition not satisfied |
| 2 | `test_boundary_2_call_without_requires` | Call `leads_to_framed_by_or` without satisfying `requires` | precondition not satisfied |
| 3 | `test_boundary_3_implies_without_implication` | Use `implies_apply` without implication holding | precondition not satisfied |
| 4 | `test_boundary_4_eventually_without_witness` | Use `eventually_proved_by_witness` without valid witness | precondition not satisfied |

**Conclusion**: All preconditions on the main lemma and helper axioms are properly enforced.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

| # | Test | Mutation | Error Type |
|---|------|----------|------------|
| 1 | `test_mutation_1_drop_or_from_rhs` | `q.or(r)` → `q` in RHS | assertion failed (inner proof breaks) |
| 2 | `test_mutation_2_swap_p_q` | Swap `p` ↔ `q` in framed conclusion | postcondition not satisfied |
| 3 | `test_mutation_3_replace_target` | Replace `q` with arbitrary `r` | postcondition not satisfied |
| 4 | `test_mutation_4_immediate_not_eventual` | `leads_to` → `always(implies)` (remove `eventually`) | postcondition not satisfied |

**Conclusion**: The specification correctly distinguishes between the true conclusion and structurally similar but incorrect variants. Notably, test 1 attempted the original proof strategy and failed precisely in the `else` branch where `r` holds but not `p` — the exact case where dropping `.or(r)` from the RHS breaks the argument.

---

## Logical Tests (4/4 FAILED ✅)

| # | Test | Invalid Reasoning | Error Type |
|---|------|------------------|------------|
| 1 | `test_logical_1_entails_to_valid` | `spec.entails(X)` → `valid(X)` (drop spec guard) | postcondition not satisfied |
| 2 | `test_logical_2_symmetry` | `p.leads_to(q)` → `q.leads_to(p)` (symmetry) | postcondition not satisfied |
| 3 | `test_logical_3_leads_to_implies_always` | `p.leads_to(q)` → `always(q)` (liveness → safety) | postcondition not satisfied |
| 4 | `test_logical_4_converse_framing` | `p∨r →* q∨r` → `p →* q` (reverse framing) | postcondition not satisfied |

**Conclusion**: The specification does not admit unintended logical inferences. It correctly distinguishes between relativized entailment and universal validity, rejects symmetry of leads_to, does not confuse liveness with safety, and does not support reverse framing.

---

## Overall Assessment

The specification for `leads_to_framed_by_or` is **well-constrained**:

- **Preconditions are enforced**: Invalid inputs are rejected (boundary tests).
- **Postcondition is precise**: Structurally mutated conclusions are rejected (mutation tests).
- **No unintended entailments**: The axioms do not allow reasoning beyond what is semantically justified (logical tests).

No spec weaknesses were detected. The specification correctly encodes the temporal logic framing rule without admitting any of the 12 adversarial properties tested.
