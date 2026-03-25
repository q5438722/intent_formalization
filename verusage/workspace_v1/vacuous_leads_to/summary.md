# Adversarial Test Summary: `vacuous_leads_to`

## Target
`source-projects/anvil-library/verified/temporal_logic/vacuous_leads_to.rs`

**Theorem under test:** `vacuous_leads_to<T>(spec, p, q, r)`
- **Requires:** `spec ⊨ □r` and `p ∧ r ≡ false`
- **Ensures:** `spec ⊨ p ↝ q`
- **Semantics:** If `r` always holds under `spec` and `p ∧ r` is contradictory, then `p` is never satisfied under `spec`, making `p ↝ q` vacuously true.

---

## Results Summary

| File | Tests | All Failed? | Verdict |
|------|-------|-------------|---------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors) | Preconditions properly enforced |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5 errors) | Mutated outputs correctly rejected |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors) | Unintended reasoning correctly rejected |

**Total: 15/15 tests failed verification as expected.**

---

## Boundary Tests (5/5 FAIL ✅)

| Test | Failure Mode | Error Type |
|------|-------------|------------|
| `test_boundary_missing_always_r` | Omits `spec.entails(always(r))` | precondition not satisfied |
| `test_boundary_missing_false_pred` | Omits `p.and(r) == false_pred()` | precondition not satisfied |
| `test_boundary_both_missing` | Omits both preconditions | precondition not satisfied |
| `test_boundary_eventually_r_instead_of_always` | Weakens `always(r)` to `eventually(r)` | precondition not satisfied |
| `test_boundary_negated_r` | Replaces `r` with `not(r)` in always clause | precondition not satisfied |

**Finding:** The specification correctly rejects all invalid/weakened inputs. Preconditions are tight.

---

## Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Mutation | Error Type |
|------|----------|------------|
| `test_mutation_swap_leads_to` | `q ↝ p` instead of `p ↝ q` | postcondition not satisfied |
| `test_mutation_always_q` | `□q` instead of `p ↝ q` | postcondition not satisfied |
| `test_mutation_eventually_q` | `◇q` instead of `p ↝ q` | postcondition not satisfied |
| `test_mutation_entails_instead_of_leads_to` | `p ⊨ q` (pointwise) instead of temporal leads_to | postcondition not satisfied |
| `test_mutation_leads_to_not_q` | `p ↝ ¬q` instead of `p ↝ q` | postcondition not satisfied |

**Finding:** The specification rejects all output mutations. The ensures clause is precise — it cannot be substituted with stronger, weaker, or structurally different conclusions.

---

## Logical Tests (5/5 FAIL ✅)

| Test | Unintended Property | Error Type |
|------|---------------------|------------|
| `test_logical_unrestricted_validity` | `valid(p ↝ q)` (holds for ALL executions, not just spec) | postcondition not satisfied |
| `test_logical_always_not_p_from_ensures` | `spec ⊨ □¬p` from only `spec ⊨ p ↝ q` | postcondition not satisfied |
| `test_logical_implies_apply_missing_implication` | Misuse `implies_apply` without `p ⇒ q` established | precondition not satisfied |
| `test_logical_spec_is_true` | `spec == true_pred` (spec is trivially true) | postcondition not satisfied |
| `test_logical_r_entails_not_p_globally` | `valid(true ⇒ ¬p)` (¬p holds globally, not just under spec) | postcondition not satisfied |

**Finding:** The specification does not leak unintended logical consequences. Results are properly scoped to `spec`-satisfying executions, and no overly strong properties can be extracted.

---

## Conclusion

The `vacuous_leads_to` specification is **consistent**: it correctly rejects all 15 adversarial queries spanning boundary violations, behavioral mutations, and logical over-approximations. The preconditions are tight (no weakening allowed), the ensures clause is precise (no mutation passes), and no unintended reasoning is admitted.
