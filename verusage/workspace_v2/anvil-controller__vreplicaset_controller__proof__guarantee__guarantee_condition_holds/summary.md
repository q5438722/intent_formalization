# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__guarantee__guarantee_condition_holds/original.rs`
**Date:** 2026-03-24T05:18:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. ConfigMap's status is unit type `()` so discarding it during unmarshal is correct. DaemonSet's transition validation correctly models Kubernetes semantics where only selector is immutable while template changes are the standard update path.

## All Candidates

### φ1: init_invariant_without_next
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** init_invariant requires both the init condition AND an inductive next-step — without the next-step obligation, an invariant holding at init should NOT be guaranteed to hold always

### φ2: configmap_unmarshal_ignores_status
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** _unmarshal_helper discards status entirely — but unmarshal_status is uninterpreted so the solver cannot know both calls produce the same Ok_0 value; if it can prove equality despite different status values, the status field is provably irrelevant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** ConfigMap has `EmptyStatusView = ()` as status type. The `_unmarshal_helper` correctly discards status because ConfigMap status is unit type — there's nothing meaningful to preserve. The equality holds because both unmarshal results use the same metadata and spec.

### φ3: daemonset_transition_allows_template_change
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** transition_validation only checks selector equality — if template changes are freely allowed by the spec, that may be too permissive (real Kubernetes restricts certain template mutations)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This matches real Kubernetes semantics — DaemonSet transition validation only requires selector immutability. Template changes (e.g., updating container images) are explicitly allowed and are the primary mechanism for rolling updates.

### φ4: with_namespace_preserves_object_ref_name
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** the last ensures claims namespaces always differ — but if ns happens to equal the original namespace, this is false; the spec should not entail that with_namespace always changes the namespace

### φ5: entails_trans_collapses_always
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** entails_preserved_by_always lifts p⇒q to always(p)⇒always(q), but combining with entails_trans should NOT allow promoting a one-shot entailment to an always-entailment without the spec itself being an always-property

