# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__impl_new_container__impl0__new_container_with_endpoint.rs`
**Date:** 2026-03-24T07:53:28Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `owned_endpoints` Ghost field has no cross-container disjointness constraint in any well-formedness predicate, allowing two distinct containers to claim the same endpoint

### φ2: endpoint_owning_container_not_in_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Endpoint has an `owning_container` field but there may be no well-formedness predicate ensuring it points to a valid container in the container domain

### φ3: get_endpoint_ptr_by_idx_some_implies_owned
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A thread's endpoint descriptor pointing to an endpoint should imply that endpoint's `owning_threads` set includes that (thread, index) pair — if this isn't entailed, the bidirectional ownership invariant is missing

### φ4: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `owned_threads` Ghost field on Container lacks any cross-container disjointness constraint, allowing two distinct containers to simultaneously claim ownership of the same thread

### φ5: get_proc_container_dom_without_full_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** get_proc's external_body ensures `self.wf() ==> container_dom.contains(ret.owning_container)` — when wf is false the implication is vacuously true, so the container_dom membership is NOT guaranteed, yet the SMT solver might derive it from other constraints leaking through the weaker preconditions

