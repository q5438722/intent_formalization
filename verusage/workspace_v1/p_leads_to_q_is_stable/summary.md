# Test Execution Summary: `p_leads_to_q_is_stable`

## Target Specification

The spec proves that the temporal "leads-to" relation (`p ~> q`) is **stable**: once it holds for an execution, it holds at all future suffixes. This is proved by reducing to the axiom that `always(p)` is stable.

Key constructs:
- `always_p_is_stable(p)` — axiom: `valid(stable(always(p)))`
- `p_leads_to_q_is_stable(p, q)` — theorem: `valid(stable(p.leads_to(q)))`

---

## Results Overview

| Category | Tests | All Failed? | Spec Adequate? |
|---|---|---|---|
| Boundary | 3 | ✅ Yes | ✅ |
| Behavioral Mutation | 3 | ✅ Yes | ✅ |
| Logical | 3 | ✅ Yes | ✅ |

**All 9 tests failed verification as expected.** The specification correctly rejects every unintended property.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_arbitrary_predicate_is_stable` | `valid(stable(p))` for arbitrary `p` | ❌ FAILED (correct) |
| 2 | `test_eventually_is_stable` | `valid(stable(eventually(p)))` | ❌ FAILED (correct) |
| 3 | `test_leads_to_is_unconditionally_valid` | `valid(p.leads_to(q))` unconditionally | ❌ FAILED (correct) |

**Analysis:** The spec correctly distinguishes `always(p)` (which is stable) from arbitrary or `eventually` predicates (which are not). It also correctly separates stability (conditional persistence) from unconditional validity.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_swap_leads_to_direction` | `valid(stable(q.leads_to(p)))` from `p_leads_to_q_is_stable(p,q)` | ❌ FAILED (correct) |
| 2 | `test_strengthen_stable_to_always` | `valid(always(p.leads_to(q)))` | ❌ FAILED (correct) |
| 3 | `test_leads_to_at_specific_execution` | `p.leads_to(q).satisfied_by(ex)` for arbitrary `ex` | ❌ FAILED (correct) |

**Analysis:** The spec correctly rejects mutations: reversing the leads-to direction, strengthening stable to unconditional always, and instantiating leads-to at arbitrary executions. The ensures clause is tightly scoped to its parameters.

---

## Logical Tests (`logical_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_leads_to_implies_eventual_q` | `p ~> q` implies `eventually(q)` without assuming `p` | ❌ FAILED (correct) |
| 2 | `test_misuse_always_stable_for_plain_pred` | Using `always_p_is_stable` to get `stable(p)` | ❌ FAILED (correct) |
| 3 | `test_leads_to_transitivity` | `(p ~> q) ∧ (q ~> r) ⟹ (p ~> r)` | ❌ FAILED (correct) |

**Analysis:** The spec does not admit unintended logical inferences. The leads-to relation does not magically guarantee `eventually(q)` without `p` holding. The `always_p_is_stable` axiom cannot be misused for non-always predicates. Transitivity of leads-to is not derivable from stability alone — it would require a separate proof.

---

## Conclusion

The specification for `p_leads_to_q_is_stable` is **well-scoped and consistent** with respect to all tested semantic queries. No unintended entailments were discovered across boundary, behavioral, or logical dimensions.
