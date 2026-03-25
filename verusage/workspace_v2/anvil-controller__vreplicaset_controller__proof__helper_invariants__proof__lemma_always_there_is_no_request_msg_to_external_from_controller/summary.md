# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_always_there_is_no_request_msg_to_external_from_controller.rs`
**Date:** 2026-03-24T02:43:02Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: double_marshal_fixpoint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** this assumes unmarshal_spec succeeds on marshal_spec output AND that re-marshalling produces the same Value — only unmarshal∘marshal = id is axiomatized (via external_body), not marshal∘unmarshal = id

### φ2: different_objects_different_marshalled_specs
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted — the spec guarantees left-inverse (unmarshal∘marshal = id which implies injectivity), but this is only assumed via external_body and not actually proven, so the SMT solver should not be able to derive injectivity

### φ3: allocate_counter_unbounded_gap
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** allocate increments by exactly 1, so the post-state counter should equal original + 1, not original + arbitrary n

### φ4: unmarshal_wrong_kind_succeeds
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** unmarshal explicitly rejects kind mismatches as its first check — valid spec and status should NOT override a wrong kind

### φ5: update_key_independent_of_obj
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** key() pulls kind from self.obj.kind — two requests with same namespace/name but different obj.kind yield different keys, so namespace+name alone should NOT determine key equality

