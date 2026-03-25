# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_tla_forall_apply.rs`
**Date:** 2026-03-24T03:50:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `entails_trans` and `entails_preserved_by_always` are independent external_body axioms each bridging distinct intensional closure gaps — transitivity across chained implications and lifting entailment through always's forall-over-suffix, respectively. The remaining two are false positives: tla_forall instantiation is basic universal elimination, and chaining entails_trans is a redundant restatement of φ1's trust surface.

## True Positives (Spec Issues)

### entails_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `entails_trans` is external_body bridging transitivity across chained `implies`/`TempPred::new` closures. The SMT solver cannot verify that `p ⟹ q` and `q ⟹ r` compose to `p ⟹ r` through the intensional closure boundaries without this unverified axiom.

### entails_preserved_by_always_intensional_bridge
- **Confidence:** high
- **Reasoning:** `entails_preserved_by_always` is external_body lifting entailment through `always`, requiring the SMT solver to bridge `p.entails(q)` into the `forall`-over-suffix closure inside `always`. This is a separate unverified trust assumption from `entails_trans`.

## All Candidates

### φ1: entails_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_trans is external_body bridging three layers of TempPred::new/implies closures — the transitivity requires resolving intensional closure identity across chained implications without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_trans` is external_body bridging transitivity across chained `implies`/`TempPred::new` closures. The SMT solver cannot verify that `p ⟹ q` and `q ⟹ r` compose to `p ⟹ r` through the intensional closure boundaries without this unverified axiom.

### φ2: entails_preserved_by_always_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_preserved_by_always is external_body lifting entailment through always — bridging the forall-over-suffix closure inside always with the entailment closure without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_preserved_by_always` is external_body lifting entailment through `always`, requiring the SMT solver to bridge `p.entails(q)` into the `forall`-over-suffix closure inside `always`. This is a separate unverified trust assumption from `entails_trans`.

### φ3: tla_forall_entails_instance
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall instantiation requires bridging the forall-inside-TempPred::new closure to a specific a_to_p(a) — if the SMT solver resolves this intensional gap automatically, the external_body axioms may be redundant for this case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Universal instantiation — `(∀a. P(a)) ⟹ P(a)` — is a basic logical tautology. If the SMT solver resolves the `tla_forall` closure to extract a specific instance without external_body help, that's correct behavior, not a spec gap.

### φ4: always_tla_forall_to_tla_forall_always
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** commuting always and tla_forall requires that forall-over-suffix and forall-over-A can be swapped through nested TempPred::new closures — if this verifies, the axioms are strong enough to enable quantifier reordering without an explicit commutativity lemma

### φ5: entails_chain_collapses_arbitrary_intermediates
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** chaining entails_trans arbitrarily composes unverified closure-identity bridges — each application trusts the SMT solver resolves a new intensional gap, and unbounded chaining amplifies the trust surface
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just two applications of `entails_trans`, which is already identified as the trust assumption in φ1. Chaining a verified-correct transitivity rule is standard — no new trust surface is exposed beyond what φ1 already captures.

