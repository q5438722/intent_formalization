# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_vrs_in_ongoing_reconciles_does_not_have_deletion_timestamp.rs`
**Date:** 2026-03-24T02:48:17Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_roundtrip_identity
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** no marshal_preserves_integrity axiom exists in this version — marshal_spec/unmarshal_spec are uninterpreted with zero roundtrip guarantees, so unmarshal∘marshal = id should be completely unprovable

### φ2: unmarshal_spec_always_succeeds
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** unmarshal_spec is uninterpreted with no axioms — totality should not be entailed since arbitrary Values may not represent valid specs

### φ3: allocate_triple_equals_plus_two
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** the third ensures claims the second allocation returns the same ID as the first — but allocate increments the counter so consecutive IDs must differ

### φ4: update_status_key_equals_get_then_delete_key
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** GetThenDeleteRequest::key returns self.key which has its own name field, while UpdateStatusRequest::key uses self.name — matching kind and namespace does NOT imply the name fields agree

### φ5: marshal_spec_unmarshal_status_cross
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec and unmarshal_status are independent uninterpreted functions over different semantic domains — a serialized spec Value should not be guaranteed to deserialize as a valid status

