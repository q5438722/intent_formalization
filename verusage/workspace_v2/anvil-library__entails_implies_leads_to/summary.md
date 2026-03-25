# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/entails_implies_leads_to.rs`
**Date:** 2026-03-24T03:55:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `valid_p_implies_always_p` and `always_implies_to_leads_to` are independent external_body axioms each bridging distinct intensional closure gaps — valid-to-always lifting and always-implies-to-leads-to strengthening, respectively. The remaining three are false positives: spec-independent entailment of valid leads_to properties, ex falso on false specs, and reflexive leads_to are all standard temporal logic properties.

## True Positives (Spec Issues)

### valid_p_implies_always_p_intensional_bridge
- **Confidence:** high
- **Reasoning:** `valid_p_implies_always_p` is external_body bridging `valid(p)` to `valid(always(p))`, where `always` wraps `p` in a forall-over-suffix closure inside `TempPred::new`. This is an unverified trust assumption resolving the intensional closure gap.

### always_implies_to_leads_to_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_implies_to_leads_to` is external_body bridging `always(p.implies(q))` to `p.leads_to(q)` which expands to `always(p.implies(eventually(q)))`. The `eventually` wrapper introduces an existential-over-suffix closure that is intensionally distinct, and the strengthening from `q` to `eventually(q)` is semantically nontrivial — this is a separate unverified trust assumption.

## All Candidates

### φ1: valid_p_implies_always_p_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** valid_p_implies_always_p is external_body bridging valid(p) to valid(always(p)) — always wraps p in a forall-over-suffix closure inside TempPred::new, and the axiom silently resolves this intensional gap
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `valid_p_implies_always_p` is external_body bridging `valid(p)` to `valid(always(p))`, where `always` wraps `p` in a forall-over-suffix closure inside `TempPred::new`. This is an unverified trust assumption resolving the intensional closure gap.

### φ2: always_implies_to_leads_to_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_implies_to_leads_to is external_body bridging always(p→q) to always(p→eventually(q)) — the eventually(q) wrapping introduces an existential-over-suffix closure intensionally distinct from the direct implication
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_implies_to_leads_to` is external_body bridging `always(p.implies(q))` to `p.leads_to(q)` which expands to `always(p.implies(eventually(q)))`. The `eventually` wrapper introduces an existential-over-suffix closure that is intensionally distinct, and the strengthening from `q` to `eventually(q)` is semantically nontrivial — this is a separate unverified trust assumption.

### φ3: entails_leads_to_any_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if p entails q then ANY spec (including unsatisfiable or unrelated ones) entails p.leads_to(q) — the liveness property is derived without any reference to spec's content, making spec irrelevant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p.entails(q)` (i.e., `valid(p.implies(q))`), then `p.leads_to(q)` is `always(p.implies(eventually(q)))` which is valid because whenever p holds, q holds immediately (hence eventually). Any spec entailing a valid property is standard — the spec-independence is correct.

### φ4: false_spec_entails_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails leads_to — entails_implies_leads_to propagates this without checking spec satisfiability, potentially hiding specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a specific instance of φ3 with an unsatisfiable spec. Ex falso quodlibet — a false spec entailing anything is standard logic, not a spec gap.

### φ5: leads_to_self_trivial
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** p.leads_to(p) holds for any spec unconditionally with no preconditions at all — every property trivially leads to itself, meaning leads_to cannot distinguish any spec from any other for reflexive queries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p.leads_to(p)` is a standard temporal logic tautology — whenever p holds, p holds at suffix 0, so eventually(p) is satisfied. This is expected and desirable, not a spec gap.

