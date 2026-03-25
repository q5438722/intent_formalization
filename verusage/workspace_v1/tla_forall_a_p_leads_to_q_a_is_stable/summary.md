# Adversarial Test Results Summary

**Target**: `tla_forall_a_p_leads_to_q_a_is_stable.rs`  
**Spec under test**: If each individual `p ~> q(a)` is stable (for all `a`), then `∀a. p ~> q(a)` is also stable.

## Overall Result: All 9 tests correctly FAILED ✅

The specification correctly rejects all adversarial properties — no weaknesses detected.

---

## Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| B1: Derive false | Use lemma with correct precondition to derive `false` | postcondition not satisfied |
| B2: No precondition | Call lemma without establishing `∀a. valid(stable(p ~> q(a)))` | precondition not satisfied |
| B3: Single instance | Assume stability for only `a=0`, not all `a` | precondition not satisfied |

**Analysis**: The precondition correctly guards the lemma. Even though `valid(stable(leads_to))` is semantically always true, the SMT solver cannot prove it without explicit reasoning about closure composition and suffix properties (B2, B3). The postcondition does not introduce inconsistency (B1).

---

## Behavioral Mutation Tests (3/3 failed ✅)

| Test | Mutation | Verus Error |
|------|----------|-------------|
| M1: Drop `stable` | `valid(∀a. p ~> q(a))` instead of `valid(stable(∀a. p ~> q(a)))` | postcondition not satisfied |
| M2: `always` for `stable` | `valid(□(∀a. p ~> q(a)))` instead of `valid(stable(∀a. p ~> q(a)))` | postcondition not satisfied |
| M3: Simultaneous leads-to | `valid(p ~> ∀a.q(a))` instead of `valid(stable(∀a. p ~> q(a)))` | postcondition not satisfied |

**Analysis**: The spec precisely characterizes stability of the universally quantified leads-to. Dropping the `stable` wrapper (M1), strengthening to `always` (M2), or asserting simultaneous eventuality instead of individual (M3) are all correctly rejected. The key distinction: `stable(X)` is `X ⟹ □X` (vacuously true when X fails), which is categorically weaker than `X` or `□X`.

---

## Logical Tests (3/3 failed ✅)

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| L1: Derive `valid(p)` | Spec does not make `p` universally true | postcondition not satisfied |
| L2: Individual leads-to | `valid(p ~> q(42))` from universal stability — not derivable | postcondition not satisfied |
| L3: Simultaneous eventually | `valid(◇(∀a. q(a)))` — all q's hold at once eventually | postcondition not satisfied |

**Analysis**: The spec does not leak unintended consequences. The precondition `valid(stable(p ~> q(a)))` is semantically a tautology (leads-to is inherently stable), so it provides no information about whether `p` holds (L1), whether individual leads-to properties actually hold (L2), or whether all q predicates are simultaneously satisfiable (L3).

---

## Conclusion

The specification for `tla_forall_a_p_leads_to_q_a_is_stable` is **consistent** with respect to all tested semantic boundaries:
- **Preconditions** correctly guard against unproven invocations and partial quantification
- **Behavioral mutations** (dropping stable, strengthening to always, restructuring leads-to) are all rejected
- **Logical overreach** (deriving predicate validity, individual properties, simultaneous eventuality) is blocked

No spec weaknesses were identified across 9 adversarial tests covering 9 distinct failure modes.
