# Adversarial Test Summary: `leads_to_rank_step_one_help`

## Target
`leads_to_rank_step_one_help<T>` — proves that a descending leads-to chain (`∀n>0: p(n) ~> p(n-1)`) implies `p(n) ~> p(0)` by induction.

## Results: All 9/9 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (3/3 failed) — Precondition Violations

| Test | Property Tested | Result | Error Type |
|------|----------------|--------|------------|
| `test_no_chain_precondition` | Call without chain condition | ❌ FAILED | Precondition not satisfied |
| `test_partial_chain` | Only p(1)~>p(0), p(2)~>p(1) given; try p(3)~>p(0) | ❌ FAILED | Precondition not satisfied |
| `test_trans_missing_precondition` | leads_to_trans with one of two required premises | ❌ FAILED | Precondition not satisfied |

**Conclusion:** The `requires` clauses correctly guard against invalid inputs. Partial chain links do not satisfy the universal quantifier.

---

### Behavioral Mutation Tests (3/3 failed) — Mutated Outputs

| Test | Property Tested | Result | Error Type |
|------|----------------|--------|------------|
| `test_reverse_direction` | p(0) ~> p(3) instead of p(3) ~> p(0) | ❌ FAILED | Postcondition not satisfied |
| `test_leads_to_arbitrary_target` | p(3) ~> q for arbitrary unrelated q | ❌ FAILED | Postcondition not satisfied |
| `test_trans_reverse_conclusion` | r ~> p from p~>q, q~>r (should be p~>r) | ❌ FAILED | Postcondition not satisfied |

**Conclusion:** The spec correctly rejects mutated outputs. The downward chain does not entail upward leads-to, leads-to to arbitrary predicates, or reversed transitivity conclusions.

---

### Logical Tests (3/3 failed) — Unintended Reasoning

| Test | Property Tested | Result | Error Type |
|------|----------------|--------|------------|
| `test_valid_without_spec` | `valid(p(3).leads_to(p(0)))` instead of `spec.entails(...)` | ❌ FAILED | Postcondition not satisfied |
| `test_leads_to_symmetry` | p~>q implies q~>p (symmetry) | ❌ FAILED | Postcondition not satisfied |
| `test_self_leads_to_implies_universal` | p~>p implies p~>q for any q | ❌ FAILED | Postcondition not satisfied |

**Conclusion:** The spec does not permit unintended logical inferences. The distinction between `entails` (conditioned on spec) and `valid` (universal) is properly maintained. Leads-to is correctly non-symmetric, and self-reflexivity does not generalize.

---

## Overall Assessment

**The specification is consistent.** All 9 adversarial queries — spanning boundary violations, behavioral mutations, and logical overreach — were correctly rejected by the verification system. The spec:

1. **Guards inputs** via the universal quantifier in `requires`
2. **Constrains outputs** precisely to `p(n) ~> p(0)` under the given spec
3. **Prevents logical overreach** by maintaining the `entails` vs `valid` distinction and non-symmetry of leads-to
