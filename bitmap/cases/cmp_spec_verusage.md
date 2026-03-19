# Semantic Comparison: `generated_spec` vs `ground_truth` — Verusage Spec Generation (Claude Opus 4.5)

**Source file:** `/home/chentianyu/data/spec_gen_verusage/claude-opus-4.5/results.jsonl`

**Total cases:** 520

**Comparison methodology:** This comparison extracts the actual Verus code from generated specs (stripping natural language reasoning), locates the target function's `requires` and `ensures` clauses in both generated and ground truth, and compares them **semantically** — ignoring whitespace differences, trailing commas, and treating extensional equality (`=~=`) and structural equality (`==`) as equivalent.

---

## Case-by-Case Comparison

### Case 0 — `AC__vreplicaset_controller__proof__guarantee__guarantee_condition_holds`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.guarantee
- **Target function:** `guarantee_condition_holds`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 1 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_always_each_vrs_in_reconcile_implies_filtered_pods_owned_by_vrs`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_always_each_vrs_in_reconcile_implies_filtered_pods_owned_by_vrs`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 2 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_always_every_msg_from_vrs_controller_carries_vrs_key`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_always_every_msg_from_vrs_controller_carries_vrs_key`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 3 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_always_there_is_no_request_msg_to_external_from_controller`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_always_there_is_no_request_msg_to_external_from_controller`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 4 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_always_vrs_reconcile_request_only_interferes_with_itself`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_always_vrs_reconcile_request_only_interferes_with_itself`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 5 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_garbage_collector_does_not_delete_vrs_pods`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_garbage_collector_does_not_delete_vrs_pods`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 6 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_other_pending_request_interferes_with_vrs_reconcile`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_no_other_pending_request_interferes_with_vrs_reconcile`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 7 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_pending_interfering_update_request`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_no_pending_interfering_update_request`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 8 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_pending_mutation_request_not_from_controller_on_pods`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_no_pending_mutation_request_not_from_controller_on_pods`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 9 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_vrs_in_ongoing_reconciles_does_not_have_deletion_timestamp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_vrs_in_ongoing_reconciles_does_not_have_deletion_timestamp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 10 — `AC__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_vrs_in_schedule_does_not_have_deletion_timestamp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_invariants.proof
- **Target function:** `lemma_eventually_always_vrs_in_schedule_does_not_have_deletion_timestamp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 11 — `AC__vreplicaset_controller__proof__helper_lemmas__matching_pods_equal_to_matching_pod_entries_values`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_lemmas
- **Target function:** `matching_pods_equal_to_matching_pod_entries_values`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `matching_pods(vrs, s) =~= matching_pod_entries(vrs, s).values(),`

### Case 12 — `AC__vreplicaset_controller__proof__helper_lemmas__only_interferes_with_itself_equivalent_to_lifted_only_interferes_with_itself_action`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_lemmas
- **Target function:** `only_interferes_with_itself_equivalent_to_lifted_only_interferes_with_itself_action`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `spec.entails(always(lifted_vrs_reconcile_request_only_interferes_with_itself_action(controller_id)))
        <==>
        forall |vrs: VReplicaSetView`
- **Ground truth ensures:** `spec.entails(always(tla_forall(|vrs: VReplicaSetView| 
            lift_state(vrs_reconcile_request_only_interferes_with_itself(controller_id, vrs))))`

### Case 13 — `AC__vreplicaset_controller__proof__helper_lemmas__vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_lemmas
- **Target function:** `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(always(lift_state(|s: ClusterState|`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `spec.entails(always(lifted_vrs_rely_condition(cluster, controller_id))),`
- **Ground truth ensures:** `(forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vr`

### Case 14 — `AC__vreplicaset_controller__proof__helper_lemmas__vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition_action`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.helper_lemmas
- **Target function:** `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition_action`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(always(lifted_vrs_rely_condition_action(cluster, controller_id))),`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vrs`
- **Ground truth ensures:** `(forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vr`

### Case 15 — `AC__vreplicaset_controller__proof__liveness__api_actions__lemma_api_request_other_than_pending_req_msg_maintains_matching_pods`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.api_actions
- **Target function:** `lemma_api_request_other_than_pending_req_msg_maintains_matching_pods`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 16 — `AC__vreplicaset_controller__proof__liveness__api_actions__lemma_create_matching_pod_request_adds_matching_pod_and_returns_ok`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.api_actions
- **Target function:** `lemma_create_matching_pod_request_adds_matching_pod_and_returns_ok`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 17 — `AC__vreplicaset_controller__proof__liveness__api_actions__lemma_get_then_delete_matching_pod_request_deletes_matching_pod_and_returns_ok`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.api_actions
- **Target function:** `lemma_get_then_delete_matching_pod_request_deletes_matching_pod_and_returns_ok`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 18 — `AC__vreplicaset_controller__proof__liveness__api_actions__lemma_list_pods_request_returns_ok_list_resp_containing_matching_pods`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.api_actions
- **Target function:** `lemma_list_pods_request_returns_ok_list_resp_containing_matching_pods`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 19 — `AC__vreplicaset_controller__proof__liveness__proof__eventually_stable_reconciliation_holds`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.proof
- **Target function:** `eventually_stable_reconciliation_holds`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 20 — `AC__vreplicaset_controller__proof__liveness__proof__eventually_stable_reconciliation_holds_per_cr`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.proof
- **Target function:** `eventually_stable_reconciliation_holds_per_cr`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 21 — `AC__vreplicaset_controller__proof__liveness__proof__lemma_from_reconcile_idle_to_scheduled`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.proof
- **Target function:** `lemma_from_reconcile_idle_to_scheduled`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 22 — `AC__vreplicaset_controller__proof__liveness__proof__lemma_from_scheduled_to_init_step`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.proof
- **Target function:** `lemma_from_scheduled_to_init_step`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 23 — `AC__vreplicaset_controller__proof__liveness__proof__spec_before_phase_n_entails_true_leads_to_current_state_matches`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.proof
- **Target function:** `spec_before_phase_n_entails_true_leads_to_current_state_matches`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 24 — `AC__vreplicaset_controller__proof__liveness__proof_lemma_true_leads_to_always_current_state_matches`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness
- **Target function:** `proof_lemma_true_leads_to_always_current_state_matches`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `proof_lemma_true_leads_to_always_current_state_matches` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 25 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_create_pod_resp_to_receive_create_pod_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_create_pod_resp_to_receive_create_pod_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 26 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_delete_pod_resp_to_receive_delete_pod_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_delete_pod_resp_to_receive_delete_pod_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 27 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_done`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_list_pods_resp_to_done`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 28 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_receive_create_pod_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_list_pods_resp_to_receive_create_pod_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 29 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_receive_delete_pod_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_list_pods_resp_to_receive_delete_pod_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 30 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_send_create_pod_req`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_list_pods_resp_to_send_create_pod_req`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 31 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_send_delete_pod_req`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_list_pods_resp_to_send_delete_pod_req`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 32 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_ok_resp_at_after_create_pod_step_to_done`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_ok_resp_at_after_create_pod_step_to_done`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 33 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_ok_resp_at_after_delete_pod_step_to_done`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_ok_resp_at_after_delete_pod_step_to_done`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 34 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_ok_resp_to_send_create_pod_req`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_ok_resp_to_send_create_pod_req`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 35 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_ok_resp_to_send_delete_pod_req`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_receive_ok_resp_to_send_delete_pod_req`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 36 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_send_create_pod_req_to_receive_ok_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_send_create_pod_req_to_receive_ok_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 37 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_send_delete_pod_req_to_receive_ok_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_send_delete_pod_req_to_receive_ok_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 38 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_send_list_pods_req_to_receive_list_pods_resp`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_after_send_list_pods_req_to_receive_list_pods_resp`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 39 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_diff_and_init_to_current_state_matches`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_diff_and_init_to_current_state_matches`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 40 — `AC__vreplicaset_controller__proof__liveness__resource_match__lemma_from_init_step_to_send_list_pods_req`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.resource_match
- **Target function:** `lemma_from_init_step_to_send_list_pods_req`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 41 — `AC__vreplicaset_controller__proof__liveness__spec__assumption_and_invariants_of_all_phases_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `assumption_and_invariants_of_all_phases_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 42 — `AC__vreplicaset_controller__proof__liveness__spec__derived_invariants_since_beginning_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `derived_invariants_since_beginning_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 43 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 44 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_since_phase_i_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_since_phase_i_is_stable`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 45 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_since_phase_ii_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_since_phase_ii_is_stable`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 46 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_since_phase_iii_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_since_phase_iii_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 47 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_since_phase_iv_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_since_phase_iv_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 48 — `AC__vreplicaset_controller__proof__liveness__spec__invariant_since_phase_v_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `invariant_since_phase_v_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 49 — `AC__vreplicaset_controller__proof__liveness__spec__next_with_wf_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `next_with_wf_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 50 — `AC__vreplicaset_controller__proof__liveness__spec__spec_and_invariants_entails_stable_spec_and_invariants`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `spec_and_invariants_entails_stable_spec_and_invariants`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 51 — `AC__vreplicaset_controller__proof__liveness__spec__spec_entails_all_invariants`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `spec_entails_all_invariants`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 52 — `AC__vreplicaset_controller__proof__liveness__spec__spec_of_previous_phases_entails_eventually_new_invariants`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `spec_of_previous_phases_entails_eventually_new_invariants`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 53 — `AC__vreplicaset_controller__proof__liveness__spec__stable_spec_and_assumption_and_invariants_of_all_phases_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `stable_spec_and_assumption_and_invariants_of_all_phases_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 54 — `AC__vreplicaset_controller__proof__liveness__spec__stable_spec_is_stable`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.spec
- **Target function:** `stable_spec_is_stable`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 55 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_create_or_delete_pod_rank_zero_to_reconcile_idle`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `lemma_from_after_create_or_delete_pod_rank_zero_to_reconcile_idle`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 56 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_create_pod_rank_n_to_create_pod_rank_n_minus_1`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `lemma_from_after_create_pod_rank_n_to_create_pod_rank_n_minus_1`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 57 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_delete_pod_rank_n_to_delete_pod_rank_n_minus_1`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `lemma_from_after_delete_pod_rank_n_to_delete_pod_rank_n_minus_1`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 58 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_list_pods_to_reconcile_idle`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `lemma_from_after_list_pods_to_reconcile_idle`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 59 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_pending_req_in_flight_or_resp__in_flight_at_all_delete_to_delete`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate.lemma_from_pending_req_in_flight_or_resp
- **Target function:** `in_flight_at_all_delete_to_delete`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 60 — `AC__vreplicaset_controller__proof__liveness__terminate__lemma_from_pending_req_in_flight_or_resp_in_flight_at_all_create_to_create_n`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `lemma_from_pending_req_in_flight_or_resp_in_flight_at_all_create_to_create_n`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 61 — `AC__vreplicaset_controller__proof__liveness__terminate__reconcile_eventually_terminates`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `reconcile_eventually_terminates`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 62 — `AC__vreplicaset_controller__proof__liveness__terminate__reconcile_eventually_terminates_on_vrs_object`

- **Project:** Anvil-Advanced
- **Module:** vreplicaset_controller.proof.liveness.terminate
- **Target function:** `reconcile_eventually_terminates_on_vrs_object`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 63 — `AL__a_submap_of_a_finite_map_is_finite`

- **Project:** Anvil
- **Module:** 
- **Target function:** `a_submap_of_a_finite_map_is_finite`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `m1.submap_of(m2),
        m2.dom().finite(),`
- **Ensures:** `m1.dom().finite(),`

### Case 64 — `AL__a_to_temp_pred_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `a_to_temp_pred_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| #[trigger] p(a).entails(q(a)),
        forall |a: A| #[trigger] q(a).entails(p(a)),`
- **Ground truth requires:** `forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),`
- **Generated ensures:** `p == q,`
- **Ground truth ensures:** `p == q,`

### Case 65 — `AL__always_and_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_and_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `always(p).and(always(q)) == always(p.and(q)),`
- **Ground truth ensures:** `always(p.and(q)) == always(p).and(always(q)),`

### Case 66 — `AL__always_distributed_by_and`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_distributed_by_and`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `valid(always(p).and(always(q)).implies(always(p.and(q)))),`
- **Ground truth ensures:** `valid(always(p.and(q)).implies(always(p).and(always(q)))),`

### Case 67 — `AL__always_double`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_double`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `always(p).satisfied_by(ex),`
- **Ensures:** `always(always(p)).satisfied_by(ex),`

### Case 68 — `AL__always_double_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_double_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `always(always(p)) == always(p),`

### Case 69 — `AL__always_implies_forall_intro`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_implies_forall_intro`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| spec.entails(#[trigger] always(p.implies(a_to_q(a)))),`
- **Ground truth requires:** `forall |a: A| #[trigger] spec.entails(always(p.implies(a_to_q(a)))),`
- **Generated ensures:** `spec.entails(always(p.implies(tla_forall(a_to_q)))),`
- **Ground truth ensures:** `spec.entails(always(p.implies(tla_forall(a_to_q)))),`

### Case 70 — `AL__always_implies_preserved_by_always`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_implies_preserved_by_always`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(always(p.implies(q))),
        spec.entails(always(p)),`
- **Ground truth requires:** `spec.entails(always(p.implies(q))),`
- **Generated ensures:** `spec.entails(always(q)),`
- **Ground truth ensures:** `spec.entails(always(always(p).implies(always(q)))),`

### Case 71 — `AL__always_implies_to_leads_to`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_implies_to_leads_to`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(always(p.implies(q))),`
- **Ensures:** `spec.entails(p.leads_to(q)),`

### Case 72 — `AL__always_lift_action_unfold`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_lift_action_unfold`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `always(lift_action(p)).satisfied_by(ex),`
- **Ground truth requires:** `always(lift_action(p)).satisfied_by(ex),`
- **Generated ensures:** `forall |i: nat| p(#[trigger] ex.suffix(i).head(), ex.suffix(i).head_next()),`
- **Ground truth ensures:** `forall |i| p(#[trigger] ex.suffix(i).head(), ex.suffix(i).head_next()),`

### Case 73 — `AL__always_lift_state_unfold`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_lift_state_unfold`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `always(lift_state(p)).satisfied_by(ex),`
- **Ground truth requires:** `always(lift_state(p)).satisfied_by(ex),`
- **Generated ensures:** `forall |i: nat| p(#[trigger] ex.suffix(i).head()),`
- **Ground truth ensures:** `forall |i| p(#[trigger] ex.suffix(i).head()),`

### Case 74 — `AL__always_p_is_stable`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_p_is_stable`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `valid(stable(always(p))),`

### Case 75 — `AL__always_p_or_eventually_q`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_p_or_eventually_q`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `p.satisfied_by(ex),
        always(next).satisfied_by(ex),
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),`
- **Ground truth requires:** `always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),`
- **Generated ensures:** `always(p).satisfied_by(ex) || eventually(q).satisfied_by(ex),`
- **Ground truth ensures:** `always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),`

### Case 76 — `AL__always_p_or_eventually_q_rec`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_p_or_eventually_q_rec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |j: nat| j < i ==> p.satisfied_by(ex.suffix(j)),`
- **Ground truth requires:** `forall |idx| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) || q.satisfied_by(ex.suffix(id`
- **Generated ensures:** `(forall |j: nat| p.satisfied_by(ex.suffix(j))) || (exists |j: nat| q.satisfied_by(ex.suffix(j))),`
- **Ground truth ensures:** `p.satisfied_by(ex.suffix(i)),`

### Case 77 — `AL__always_propagate_forwards`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_propagate_forwards`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `always(p).satisfied_by(ex),`
- **Ensures:** `always(p).satisfied_by(ex.suffix(i)),`

### Case 78 — `AL__always_tla_forall_apply`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_tla_forall_apply`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(always(tla_forall(a_to_p))),`
- **Ensures:** `spec.entails(always(a_to_p(a))),`

### Case 79 — `AL__always_to_always_later`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_to_always_later`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(always(p)),`
- **Ensures:** `spec.entails(always(later(p))),`

### Case 80 — `AL__always_to_current`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_to_current`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `always(p).satisfied_by(ex),`
- **Ensures:** `p.satisfied_by(ex),`

### Case 81 — `AL__always_weaken`

- **Project:** Anvil
- **Module:** 
- **Target function:** `always_weaken`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(p)),
        p.entails(q),`
- **Ground truth requires:** `valid(p.implies(q)),
        spec.entails(always(p)),`
- **Generated ensures:** `spec.entails(always(q)),`
- **Ground truth ensures:** `spec.entails(always(q)),`

### Case 82 — `AL__commutativity_of_seq_map_and_filter`

- **Project:** Anvil
- **Module:** 
- **Target function:** `commutativity_of_seq_map_and_filter`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |a: A| #[trigger] pred(a) == pred_on_mapped(map(a)),`
- **Ground truth requires:** `forall |i: int| 0 <= i < s.len() ==> #[trigger] pred(s[i]) == #[trigger] pred_on_mapped(map(s[i])),`
- **Generated ensures:** `s.filter(pred).map_values(map) == s.map_values(map).filter(pred_on_mapped),`
- **Ground truth ensures:** `s.map_values(map).filter(pred_on_mapped) == s.filter(pred).map_values(map),`

### Case 83 — `AL__element_in_finite_set_exists_in_set_to_seq`

- **Project:** Anvil
- **Module:** 
- **Target function:** `element_in_finite_set_exists_in_set_to_seq`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.finite(),
        s.contains(e),`
- **Ensures:** `s.to_seq().contains(e),`

### Case 84 — `AL__element_in_seq_exists_in_original_finite_set`

- **Project:** Anvil
- **Module:** 
- **Target function:** `element_in_seq_exists_in_original_finite_set`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.finite(),
        s.to_seq().contains(e),`
- **Ensures:** `s.contains(e),`

### Case 85 — `AL__eliminate_always`

- **Project:** Anvil
- **Module:** 
- **Target function:** `eliminate_always`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `spec.entails(always(p)),`
- **Ground truth requires:** `spec.entails(always(p)),`
- **Generated ensures:** `valid(spec.implies(p)),`
- **Ground truth ensures:** `spec.entails(p),`

### Case 86 — `AL__empty_filter_implies_seq_pred_false_on_all_elements`

- **Project:** Anvil
- **Module:** 
- **Target function:** `empty_filter_implies_seq_pred_false_on_all_elements`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `s.filter(pred).len() == 0,`
- **Ground truth requires:** `s.filter(pred).len() == 0,`
- **Generated ensures:** `forall|i: int| 0 <= i < s.len() ==> !pred(s[i]),`
- **Ground truth ensures:** `forall |e: A| #![auto] s.contains(e) ==> !pred(e)`

### Case 87 — `AL__entails_and_different_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_and_different_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec1.entails(p),
        spec2.entails(q),`
- **Ensures:** `spec1.and(spec2).entails(p.and(q)),`

### Case 88 — `AL__entails_and_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_and_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(p),
        spec.entails(q),`
- **Ensures:** `spec.entails(p.and(q)),`

### Case 89 — `AL__entails_apply`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_apply`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `p.entails(q),
        p.satisfied_by(ex),`
- **Ensures:** `q.satisfied_by(ex),`

### Case 90 — `AL__entails_implies_leads_to`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_implies_leads_to`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `valid(p.implies(q)),`
- **Ground truth requires:** `p.entails(q),`
- **Generated ensures:** `spec.entails(p.leads_to(q)),`
- **Ground truth ensures:** `spec.entails(p.leads_to(q)),`

### Case 91 — `AL__entails_preserved_by_always`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_preserved_by_always`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 92 — `AL__entails_trans`

- **Project:** Anvil
- **Module:** 
- **Target function:** `entails_trans`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `p.entails(q),
        q.entails(r),`
- **Ensures:** `p.entails(r),`

### Case 93 — `AL__eventually_propagate_backwards`

- **Project:** Anvil
- **Module:** 
- **Target function:** `eventually_propagate_backwards`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `eventually(p).satisfied_by(ex.suffix(i)),`
- **Ensures:** `eventually(p).satisfied_by(ex),`

### Case 94 — `AL__execution_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `execution_equality`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),`
- **Ensures:** `ex1 == ex2,`

### Case 95 — `AL__filtered_size_is_one_means_only_one_such_value`

- **Project:** Anvil
- **Module:** 
- **Target function:** `filtered_size_is_one_means_only_one_such_value`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `(m.filter(f).len() == 1) ==> (exists |v: V| #[trigger] m.contains(v) && f(v) && m.filter(f).count(v) == 1),`
- **Ground truth ensures:** `(m.filter(f).len() == 1) ==`

### Case 96 — `AL__filtered_size_is_zero_means_no_such_value`

- **Project:** Anvil
- **Module:** 
- **Target function:** `filtered_size_is_zero_means_no_such_value`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `(m.filter(f).len() == 0) == (forall |v| f(v) ==> m.count(v) == 0),`
- **Ground truth ensures:** `(m.filter(f).len() == 0) == (forall |v: V| !(#[trigger] m.contains(v) && f(v)))`

### Case 97 — `AL__finite_set_to_seq_contains_all_set_elements`

- **Project:** Anvil
- **Module:** 
- **Target function:** `finite_set_to_seq_contains_all_set_elements`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `s.finite(),`
- **Ground truth requires:** `s.finite(),`
- **Generated ensures:** `forall|e: A| s.contains(e) ==> s.to_seq().contains(e),`
- **Ground truth ensures:** `forall |e: A| #[trigger] s.contains(e) <==> #[trigger] s.to_seq().contains(e)`

### Case 98 — `AL__implies_apply_with_always`

- **Project:** Anvil
- **Module:** 
- **Target function:** `implies_apply_with_always`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),`
- **Ensures:** `always(q).satisfied_by(ex),`

### Case 99 — `AL__init_invariant`

- **Project:** Anvil
- **Module:** 
- **Target function:** `init_invariant`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(lift_state(init)),
        spec.entails(always(lift_action(next))),
        forall |s: T| #[trigger] init(s) ==> inv(s),
        forall |`
- **Ground truth requires:** `forall |s: T| #[trigger] init(s) ==> inv(s),
        forall |s, s_prime: T| inv(s) && #[trigger] next(s, s_prime) ==> inv(s_prime),
        spec.entai`
- **Generated ensures:** `spec.entails(always(lift_state(inv))),`
- **Ground truth ensures:** `spec.entails(always(lift_state(inv))),`

### Case 100 — `AL__init_invariant_rec`

- **Project:** Anvil
- **Module:** 
- **Target function:** `init_invariant_rec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `init(ex.head()),
        forall |s: T| #[trigger] init(s) ==> inv(s),
        forall |s: T, s_next: T| inv(s) && #[trigger] next(s, s_next) ==> inv(s_`
- **Ground truth requires:** `init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigge`
- **Generated ensures:** `inv((ex.nat_to_state)(i)),`
- **Ground truth ensures:** `inv(ex.suffix(i).head()),`

### Case 101 — `AL__injective_finite_map_implies_dom_len_is_equal_to_values_len`

- **Project:** Anvil
- **Module:** 
- **Target function:** `injective_finite_map_implies_dom_len_is_equal_to_values_len`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `m.dom().finite(),
        m.is_injective(),`
- **Ensures:** `m.dom().len() == m.values().len(),`

### Case 102 — `AL__leads_to_always_combine`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_always_combine`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(p.leads_to(q)),
        spec.entails(p.leads_to(always(r))),`
- **Ground truth requires:** `spec.entails(p.leads_to(always(q))),
        spec.entails(p.leads_to(always(r))),`
- **Generated ensures:** `spec.entails(p.leads_to(q.and(always(r)))),`
- **Ground truth ensures:** `spec.entails(p.leads_to(always(q.and(r)))),
        spec.entails(p.leads_to(always(q).and(always(r)))),`

### Case 103 — `AL__leads_to_always_enhance`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_always_enhance`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(p.leads_to(q1)),
        spec.entails(always(inv)),
        spec.entails(always(q1.and(inv).implies(always(q2)))),`
- **Ground truth requires:** `spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        q1.and(inv).entails(q2),`
- **Generated ensures:** `spec.entails(p.leads_to(always(q2))),`
- **Ground truth ensures:** `spec.entails(p.leads_to(always(q2))),`

### Case 104 — `AL__leads_to_always_tla_forall`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_always_tla_forall`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| domain.contains(a) ==> spec.entails(p.leads_to(always(a_to_p(a)))),`
- **Ground truth requires:** `forall |a: A| spec.entails(p.leads_to(always(#[trigger] a_to_p(a)))),
        domain.finite(),
        domain.len() > 0,
        forall |a: A| #[trigg`
- **Generated ensures:** `spec.entails(p.leads_to(always(tla_forall(a_to_p)))),`
- **Ground truth ensures:** `spec.entails(p.leads_to(always(tla_forall(a_to_p)))),`

### Case 105 — `AL__leads_to_apply`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_apply`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(p.leads_to(q)),
        spec.entails(p),`
- **Ground truth requires:** `spec.entails(p),
        spec.entails(p.leads_to(q)),`
- **Generated ensures:** `spec.entails(eventually(q)),`
- **Ground truth ensures:** `spec.entails(eventually(q)),`

### Case 106 — `AL__leads_to_by_borrowing_inv`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_by_borrowing_inv`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(inv)),
        spec.and(always(inv)).entails(p.leads_to(q)),`
- **Ground truth requires:** `spec.entails(p.and(inv).leads_to(q)),
        spec.entails(always(inv)),`
- **Generated ensures:** `spec.entails(p.leads_to(q)),`
- **Ground truth ensures:** `spec.entails(p.leads_to(q)),`

### Case 107 — `AL__leads_to_exists_intro`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_exists_intro`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| spec.entails(#[trigger] a_to_p(a).leads_to(q)),`
- **Ground truth requires:** `forall |a: A| #[trigger] spec.entails(a_to_p(a).leads_to(q)),`
- **Generated ensures:** `spec.entails(tla_exists(a_to_p).leads_to(q)),`
- **Ground truth ensures:** `spec.entails(tla_exists(a_to_p).leads_to(q)),`

### Case 108 — `AL__leads_to_framed_by_or`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_framed_by_or`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(p.leads_to(q)),`
- **Ensures:** `spec.entails(p.or(r).leads_to(q.or(r))),`

### Case 109 — `AL__leads_to_rank_step_one`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_rank_step_one`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),`
- **Ensures:** `forall |n: nat| #[trigger] spec.entails(p(n).leads_to(p(0))),`

### Case 110 — `AL__leads_to_rank_step_one_help`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_rank_step_one_help`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |i: nat| #[trigger] spec.entails(p(i).leads_to(p(i + 1))) || i >= n,`
- **Ground truth requires:** `forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),`
- **Generated ensures:** `spec.entails(p(0).leads_to(p(n))),`
- **Ground truth ensures:** `spec.entails(p(n).leads_to(p(0))),`

### Case 111 — `AL__leads_to_rank_step_one_usize`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_rank_step_one_usize`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),`
- **Ensures:** `forall |n: usize| #[trigger] spec.entails(p(n).leads_to(p(0))),`

### Case 112 — `AL__leads_to_rank_step_one_usize_help`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_rank_step_one_usize_help`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |i: usize| #![trigger p(i)] 0 < i <= n ==> spec.entails(p(i).leads_to(p((i - 1) as usize))),`
- **Ground truth requires:** `forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),`
- **Generated ensures:** `spec.entails(p(n).leads_to(p(0))),`
- **Ground truth ensures:** `spec.entails(p(n).leads_to(p(0))),`

### Case 113 — `AL__leads_to_self_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_self_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `valid(p.leads_to(p)),`

### Case 114 — `AL__leads_to_shortcut_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_shortcut_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(p.leads_to(q.or(r))),
        spec.entails(r.leads_to(s)),`
- **Ground truth requires:** `spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),`
- **Generated ensures:** `spec.entails(p.leads_to(q.or(s))),`
- **Ground truth ensures:** `spec.entails(p.leads_to(r.or(s))),`

### Case 115 — `AL__leads_to_stable`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_stable`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(next)),
        spec.entails(p.leads_to(q)),
        spec.entails(always(q.implies(later(q)))),`
- **Ground truth requires:** `spec.entails(always(q.and(next).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(p.leads_to(q)),`
- **Generated ensures:** `spec.entails(p.leads_to(always(q))),`
- **Ground truth ensures:** `spec.entails(p.leads_to(always(q))),`

### Case 116 — `AL__leads_to_trans`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_trans`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),`
- **Ensures:** `spec.entails(p.leads_to(r)),`

### Case 117 — `AL__leads_to_weaken`

- **Project:** Anvil
- **Module:** 
- **Target function:** `leads_to_weaken`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),`
- **Ground truth requires:** `spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),`
- **Generated ensures:** `spec.entails(p2.leads_to(q2)),`
- **Ground truth ensures:** `spec.entails(p2.leads_to(q2)),`

### Case 118 — `AL__len_is_zero_means_count_for_each_value_is_zero`

- **Project:** Anvil
- **Module:** 
- **Target function:** `len_is_zero_means_count_for_each_value_is_zero`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `m.len() == 0,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `forall|v: V| m.count(v) == 0,`
- **Ground truth ensures:** `(forall |v| m.count(v) == 0) == (m.len() == 0),`

### Case 119 — `AL__map_values_to_set_eq_to_set_mk_map_values`

- **Project:** Anvil
- **Module:** 
- **Target function:** `map_values_to_set_eq_to_set_mk_map_values`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.map_values(map).to_set() == Set::new(|b: B| exists|a: A| s.to_set().contains(a) && map(a) == b),`
- **Ground truth ensures:** `s.map_values(map).to_set() == s.to_set().mk_map(map).values(),`

### Case 120 — `AL__map_values_weaknes_no_duplicates`

- **Project:** Anvil
- **Module:** 
- **Target function:** `map_values_weaknes_no_duplicates`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `map_values_weaknes_no_duplicates` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 121 — `AL__next_preserves_inv_rec`

- **Project:** Anvil
- **Module:** 
- **Target function:** `next_preserves_inv_rec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `inv.satisfied_by(ex),
        forall |idx: nat| next.satisfied_by(ex.suffix(idx)),
        forall |any_ex: Execution<T>| inv.satisfied_by(any_ex) && n`
- **Ground truth requires:** `inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(id`
- **Generated ensures:** `inv.satisfied_by(ex.suffix(i)),`
- **Ground truth ensures:** `inv.satisfied_by(ex.suffix(i)),`

### Case 122 — `AL__not_eventually_by_always_not`

- **Project:** Anvil
- **Module:** 
- **Target function:** `not_eventually_by_always_not`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `always(not(p)).satisfied_by(ex),`
- **Ensures:** `not(eventually(p)).satisfied_by(ex),`

### Case 123 — `AL__or_leads_to_combine`

- **Project:** Anvil
- **Module:** 
- **Target function:** `or_leads_to_combine`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(p.leads_to(r)),
        spec.entails(q.leads_to(r)),`
- **Ensures:** `spec.entails(p.or(q).leads_to(r)),`

### Case 124 — `AL__p_and_always_p_equals_always_p`

- **Project:** Anvil
- **Module:** 
- **Target function:** `p_and_always_p_equals_always_p`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `p.and(always(p)) == always(p),`

### Case 125 — `AL__p_leads_to_q_is_stable`

- **Project:** Anvil
- **Module:** 
- **Target function:** `p_leads_to_q_is_stable`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `valid(stable(p.leads_to(q))),`

### Case 126 — `AL__pack_conditions_to_spec`

- **Project:** Anvil
- **Module:** 
- **Target function:** `pack_conditions_to_spec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(always(c)),
        spec.and(always(c)).entails(p.leads_to(q)),`
- **Ground truth requires:** `spec.entails(p.and(c).leads_to(q)),`
- **Generated ensures:** `spec.entails(p.leads_to(q)),`
- **Ground truth ensures:** `spec.and(always(c)).entails(p.leads_to(q)),`

### Case 127 — `AL__push_filter_and_filter_push`

- **Project:** Anvil
- **Module:** 
- **Target function:** `push_filter_and_filter_push`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.push(e).filter(pred) == if pred(e)`
- **Ground truth ensures:** `pred(e) ==> s.push(e).filter(pred) == s.filter(pred).push(e),
        !pred(e) ==> s.push(e).filter(pred) == s.filter(pred),`

### Case 128 — `AL__push_to_set_seq_to_set_insert`

- **Project:** Anvil
- **Module:** 
- **Target function:** `push_to_set_seq_to_set_insert`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `push_to_set_seq_to_set_insert` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 129 — `AL__seq_equal_preserved_by_add`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_equal_preserved_by_add`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s1 =~= s2,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `s1 + suffix =~= s2 + suffix,`
- **Ground truth ensures:** `s1 == s2 <==> s1 + suffix == s2 + suffix`

### Case 130 — `AL__seq_equal_preserved_by_add_prefix`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_equal_preserved_by_add_prefix`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s1 == s2,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `prefix + s1 == prefix + s2,`
- **Ground truth ensures:** `s1 == s2 <==> prefix + s1 == prefix + s2`

### Case 131 — `AL__seq_filter_contains_implies_seq_contains`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_filter_contains_implies_seq_contains`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.filter(pred).contains(elt),`
- **Ensures:** `s.contains(elt),`

### Case 132 — `AL__seq_filter_is_a_subset_of_original_seq`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_filter_is_a_subset_of_original_seq`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),`
- **Ground truth ensures:** `forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger]`

### Case 133 — `AL__seq_filter_preserves_no_duplicates`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_filter_preserves_no_duplicates`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.no_duplicates(),`
- **Ensures:** `s.filter(pred).no_duplicates(),`

### Case 134 — `AL__seq_pred_false_on_all_elements_implies_empty_filter`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_pred_false_on_all_elements_implies_empty_filter`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall|i: int| 0 <= i < s.len() ==> !pred(s[i]),`
- **Ground truth requires:** `forall |e: A| #![auto] s.contains(e) ==> !pred(e),`
- **Generated ensures:** `s.filter(pred).len() == 0,`
- **Ground truth ensures:** `s.filter(pred).len() == 0,`

### Case 135 — `AL__seq_pred_false_on_all_elements_is_equivalent_to_empty_filter`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_pred_false_on_all_elements_is_equivalent_to_empty_filter`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.filter(pred).len() == 0 <==> forall |e: A| #![auto] s.contains(e) ==> !pred(e),`
- **Ground truth ensures:** `(forall |e: A| #[trigger] s.contains(e) ==> !pred(e)) <==> s.filter(pred).len() == 0,`

### Case 136 — `AL__seq_unequal_preserved_by_add`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_unequal_preserved_by_add`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s1 != s2,`
- **Ensures:** `s1 + suffix != s2 + suffix,`

### Case 137 — `AL__seq_unequal_preserved_by_add_auto`

- **Project:** Anvil
- **Module:** 
- **Target function:** `seq_unequal_preserved_by_add_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix,`
- **Ground truth ensures:** `forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix`

### Case 138 — `AL__simplify_predicate`

- **Project:** Anvil
- **Module:** 
- **Target function:** `simplify_predicate`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `p.entails(q),
        q.entails(p),`
- **Ground truth requires:** `p.entails(q),`
- **Generated ensures:** `p == q,`
- **Ground truth ensures:** `p == p.and(q),`

### Case 139 — `AL__spec_entails_always_tla_forall`

- **Project:** Anvil
- **Module:** 
- **Target function:** `spec_entails_always_tla_forall`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| spec.entails(#[trigger] always(a_to_p(a))),`
- **Ground truth requires:** `forall |a: A| spec.entails(always(#[trigger] a_to_p(a))),`
- **Generated ensures:** `spec.entails(always(tla_forall(a_to_p))),`
- **Ground truth ensures:** `spec.entails(always(tla_forall(a_to_p))),`

### Case 140 — `AL__spec_entails_tla_forall`

- **Project:** Anvil
- **Module:** 
- **Target function:** `spec_entails_tla_forall`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `forall |a: A| spec.entails(#[trigger] a_to_p(a)),`
- **Ensures:** `spec.entails(tla_forall(a_to_p)),`

### Case 141 — `AL__stable_and_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `stable_and_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `valid(stable(p)),
        valid(stable(q)),`
- **Ensures:** `valid(stable(p.and(q))),`

### Case 142 — `AL__strengthen_next`

- **Project:** Anvil
- **Module:** 
- **Target function:** `strengthen_next`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        forall |s, s_next| inv(s) && #[trigger] next(s, s_next`
- **Ground truth requires:** `spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        lift_action(next_and_inv).entails(lift_action(next).an`
- **Generated ensures:** `spec.entails(always(lift_action(next_and_inv))),`
- **Ground truth ensures:** `spec.entails(always(lift_action(next_and_inv))),`

### Case 143 — `AL__temp_pred_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `temp_pred_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `p.entails(q),
        q.entails(p),`
- **Ensures:** `p == q,`

### Case 144 — `AL__tla_exists_and_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_exists_and_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_exists(|a: A| a_to_p(a).and(q)) == tla_exists(a_to_p).and(q),`

### Case 145 — `AL__tla_exists_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_exists_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `tla_exists(|a: A| lift_state(|t: T| f(a, t))) == lift_state(|t: T| exists |a: A| #[trigger] f(a, t)),`
- **Ground truth ensures:** `lift_state(|t| exists |a| #[trigger] f(a, t)) == tla_exists(|a| lift_state(|t| f(a, t))),`

### Case 146 — `AL__tla_exists_implies_equality1`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_exists_implies_equality1`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_exists(|a: A| p.implies(a_to_q(a))) == p.implies(tla_exists(a_to_q)),`

### Case 147 — `AL__tla_exists_or_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_exists_or_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_exists(|a: A| a_to_p(a).or(q)) == tla_exists(a_to_p).or(q),`

### Case 148 — `AL__tla_forall_a_p_leads_to_q_a_is_stable`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_a_p_leads_to_q_a_is_stable`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |a: A| #[trigger] valid(p.leads_to(a_to_q(a))),
        forall |a: A| #[trigger] valid(stable(a_to_q(a))),`
- **Ground truth requires:** `forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a)))),`
- **Generated ensures:** `valid(p.leads_to(tla_forall(a_to_q))),`
- **Ground truth ensures:** `valid(stable(tla_forall(|a: A| p.leads_to(a_to_q(a))))),`

### Case 149 — `AL__tla_forall_always_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_always_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_forall(|a: A| always(a_to_p(a))) == always(tla_forall(a_to_p)),`

### Case 150 — `AL__tla_forall_always_equality_variant`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_always_equality_variant`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |a: A| #[trigger] a_to_always(a) == always(a_to_p(a)),`
- **Ground truth requires:** `forall |a: A| #![trigger a_to_always(a)] a_to_always(a).entails((|a: A| always(a_to_p(a)))(a)) && ((|a: A| always(a_to_p(a)))(a)).entails(a_to_always(`
- **Generated ensures:** `tla_forall(a_to_always) == always(tla_forall(a_to_p)),`
- **Ground truth ensures:** `tla_forall(a_to_always) == always(tla_forall(a_to_p)),`

### Case 151 — `AL__tla_forall_always_implies_equality2`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_always_implies_equality2`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_forall(|a: A| always(p.implies(a_to_q(a)))) == always(p.implies(tla_forall(a_to_q))),`

### Case 152 — `AL__tla_forall_and_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_and_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `tla_forall(a_to_p).and(q) == tla_forall(|a: A| a_to_p(a).and(q)),`
- **Ground truth ensures:** `tla_forall(|a: A| a_to_p(a).and(q)) == tla_forall(a_to_p).and(q),`

### Case 153 — `AL__tla_forall_implies_equality1`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_implies_equality1`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `tla_forall(|a: A| q.implies(a_to_p(a))) == q.implies(tla_forall(a_to_p)),`
- **Ground truth ensures:** `tla_forall(|a: A| a_to_p(a).implies(q)) == tla_exists(a_to_p).implies(q),`

### Case 154 — `AL__tla_forall_implies_equality2`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_implies_equality2`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_forall(|a: A| p.implies(a_to_q(a))) == p.implies(tla_forall(a_to_q)),`

### Case 155 — `AL__tla_forall_leads_to_equality1`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_leads_to_equality1`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_forall(|a: A| a_to_p(a).leads_to(q)) == tla_exists(a_to_p).leads_to(q),`

### Case 156 — `AL__tla_forall_not_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_not_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `not(tla_forall(a_to_p)) == tla_exists(|a: A| not(a_to_p(a))),`
- **Ground truth ensures:** `tla_forall(|a: A| not(a_to_p(a))) == not(tla_exists(a_to_p)),`

### Case 157 — `AL__tla_forall_or_equality`

- **Project:** Anvil
- **Module:** 
- **Target function:** `tla_forall_or_equality`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),`

### Case 158 — `AL__transform_leads_to_with_until`

- **Project:** Anvil
- **Module:** 
- **Target function:** `transform_leads_to_with_until`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 159 — `AL__true_pred_on_all_element_equal_to_pred_on_all_index`

- **Project:** Anvil
- **Module:** 
- **Target function:** `true_pred_on_all_element_equal_to_pred_on_all_index`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall |a: A| s.contains(a) ==> pred(a)
        <==>
        forall |i: int| 0 <= i < s.len() ==> pred(s[i]),`
- **Ground truth ensures:** `(forall |obj: A| #[trigger] s.contains(obj) ==> pred(obj)) <==> (forall |i: int| 0 <= i < s.len() ==> pred(s[i]))`

### Case 160 — `AL__true_pred_on_seq_implies_true_pred_on_filtered_seq`

- **Project:** Anvil
- **Module:** 
- **Target function:** `true_pred_on_seq_implies_true_pred_on_filtered_seq`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `forall |e: A| s.contains(e) ==> pred(e),`
- **Ensures:** `forall |e: A| s.filter(filter_pred).contains(e) ==> pred(e),`

### Case 161 — `AL__unpack_conditions_from_spec`

- **Project:** Anvil
- **Module:** 
- **Target function:** `unpack_conditions_from_spec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `spec.entails(stable(c)),
        spec.entails(p.leads_to(q)),`
- **Ground truth requires:** `valid(stable(spec)),
        spec.and(c).entails(p.leads_to(q)),`
- **Generated ensures:** `spec.entails(p.and(c).leads_to(q.and(c))),`
- **Ground truth ensures:** `spec.entails(p.and(c).leads_to(q)),`

### Case 162 — `AL__use_tla_forall`

- **Project:** Anvil
- **Module:** 
- **Target function:** `use_tla_forall`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `spec.entails(tla_forall(a_to_p)),`
- **Ensures:** `spec.entails(a_to_p(a)),`

### Case 163 — `AL__vacuous_leads_to`

- **Project:** Anvil
- **Module:** 
- **Target function:** `vacuous_leads_to`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(not(p))),`
- **Ground truth requires:** `spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),`
- **Generated ensures:** `spec.entails(p.leads_to(q)),`
- **Ground truth ensures:** `spec.entails(p.leads_to(q)),`

### Case 164 — `AL__vec_lib`

- **Project:** Anvil
- **Module:** 
- **Target function:** `vec_lib`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `vec_lib` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 165 — `AL__wf1`

- **Project:** Anvil
- **Module:** 
- **Target function:** `wf1`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(lift_state(p).and(lift_action(next)).implies(later(lift_state(p)).or(later(lift_state(q)))))),
        spec.entails(always(lift_st`
- **Ground truth requires:** `forall |s, s_prime: T| p(s) && #[trigger] next(s, s_prime) ==> p(s_prime) || q(s_prime),
        forall |s, s_prime: T| p(s) && #[trigger] next(s, s_p`
- **Generated ensures:** `spec.entails(lift_state(p).leads_to(lift_state(q))),`
- **Ground truth ensures:** `spec.entails(lift_state(p).leads_to(lift_state(q))),`

### Case 166 — `AL__wf1_variant_temp`

- **Project:** Anvil
- **Module:** 
- **Target function:** `wf1_variant_temp`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `spec.entails(always(next)),
        spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(forward).impli`
- **Ground truth requires:** `spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        sp`
- **Generated ensures:** `spec.entails(p.leads_to(q)),`
- **Ground truth ensures:** `spec.entails(p.leads_to(q)),`

### Case 167 — `IR__delegation_map_v__impl1_erase`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `impl1_erase`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `impl1_erase` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 168 — `IR__delegation_map_v__impl1_insert`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `impl1_insert`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `impl1_insert` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 169 — `IR__delegation_map_v__impl1_remove`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `impl1_remove`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `impl1_remove` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 170 — `IR__delegation_map_v__impl1_set`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `impl1_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `impl1_set` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 171 — `IR__delegation_map_v__impl1_to_set`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `impl1_to_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `impl1_to_set` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 172 — `IR__delegation_map_v__impl3__choose_gap_violator`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `choose_gap_violator`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `!self.gap(lo, hi),`
- **Ground truth requires:** `!self.gap(lo, hi),`
- **Generated ensures:** `lo.lt_spec(r),
            r.lt_spec(hi),
            self@.contains_key(*r.get()),`
- **Ground truth ensures:** `lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get()),`

### Case 173 — `IR__delegation_map_v__impl3__erase`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `erase`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `start <= end <= old(self)@.len(),`
- **Ground truth requires:** `old(self).valid(),
            start <= end <= old(self)@.len(),`
- **Generated ensures:** `self@ == old(self)@.subrange(0, start as int) + old(self)@.subrange(end as int, old(self)@.len() as int),`
- **Ground truth ensures:** `self.valid(),
            self@ == old(self)@.subrange(0, start as int) + old(self)@.subrange(end as int, old(self)@.len() as int),
            // TOD`

### Case 174 — `IR__delegation_map_v__impl3__find_key`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `find_key`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.valid(),`
- **Ensures:** `match o`

### Case 175 — `IR__delegation_map_v__impl3__greatest_lower_bound_index`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `greatest_lower_bound_index`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.valid(),
            self.keys@.len() > 0,`
- **Ground truth requires:** `self.valid(),
            self@.contains_key(K::zero_spec()),`
- **Generated ensures:** `index < self.keys@.len(),`
- **Ground truth ensures:** `0 <= index < self.keys@.len(),
            self.greatest_lower_bound_spec(*iter, KeyIterator::new_spec(self.keys@[index as int])),`

### Case 176 — `IR__delegation_map_v__impl3__keys_in_index_range_agree`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `keys_in_index_range_agree`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.valid(),
            0 <= lo <= hi < self.keys@.len(),`
- **Ground truth requires:** `self.valid(),
            0 <= lo <= hi < self.keys@.len(),`
- **Generated ensures:** `ret.0 == forall|i| #![auto] lo <= i <= hi ==> self.vals@[i]@ == v@,
            !ret.0 ==> (ret.1 == (self.vals@[hi as int]@ != v@ && forall|i|`
- **Ground truth ensures:** `ret.0 == forall |i| #![auto] lo <= i <= hi ==> self@[self.keys@[i]]@ == v@,
            !ret.0 ==> (ret.1 == (self@[self.keys@[hi as int]]@ != v@ && (`

### Case 177 — `IR__delegation_map_v__impl3__mind_the_gap`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `mind_the_gap`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|lo: KeyIterator<K>, hi: KeyIterator<K>|
                #![auto]
                self.gap(lo, hi) ==> self.gap(lo, hi),`
- **Ground truth ensures:** `forall|w, x, y, z| self.gap(w, x) && self.gap(y, z) && #[trigger] y.lt_spec(x) ==> #[trigger] self.gap(w, z),
            forall|w, x, y: KeyIterator<`

### Case 178 — `IR__delegation_map_v__impl3__new`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `new`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `cannot refer to private items`
- **Generated ensures:** `v.valid(),
            v@.len() == 0,`
- **Ground truth ensures:** `v@ == Seq::<K>::empty(),
                v.valid(),`

### Case 179 — `IR__delegation_map_v__impl3__set`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `old(self).valid(),`
- **Ground truth requires:** `old(self).valid(),`
- **Generated ensures:** `self.valid(),
            self@ == old(self)@.insert(k, v),`
- **Ground truth ensures:** `self.valid(),
            self@ == old(self)@.insert(k, v),
            forall |lo, hi| self.gap(lo, hi) <==>
                            old(self).ga`

### Case 180 — `IR__delegation_map_v__impl3__values_agree`

- **Project:** IronKV
- **Module:** delegation_map_v.impl3
- **Target function:** `values_agree`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.valid(),
            lo <= hi,
            hi < self.vals.len(),`
- **Ground truth requires:** `self.valid(),
            0 <= lo <= hi < self.keys@.len(),`
- **Generated ensures:** `ret.0 ==> forall|i: int| lo <= i <= hi ==> self.vals@[i]@ == v@,
            ret.0 ==> ret.1,
            !ret.0 && ret.1 ==> self.vals@[hi as int]@ !`
- **Ground truth ensures:** `ret.0 == forall |i| #![auto] lo <= i <= hi ==> self.vals@[i]@ == v@,
            !ret.0 ==> (ret.1 == (self.vals@[hi as int]@ != v@ && forall |i| #![a`

### Case 181 — `IR__delegation_map_v__impl4__all_keys_agree`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `all_keys_agree`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.valid(),
            0 <= lo <= hi,
            (hi + 1) < self.lows.keys@.len(),
            forall|i| #![auto] lo <= i <= hi ==> self.lows@[sel`
- **Ground truth requires:** `self.valid(),
            0 <= lo <= hi < self.lows.keys@.len(),
            forall |i| #![auto] lo <= i <= hi ==> self.lows@[self.lows.keys@[i]]@ ==`
- **Generated ensures:** `self.range_consistent(
                &KeyIterator::new_spec(self.lows.keys@[lo as int]),
                &KeyIterator::new_spec(self.lows.keys@[(hi`
- **Ground truth ensures:** `self.range_consistent(&KeyIterator::new_spec(self.lows.keys@[lo as int]), &KeyIterator::new_spec(self.lows.keys@[hi as int]), id),`

### Case 182 — `IR__delegation_map_v__impl4__almost_all_keys_agree`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `almost_all_keys_agree`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.valid(),
            self.lows.valid(),
            0 <= lo <= hi,
            hi <= self.lows.keys@.len(),
            lo < self.lows.keys@.len(`
- **Ground truth requires:** `self.valid(),
            0 <= lo <= hi < self.lows.keys@.len(),
            forall |i| #![auto] lo <= i < hi ==> self.lows@[self.lows.keys@[i]]@ == i`
- **Generated ensures:** `lo < hi ==> self.range_consistent(
                &KeyIterator::new_spec(self.lows.keys@[lo as int]),
                &if hi < self.lows.keys@.len()`
- **Ground truth ensures:** `self.range_consistent(&KeyIterator::new_spec(self.lows.keys@[lo as int]), &KeyIterator::new_spec(self.lows.keys@[hi as int]), id),`

### Case 183 — `IR__delegation_map_v__impl4__empty_key_range_is_consistent`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `empty_key_range_is_consistent`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `!lo.lt_spec(*hi),`
- **Ground truth requires:** `lo.geq_spec(*hi),`
- **Generated ensures:** `self.range_consistent(lo, hi, id),`
- **Ground truth ensures:** `self.range_consistent(lo, hi, id),`

### Case 184 — `IR__delegation_map_v__impl4__new`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `new`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `old(self).valid(),`
- **Ground truth requires:** `old(self).valid(),`
- **Generated ensures:** `s.valid(),
            s@ == Map::<K, ID>::empty(),`
- **Ground truth ensures:** `s.valid(),
            s@ == Map::<K,ID>::empty(),`

### Case 185 — `IR__delegation_map_v__impl4__range_consistent_impl`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `range_consistent_impl`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.valid(),`
- **Ensures:** `b == self.range_consistent(lo, hi, dst),`

### Case 186 — `IR__delegation_map_v__impl4__range_consistent_subset`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `range_consistent_subset`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `self.range_consistent(x, y, dst),
            !x_inner.lt_spec(*x),
            !(*y).lt_spec(*y_inner),`
- **Ground truth requires:** `self.range_consistent(x, y, dst),
            x_inner.geq_spec(*x),
            !y.lt_spec(*y_inner),`
- **Generated ensures:** `self.range_consistent(x_inner, y_inner, dst),`
- **Ground truth ensures:** `self.range_consistent(x_inner, y_inner, dst),`

### Case 187 — `IR__delegation_map_v__impl4__set`

- **Project:** IronKV
- **Module:** delegation_map_v.impl4
- **Target function:** `set`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `old(self).valid(),`
- **Ground truth requires:** `old(self).valid(),`
- **Generated ensures:** `self.valid(),
            self@.contains_key(k),
            self@[k] == v,
            forall|other_k: K| #![auto] other_k != k ==> (`
- **Ground truth ensures:** `self.valid(),
            self@ == old(self)@.insert(k, v),
            forall |lo, hi| self.gap(lo, hi) <==>
                            old(self).ga`

### Case 188 — `IR__delegation_map_v__impl5__delegate_for_key_range_is_host_impl`

- **Project:** IronKV
- **Module:** delegation_map_v.impl5
- **Target function:** `delegate_for_key_range_is_host_impl`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.valid(),`
- **Ground truth requires:** `self.valid(),`
- **Generated ensures:** `b == self.range_consistent(lo, hi, dst),`
- **Ground truth ensures:** `b == AbstractDelegationMap::delegate_for_key_range_is_host(AbstractDelegationMap(self@), KeyRange`

### Case 189 — `IR__delegation_map_v__vec_erase`

- **Project:** IronKV
- **Module:** delegation_map_v
- **Target function:** `vec_erase`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `start <= end,
        end <= old(v).len(),`
- **Ground truth requires:** `start <= end <= old(v).len(),`
- **Generated ensures:** `v.len() == old(v).len() - (end - start),
        forall|i: int| 0 <= i < start ==> v@[i] == old(v)@[i],
        forall|i: int| start <= i < v.len() ==`
- **Ground truth ensures:** `true,
        v@ == old(v)@.subrange(0, start as int) + old(v)@.subrange(end as int, old(v)@.len() as int),`

### Case 190 — `IR__host_impl_v__impl2__deliver_packet_seq`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `deliver_packet_seq`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `old(netc).ok(),
            outbound_packet_seq_is_valid(packets@),
            outbound_packet_seq_has_correct_srcs(packets@, old(netc).my_end_point(`
- **Ensures:** `netc.my_end_point() == old(netc).my_end_point(),
            (`

### Case 191 — `IR__host_impl_v__impl2__effect_of_delegation_map_set`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `effect_of_delegation_map_set`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `pre.valid(),
        post.valid(),`
- **Ground truth requires:** `pre.valid(),
            post.valid(),
            forall |ki:KeyIterator<CKey>| #[trigger] KeyIterator::between(*lo, ki, *hi) ==> post@[*ki.get()] ==`
- **Generated ensures:** `forall|k: CKey|
            (KeyRange::<CKey>`
- **Ground truth ensures:** `AbstractDelegationMap(post@) == AbstractDelegationMap(pre@).update(KeyRange::<AbstractKey>`

### Case 192 — `IR__host_impl_v__impl2__host_model_next_delegate`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_next_delegate`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 193 — `IR__host_impl_v__impl2__host_model_next_get_request`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_next_get_request`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `old(self).next_get_request_preconditions(),`
- **Ensures:** `self.next_get_request_postconditions(*old(self), sent_packets@),`

### Case 194 — `IR__host_impl_v__impl2__host_model_next_receive_message`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_next_receive_message`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `(`
- **Ensures:** `match old(self).received_packet`

### Case 195 — `IR__host_impl_v__impl2__host_model_next_set_request`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_next_set_request`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 196 — `IR__host_impl_v__impl2__host_model_next_shard`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_next_shard`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 197 — `IR__host_impl_v__impl2__host_model_receive_packet`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_model_receive_packet`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 198 — `IR__host_impl_v__impl2__host_noreceive_noclock_next`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `host_noreceive_noclock_next`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors"). Let me read the final file:`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 199 — `IR__host_impl_v__impl2__parse_end_points`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `parse_end_points`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `match out`

### Case 200 — `IR__host_impl_v__impl2__process_received_packet_next`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `process_received_packet_next`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `old(netc).ok(),
            outbound_packet_seq_is_valid(packets@),
            outbound_packet_seq_has_correct_srcs(packets@, old(netc).my_end_point(`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `netc.my_end_point() == old(netc).my_end_point(),
            (`
- **Ground truth ensures:** `(none)`

### Case 201 — `IR__host_impl_v__impl2__real_init_impl`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `real_init_impl`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — missing requires clause
- **Generated requires:** `(none)`
- **Ground truth requires:** `netc.valid(),`
- **Generated ensures:** `Self::init_ensures(netc, *args, rc),`
- **Ground truth ensures:** `Self::init_ensures(netc, *args, rc),`

### Case 202 — `IR__host_impl_v__impl2__real_next_impl`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `real_next_impl`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 203 — `IR__host_impl_v__impl2__receive_packet_next`

- **Project:** IronKV
- **Module:** host_impl_v.impl2
- **Target function:** `receive_packet_next`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors" which means the spec is syntactically correct. Let me get the complete file content:`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 204 — `IR__host_impl_v__make_empty_event_results`

- **Project:** IronKV
- **Module:** host_impl_v
- **Target function:** `make_empty_event_results`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `res@.recvs =~= Seq::<NetEvent>::empty(),
        res@.clocks =~= Seq::<NetEvent>::empty(),
        res@.sends =~= Seq::<NetEvent>::empty(),
        re`
- **Ground truth ensures:** `res@.recvs == Seq::<NetEvent>::empty(),
        res@.clocks == Seq::<NetEvent>::empty(),
        res@.sends == Seq::<NetEvent>::empty(),
        res@.`

### Case 205 — `IR__host_impl_v__make_send_only_event_results`

- **Project:** IronKV
- **Module:** host_impl_v
- **Target function:** `make_send_only_event_results`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall|i| 0 <= i < net_events@.len() ==> net_events@[i] is Send,`
- **Ground truth requires:** `forall |i: int| 0 <= i && i < net_events@.len() ==> net_events@[i] is Send`
- **Generated ensures:** `res@.recvs =~= Seq::<NetEvent>::empty(),
        res@.clocks =~= Seq::<NetEvent>::empty(),
        res@.sends =~= net_events@,
        res@.ios =~= ne`
- **Ground truth ensures:** `res@.recvs == Seq::<NetEvent>::empty(),
        res@.clocks == Seq::<NetEvent>::empty(),
        res@.sends == net_events@,
        res@.ios == net_ev`

### Case 206 — `IR__marshal_ironsht_specific_v__ckeyhashmap_max_serialized_size_exec`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v
- **Target function:** `ckeyhashmap_max_serialized_size_exec`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `r == ckeyhashmap_max_serialized_size(),`

### Case 207 — `IR__marshal_ironsht_specific_v__impl2__deserialize`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v.impl2
- **Target function:** `deserialize`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `res == spec_sorted_keys(*v),`
- **Ground truth ensures:** `match res`

### Case 208 — `IR__marshal_ironsht_specific_v__impl2__lemma_same_views_serialize_the_same`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v.impl2
- **Target function:** `lemma_same_views_serialize_the_same`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.view_equal(other),`
- **Ground truth requires:** `self.view_equal(other),`
- **Generated ensures:** `self.ghost_serialize() == other.ghost_serialize(),`
- **Ground truth ensures:** `self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),`

### Case 209 — `IR__marshal_ironsht_specific_v__impl2__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v.impl2
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),`

### Case 210 — `IR__marshal_ironsht_specific_v__impl2__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v.impl2
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 211 — `IR__marshal_ironsht_specific_v__lemma_is_marshalable_CKeyHashMap`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v
- **Target function:** `lemma_is_marshalable_CKeyHashMap`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `valid_hashtable(h@),`
- **Ensures:** `h.is_marshalable(),`

### Case 212 — `IR__marshal_ironsht_specific_v__sorted_keys`

- **Project:** IronKV
- **Module:** marshal_ironsht_specific_v
- **Target function:** `sorted_keys`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `res == spec_sorted_keys(*v),`

### Case 213 — `IR__marshal_v__impl0__deserialize`

- **Project:** IronKV
- **Module:** marshal_v.impl0
- **Target function:** `deserialize`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `res.is_some() ==> (`
- **Ground truth ensures:** `match res`

### Case 214 — `IR__marshal_v__impl0__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl0
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),`

### Case 215 — `IR__marshal_v__impl0__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl0
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 216 — `IR__marshal_v__impl0__serialize`

- **Project:** IronKV
- **Module:** marshal_v.impl0
- **Target function:** `serialize`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.is_marshalable(),`
- **Ground truth requires:** `self.is_marshalable()`
- **Generated ensures:** `data@ == old(data)@ + self.ghost_serialize(),`
- **Ground truth ensures:** `data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.le`

### Case 217 — `IR__marshal_v__impl0__serialized_size`

- **Project:** IronKV
- **Module:** marshal_v.impl0
- **Target function:** `serialized_size`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.is_marshalable(),`
- **Ground truth requires:** `self.is_marshalable(),`
- **Generated ensures:** `res == self.ghost_serialize().len(),`
- **Ground truth ensures:** `res as int == self.ghost_serialize().len()`

### Case 218 — `IR__marshal_v__impl1__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl1
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),`

### Case 219 — `IR__marshal_v__impl1__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl1
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 220 — `IR__marshal_v__impl2__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl2
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),`

### Case 221 — `IR__marshal_v__impl2__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl2
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 222 — `IR__marshal_v__impl2__serialize`

- **Project:** IronKV
- **Module:** marshal_v.impl2
- **Target function:** `serialize`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.is_marshalable(),`
- **Ground truth requires:** `self.is_marshalable()`
- **Generated ensures:** `data@ == old(data)@ + self.ghost_serialize(),`
- **Ground truth ensures:** `data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.le`

### Case 223 — `IR__marshal_v__impl3__lemma_same_views_serialize_the_same`

- **Project:** IronKV
- **Module:** marshal_v.impl3
- **Target function:** `lemma_same_views_serialize_the_same`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.view_equal(other),`
- **Ground truth requires:** `self.view_equal(other),`
- **Generated ensures:** `self.ghost_serialize() === other.ghost_serialize(),`
- **Ground truth ensures:** `self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),`

### Case 224 — `IR__marshal_v__impl3__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl3
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),`

### Case 225 — `IR__marshal_v__impl3__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl3
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 226 — `IR__marshal_v__impl3__lemma_view_equal_symmetric`

- **Project:** IronKV
- **Module:** marshal_v.impl3
- **Target function:** `lemma_view_equal_symmetric`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.view_equal(other)`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `other.view_equal(self)`
- **Ground truth ensures:** `self.view_equal(other) == other.view_equal(self)`

### Case 227 — `IR__marshal_v__impl3__serialize`

- **Project:** IronKV
- **Module:** marshal_v.impl3
- **Target function:** `serialize`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            old(data)@.len() + self.ghost_serialize().len() <= usize::MAX,`
- **Ground truth requires:** `self.is_marshalable()`
- **Generated ensures:** `data@ == old(data)@ + self.ghost_serialize(),`
- **Ground truth ensures:** `data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.le`

### Case 228 — `IR__marshal_v__impl4__deserialize`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `deserialize`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `res.is_some() ==> (`
- **Ground truth ensures:** `match res`

### Case 229 — `IR__marshal_v__impl4__is_marshalable`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `is_marshalable`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),`
- **Ensures:** `res == self.is_marshalable(),`

### Case 230 — `IR__marshal_v__impl4__lemma_same_views_serialize_the_same`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `lemma_same_views_serialize_the_same`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.view_equal(other),`
- **Ground truth requires:** `self.view_equal(other),`
- **Generated ensures:** `self.ghost_serialize() == other.ghost_serialize(),`
- **Ground truth ensures:** `self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize()`

### Case 231 — `IR__marshal_v__impl4__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)`

### Case 232 — `IR__marshal_v__impl4__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 233 — `IR__marshal_v__impl4__lemma_view_equal_symmetric`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `lemma_view_equal_symmetric`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.view_equal(other),`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `other.view_equal(self),`
- **Ground truth ensures:** `self.view_equal(other) == other.view_equal(self)`

### Case 234 — `IR__marshal_v__impl4__serialize`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `serialize`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.is_marshalable(),`
- **Ground truth requires:** `self.is_marshalable()`
- **Generated ensures:** `data@ == old(data)@ + self.ghost_serialize(),`
- **Ground truth ensures:** `data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.le`

### Case 235 — `IR__marshal_v__impl4__serialized_size`

- **Project:** IronKV
- **Module:** marshal_v.impl4
- **Target function:** `serialized_size`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.is_marshalable(),`
- **Ground truth requires:** `self.is_marshalable(),`
- **Generated ensures:** `res == self.ghost_serialize().len(),`
- **Ground truth ensures:** `res as int == self.ghost_serialize().len()`

### Case 236 — `IR__marshal_v__impl5__lemma_same_views_serialize_the_same`

- **Project:** IronKV
- **Module:** marshal_v.impl5
- **Target function:** `lemma_same_views_serialize_the_same`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.view_equal(other),`
- **Ground truth requires:** `self.view_equal(other),`
- **Generated ensures:** `self.ghost_serialize() === other.ghost_serialize(),`
- **Ground truth ensures:** `self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize()`

### Case 237 — `IR__marshal_v__impl5__lemma_serialization_is_not_a_prefix_of`

- **Project:** IronKV
- **Module:** marshal_v.impl5
- **Target function:** `lemma_serialization_is_not_a_prefix_of`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.is_marshalable(),
            other.is_marshalable(),
            !self.view_equal(other),`
- **Ground truth requires:** `!self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),`
- **Generated ensures:** `!self.ghost_serialize().is_prefix_of(other.ghost_serialize()),`
- **Ground truth ensures:** `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)`

### Case 238 — `IR__marshal_v__impl5__lemma_serialize_injective`

- **Project:** IronKV
- **Module:** marshal_v.impl5
- **Target function:** `lemma_serialize_injective`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),`
- **Ensures:** `self.view_equal(other),`

### Case 239 — `IR__marshal_v__impl5__lemma_view_equal_symmetric`

- **Project:** IronKV
- **Module:** marshal_v.impl5
- **Target function:** `lemma_view_equal_symmetric`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.view_equal(other),`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `other.view_equal(self),`
- **Ground truth ensures:** `self.view_equal(other) == other.view_equal(self)`

### Case 240 — `IR__net_sht_v__receive_with_demarshal`

- **Project:** IronKV
- **Module:** net_sht_v
- **Target function:** `receive_with_demarshal`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `old(netc).ok(),
        old(netc).my_end_point() == local_addr@,
        old(netc).state() is Receiving,
        local_addr.abstractable(),`
- **Ensures:** `(`

### Case 241 — `IR__net_sht_v__send_packet-poly`

- **Project:** IronKV
- **Module:** net_sht_v
- **Target function:** `send_packet-poly`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `send_packet-poly` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 242 — `IR__net_sht_v__send_packet_seq`

- **Project:** IronKV
- **Module:** net_sht_v
- **Target function:** `send_packet_seq`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `old(netc).ok(),
        outbound_packet_seq_is_valid(cpackets@),
        outbound_packet_seq_has_correct_srcs(cpackets@, old(netc).my_end_point()),`
- **Ensures:** `netc.my_end_point() == old(netc).my_end_point(),
        (`

### Case 243 — `IR__net_sht_v__sht_demarshall_data_method`

- **Project:** IronKV
- **Module:** net_sht_v
- **Target function:** `sht_demarshall_data_method`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `sht_demarshall_data_method` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 244 — `IR__net_sht_v__sht_marshal_data_injective`

- **Project:** IronKV
- **Module:** net_sht_v
- **Target function:** `sht_marshal_data_injective`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),`
- **Ground truth requires:** `a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),`
- **Generated ensures:** `a.view_equal(b),`
- **Ground truth ensures:** `a@ == b@,`

### Case 245 — `IR__seq_is_unique__do_vec_u8s_match`

- **Project:** IronKV
- **Module:** seq_is_unique
- **Target function:** `do_vec_u8s_match`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `eq == (e1@ == e2@),`

### Case 246 — `IR__seq_is_unique__endpoints_contain`

- **Project:** IronKV
- **Module:** seq_is_unique
- **Target function:** `endpoints_contain`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `present == (exists|i: int| 0 <= i < endpoints@.len() && endpoints@[i]@ == endpoint@),`
- **Ground truth ensures:** `present == abstractify_end_points(*endpoints).contains(endpoint@)`

### Case 247 — `IR__seq_is_unique__singleton_seq_to_set_is_singleton_set`

- **Project:** IronKV
- **Module:** seq_is_unique
- **Target function:** `singleton_seq_to_set_is_singleton_set`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `seq![x].to_set() =~= set![x],`

### Case 248 — `IR__seq_is_unique__test_unique`

- **Project:** IronKV
- **Module:** seq_is_unique
- **Target function:** `test_unique`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `unique == seq_is_unique(endpoints@.map(|i, end_point: EndPoint| end_point@)),`
- **Ground truth ensures:** `unique == seq_is_unique(abstractify_end_points(*endpoints)),`

### Case 249 — `IR__single_delivery_model_impl2__maybe_ack_packet_impl`

- **Project:** IronKV
- **Module:** single_delivery_model_impl2
- **Target function:** `maybe_ack_packet_impl`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.abstractable(),
            pkt.abstractable(),
            pkt.msg is Message,`
- **Ground truth requires:** `self.valid(),
        pkt.abstractable(),
        pkt.msg is Message,`
- **Generated ensures:** `SingleDelivery::maybe_ack_packet(
                self@,
                pkt@,
                opt_ack.unwrap_or(CPacket`
- **Ground truth ensures:** `SingleDelivery::maybe_ack_packet(self@, pkt@, opt_ack.unwrap()@, Self::option_cpacket_to_set_packet(opt_ack)),
        opt_ack is Some ==> valid_ack(o`

### Case 250 — `IR__single_delivery_model_impl2__receive_ack_impl`

- **Project:** IronKV
- **Module:** single_delivery_model_impl2
- **Target function:** `receive_ack_impl`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `old(self).valid(),
            pkt.abstractable(),
            pkt.msg is Ack,`
- **Ground truth requires:** `old(self).valid(),
       // self.abstractable(),
        pkt.abstractable(),
        pkt.msg is Ack,`
- **Generated ensures:** `self.valid(),
            SingleDelivery::receive_ack(old(self)@, self@, pkt@, Set::<Packet>::empty()),`
- **Ground truth ensures:** `self.valid(),
        SingleDelivery::receive_ack(old(self)@, self@, pkt@, set!`

### Case 251 — `IR__single_delivery_model_v__impl2__receive_impl`

- **Project:** IronKV
- **Module:** single_delivery_model_v.impl2
- **Target function:** `receive_impl`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `old(self).valid(),
            pkt.abstractable(),`
- **Ground truth requires:** `old(self).valid(),
        old(self).abstractable(),
        pkt.abstractable(),`
- **Generated ensures:** `self.valid(),
            SingleDelivery::receive(old(self)@, self@, pkt@, rr.get_ack()@, rr.get_abstracted_ack_set()),
            rr.valid_ack(*pkt)`
- **Ground truth ensures:** `self.valid(),
        rr.valid_ack(*pkt),
        SingleDelivery::receive(old(self)@, self@, pkt@, rr.get_ack()@, rr.get_abstracted_ack_set()),`

### Case 252 — `IR__single_delivery_model_v__impl2__retransmit_un_acked_packets`

- **Project:** IronKV
- **Module:** single_delivery_model_v.impl2
- **Target function:** `retransmit_un_acked_packets`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 253 — `IR__single_delivery_model_v__impl2__retransmit_un_acked_packets_for_dst`

- **Project:** IronKV
- **Module:** single_delivery_model_v.impl2
- **Target function:** `retransmit_un_acked_packets_for_dst`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.valid(),
            src.abstractable(),
            dst.abstractable(),
            outbound_packet_seq_is_valid(old(packets)@),
            out`
- **Ground truth requires:** `self.valid(),
        src.abstractable(),
        outbound_packet_seq_is_valid(old(packets)@),
        outbound_packet_seq_has_correct_srcs(old(packet`
- **Generated ensures:** `outbound_packet_seq_is_valid(packets@),
            outbound_packet_seq_has_correct_srcs(packets@, src@),
            Self::packets_are_valid_messages`
- **Ground truth ensures:** `packets@.map_values(|p: CPacket| p@).to_set() ==
            old(packets)@.map_values(|p: CPacket| p@).to_set() + self@.un_acked_messages_for_dest(src`

### Case 254 — `IR__single_delivery_model_v__impl2__send_single_cmessage`

- **Project:** IronKV
- **Module:** single_delivery_model_v.impl2
- **Target function:** `send_single_cmessage`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `old(self).valid(),
            dst.abstractable(),
            m.abstractable(),
            m.message_marshallable(),`
- **Ground truth requires:** `old(self).valid(),
            old(self).abstractable(),
            m.abstractable(),
            m.message_marshallable(),
            m.is_marshala`
- **Generated ensures:** `self.valid(),
            SingleDelivery::send_single_message(
                old(self)@,
                self@,
                m@,`
- **Ground truth ensures:** `self.valid(),
            match sm`

### Case 255 — `IR__single_delivery_model_v__same_view_same_marshalable`

- **Project:** IronKV
- **Module:** single_delivery_model_v
- **Target function:** `same_view_same_marshalable`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `x@ == y@,`
- **Ground truth requires:** `x@ == y@,`
- **Generated ensures:** `x.is_marshalable() == y.is_marshalable(),
        x.ghost_serialize() == y.ghost_serialize(),`
- **Ground truth ensures:** `x.is_marshalable() == y.is_marshalable(),`

### Case 256 — `IR__single_delivery_state_v__impl0__clone_up_to_view`

- **Project:** IronKV
- **Module:** single_delivery_state_v.impl0
- **Target function:** `clone_up_to_view`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `c@ == self@,`

### Case 257 — `IR__single_delivery_state_v__impl0__lemma_seqno_in_un_acked_list`

- **Project:** IronKV
- **Module:** single_delivery_state_v.impl0
- **Target function:** `lemma_seqno_in_un_acked_list`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.valid(dst),
            0 <= k < self.un_acked@.len(),`
- **Ground truth requires:** `self.valid(dst),
            0 <= k < self.un_acked@.len(),`
- **Generated ensures:** `self.un_acked@[k].arrow_Message_seqno() == self.num_packets_acked + 1 + k,`
- **Ground truth ensures:** `self.un_acked@[k].arrow_Message_seqno() == self.num_packets_acked + k + 1`

### Case 258 — `IR__single_delivery_state_v__impl0__truncate`

- **Project:** IronKV
- **Module:** single_delivery_state_v.impl0
- **Target function:** `truncate`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `old(self).valid(dst),
            old(self).num_packets_acked <= seqno_acked,
            seqno_acked as int <= AbstractParameters::static_params().ma`
- **Ground truth requires:** `old(self).valid(dst),
        old(self).num_packets_acked <= seqno_acked,`
- **Generated ensures:** `self.valid(dst),
            self.num_packets_acked == seqno_acked,`
- **Ground truth ensures:** `self.valid(dst),
        abstractify_cmessage_seq(self.un_acked@) == truncate_un_ack_list(abstractify_cmessage_seq(old(self).un_acked@), seqno_acked a`

### Case 259 — `IR__single_delivery_state_v__impl1__insert`

- **Project:** IronKV
- **Module:** single_delivery_state_v.impl1
- **Target function:** `insert`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `old(self).abstractable(),
        src@.valid_physical_address(),`
- **Generated ensures:** `self@ == old(self)@.insert(src@, last_seqno as nat),`
- **Ground truth ensures:** `self@ == old(self)@.insert(key@, value)`

### Case 260 — `IR__single_delivery_state_v__impl3__un_acked_messages_extend`

- **Project:** IronKV
- **Module:** single_delivery_state_v.impl3
- **Target function:** `un_acked_messages_extend`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self@.send_state.contains_key(dst),
            i <= self.send_state.epmap@[dst].un_acked@.len(),`
- **Ground truth requires:** `self@.send_state.contains_key(dst),
            i < self@.send_state[dst].un_acked.len(),
            self.send_state.valid()`
- **Generated ensures:** `self@.un_acked_messages_for_dest_up_to(src, dst, i as nat) <= self@.un_acked_messages_for_dest_up_to(src, dst, self.send_state.epmap@[dst].un_acked@.l`
- **Ground truth ensures:** `self@.un_acked_messages_for_dest_up_to(src, dst, i+1) ==
            self@.un_acked_messages_for_dest_up_to(src, dst, i).insert(
                Packe`

### Case 261 — `IR__verus_extra__choose_v`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `choose_v`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `choose_v` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 262 — `IR__verus_extra__lemma_filter_skip_rejected`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_filter_skip_rejected`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `0 <= i < s.len(),
        !pred(s[i]),`
- **Ground truth requires:** `0 <= i <= s.len(),
        forall |j| 0 <= j < i ==> !pred(s[j]),`
- **Generated ensures:** `s.skip(i).filter(pred) == s.skip(i + 1).filter(pred),`
- **Ground truth ensures:** `s.filter(pred) == s.skip(i).filter(pred)`

### Case 263 — `IR__verus_extra__lemma_flatten_set_seq_spec`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_flatten_set_seq_spec`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall |a: A| flatten_set_seq(sets).contains(a) <==> exists |i: int| 0 <= i < sets.len() && #[trigger] sets[i].contains(a),`
- **Ground truth ensures:** `(forall |x:A| #[trigger] flatten_set_seq(sets).contains(x) ==>
            exists |i: int| 0 <= i < sets.len() && #[trigger] sets[i].contains(x)),`

### Case 264 — `IR__verus_extra__lemma_fold_left_append_merge`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_fold_left_append_merge`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s1.add(s2).fold_left(Seq::<B>::empty(), |acc: Seq<B>, x: A| acc.add(f(x)))
            == s1.fold_left(Seq::<B>::empty(), |acc: Seq<B>, x: A| acc.add(`
- **Ground truth ensures:** `(s1 + s2).fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      ==
    s1.fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      +`

### Case 265 — `IR__verus_extra__lemma_fold_left_on_equiv_seqs`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_fold_left_on_equiv_seqs`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 266 — `IR__verus_extra__lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall|i: int| 0 <= i < s.len() ==> pred(s[i]),`
- **Ground truth requires:** `forall |i: int| 0 <= i && i < s.len() ==> pred(s[i])`
- **Generated ensures:** `s.filter(pred) =~= s,`
- **Ground truth ensures:** `s.filter(pred) == s`

### Case 267 — `IR__verus_extra__lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|i: int| 0 <= i < s.len() ==> !pred(s[i]),`
- **Ground truth requires:** `forall |i: int| 0 <= i && i < s.len() ==> !pred(s[i])`
- **Generated ensures:** `s.filter(pred).len() == 0,`
- **Ground truth ensures:** `s.filter(pred) =~= Seq::<A>::empty()`

### Case 268 — `IR__verus_extra__lemma_map_set_singleton_auto`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_map_set_singleton_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|k: A, v: B| #[trigger] Map::<A, B>::empty().insert(k, v).dom() == Set::<A>::empty().insert(k),`
- **Ground truth ensures:** `forall |x: A, f: spec_fn(A) -> B| #[trigger] set![x].map(f) == set![f(x)],`

### Case 269 — `IR__verus_extra__lemma_seq_fold_left_append_len_int`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_seq_fold_left_append_len_int`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.fold_left(prefix, |acc: Seq<B>, x: A| acc + f(x)).len() as int ==
            prefix.len() + s.map_values(|x: A| f(x).len() as int).fold_left(0int,`
- **Ground truth ensures:** `s.fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a)).len() as int
    ==
    s.fold_left(prefix.len() as int, |i: int, a: A| i + f(a).len() as int),`

### Case 270 — `IR__verus_extra__lemma_seq_fold_left_append_len_int_le`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_seq_fold_left_append_len_int_le`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `0 <= i <= s.len(),
        low >= 0,`
- **Ground truth requires:** `0 <= i <= s.len() as int,
    0 <= low,`
- **Generated ensures:** `low <= s.subrange(0, i).fold_left(low, |acc: int, x: A| acc + f(x).len()),`
- **Ground truth ensures:** `s.fold_left(low, |acc: int, x: A| acc + f(x).len()) >= 0,
    s.subrange(0, i).fold_left(low, |acc: int, x: A| acc + f(x).len()) <=
    s.fold_left(lo`

### Case 271 — `IR__verus_extra__lemma_seq_fold_left_sum_le`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_seq_fold_left_sum_le`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|i: int| 0 <= i < s.len() ==> f(s[i]) <= high,`
- **Ground truth requires:** `forall |i:int| 0 <= i < s.len() ==> f(s[i]) <= high,`
- **Generated ensures:** `s.fold_left(init, |acc: int, a: A| acc + f(a)) <= init + high * s.len(),`
- **Ground truth ensures:** `s.fold_left(init, |acc: int, x: A| acc + f(x)) <= init + s.len() * high,`

### Case 272 — `IR__verus_extra__lemma_seq_fold_left_sum_len_int_positive`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_seq_fold_left_sum_len_int_positive`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.fold_left(low as int, |acc: int, a: A| acc + f(a).len() as int) >= 0,`
- **Ground truth ensures:** `s.fold_left(low as int, |acc: int, x: A| acc + f(x).len()) >= 0,`

### Case 273 — `IR__verus_extra__lemma_seq_push_to_set`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_seq_push_to_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `s.push(x).to_set() =~= s.to_set().insert(x),`

### Case 274 — `IR__verus_extra__lemma_set_map_insert`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_set_map_insert`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `s.insert(x).map(f) =~= s.map(f).insert(f(x)),`

### Case 275 — `IR__verus_extra__lemma_to_set_distributes_over_addition`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_to_set_distributes_over_addition`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s.to_set() + t.to_set() =~= (s + t).to_set(),`
- **Ground truth ensures:** `(s+t).to_set() == s.to_set() + t.to_set()`

### Case 276 — `IR__verus_extra__lemma_to_set_singleton_auto`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_to_set_singleton_auto`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|a: A| seq![a].to_set() == set![a],`
- **Ground truth ensures:** `forall |x: A| #[trigger] seq![x].to_set() == set![x],`

### Case 277 — `IR__verus_extra__lemma_to_set_union_auto`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `lemma_to_set_union_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|s: Seq<A>, t: Seq<A>| #[trigger] (s + t).to_set() == s.to_set() + t.to_set(),`
- **Ground truth ensures:** `forall |s: Seq<A>, t: Seq<A>| #[trigger] (s+t).to_set() == s.to_set() + t.to_set()`

### Case 278 — `IR__verus_extra__map_finite`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `map_finite`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.finite(),`
- **Ensures:** `s.map(f).finite(),`

### Case 279 — `IR__verus_extra__map_fold_finite`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `map_fold_finite`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.finite(),`
- **Ensures:** `map_fold(s, f).finite(),`

### Case 280 — `IR__verus_extra__map_fold_ok`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `map_fold_ok`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `s.finite(),`
- **Ground truth requires:** `s.finite()`
- **Generated ensures:** `map_fold(s, f).finite(),`
- **Ground truth ensures:** `map_fold(s, f) =~= s.map(f)`

### Case 281 — `IR__verus_extra__map_set_finite_auto`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `map_set_finite_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|s: Set<A>, f: spec_fn(A) -> B| s.finite() ==> #[trigger] s.map(f).finite(),`
- **Ground truth ensures:** `forall |s: Set<A>, f: spec_fn(A) -> B| s.finite() ==> #[trigger] (s.map(f).finite()),`

### Case 282 — `IR__verus_extra__set_map_union`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `set_map_union`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `s1.union(s2).map(f) =~= s1.map(f).union(s2.map(f)),`
- **Ground truth ensures:** `(s1 + s2).map(f) == s1.map(f) + s2.map(f)`

### Case 283 — `IR__verus_extra__set_map_union_auto`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `set_map_union_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
            (s1 + s2).map(f) == s1.map(f) + s2.map(f),`
- **Ground truth ensures:** `forall |s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
        #[trigger] (s1 + s2).map(f) == s1.map(f) + s2.map(f)`

### Case 284 — `IR__verus_extra__some_differing_index_for_unequal_seqs`

- **Project:** IronKV
- **Module:** verus_extra
- **Target function:** `some_differing_index_for_unequal_seqs`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s1 != s2,`
- **Ground truth requires:** `s1 != s2,
    s1.len() == s2.len(),`
- **Generated ensures:** `0 <= i < s1.len() || 0 <= i < s2.len(),
        i < s1.len() && i < s2.len() ==> s1[i] !== s2[i],
        (i >= s1.len() || i >= s2.len()) ==> s1.len(`
- **Ground truth ensures:** `0 <= i < s1.len(),
    s1[i] != s2[i],`

### Case 285 — `MA__bin_sizes__bin_size_result`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `bin_size_result`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `size <= 131072,
        valid_bin_idx(smallest_bin_fitting_size(size as int)),`
- **Ensures:** `131072 >= size_of_bin(smallest_bin_fitting_size(size as int) as int) >= size,`

### Case 286 — `MA__bin_sizes__bin_size_result_mul8`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `bin_size_result_mul8`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `size % 8 == 0,`
- **Ground truth requires:** `size % 8 == 0,
        size <= 131072, //  == MEDIUM_OBJ_SIZE_MAX
        valid_bin_idx(smallest_bin_fitting_size(size as int)),`
- **Generated ensures:** `131072 >= size_of_bin(smallest_bin_fitting_size(size as int)) >= size,`
- **Ground truth ensures:** `131072 >= size_of_bin(smallest_bin_fitting_size(size as int) as int) >= size,`

### Case 287 — `MA__bin_sizes__bounds_for_smallest_bin_fitting_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `bounds_for_smallest_bin_fitting_size`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `1 <= size <= 8 * (524288 + 1),`
- **Ground truth requires:** `0 <= size <= 128 * 8,`
- **Generated ensures:** `valid_bin_idx(smallest_bin_fitting_size(size)),
        size_of_bin(smallest_bin_fitting_size(size)) >= size,`
- **Ground truth ensures:** `valid_bin_idx(smallest_bin_fitting_size(size)),
        size_of_bin(smallest_bin_fitting_size(size)) >= size,`

### Case 288 — `MA__bin_sizes__different_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `different_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `valid_bin_idx(bin_idx1),
        valid_bin_idx(bin_idx2),
        bin_idx1 != bin_idx2,`
- **Ensures:** `size_of_bin(bin_idx1) != size_of_bin(bin_idx2),`

### Case 289 — `MA__bin_sizes__div2`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `div2`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `y != 0,`
- **Ground truth requires:** `y > 0,`
- **Generated ensures:** `true,`
- **Ground truth ensures:** `x as int / (y * 2) == (x as int / y) / 2,`

### Case 290 — `MA__bin_sizes__idx_in_range_has_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `idx_in_range_has_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `valid_bin_idx(bin_idx),
        pfd_lower(bin_idx) <= wsize <= pfd_upper(bin_idx),`
- **Ground truth requires:** `valid_bin_idx(bin_idx),
        (pfd_lower(bin_idx) <= wsize <= pfd_upper(bin_idx)),
        wsize <= 128,`
- **Generated ensures:** `smallest_bin_fitting_size(wsize * INTPTR_SIZE) == bin_idx,`
- **Ground truth ensures:** `smallest_bin_fitting_size(wsize * INTPTR_SIZE) == bin_idx`

### Case 291 — `MA__bin_sizes__idx_out_of_range_has_different_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `idx_out_of_range_has_different_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `valid_bin_idx(bin_idx),
        !(pfd_lower(bin_idx) <= wsize <= pfd_upper(bin_idx)),
        0 <= wsize <= 128,`
- **Ground truth requires:** `valid_bin_idx(bin_idx),
        !(pfd_lower(bin_idx) <= wsize <= pfd_upper(bin_idx)),
        0 <= wsize <= 128,`
- **Generated ensures:** `smallest_bin_fitting_size(wsize * INTPTR_SIZE as int) != bin_idx,`
- **Ground truth ensures:** `smallest_bin_fitting_size(wsize * INTPTR_SIZE) != bin_idx`

### Case 292 — `MA__bin_sizes__leading_zeros_between`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `leading_zeros_between`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `lo <= mid,
        mid <= hi,`
- **Ground truth requires:** `lo <= mid < hi,`
- **Generated ensures:** `u64_leading_zeros(hi) <= u64_leading_zeros(mid),
        u64_leading_zeros(mid) <= u64_leading_zeros(lo),`
- **Ground truth ensures:** `u64_leading_zeros(lo) >= u64_leading_zeros(mid) >= u64_leading_zeros(hi),`

### Case 293 — `MA__bin_sizes__leading_zeros_between_powers_of_2`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `leading_zeros_between_powers_of_2`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `exp < 64,
        pow2(exp as int) <= i < pow2((exp + 1) as int),`
- **Ground truth requires:** `pow2(exp as int) <= i < pow2((exp + 1) as int),
        1 <= exp < 64`
- **Generated ensures:** `u64_leading_zeros(i) == 63 - exp,`
- **Ground truth ensures:** `u64_leading_zeros(i) == 64 - exp - 1,`

### Case 294 — `MA__bin_sizes__leading_zeros_monotonic`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `leading_zeros_monotonic`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `w > 0,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `u64_leading_zeros(w) < u64_leading_zeros(w / 2),`
- **Ground truth ensures:** `forall |x:u64| x < w ==> u64_leading_zeros(w) <= u64_leading_zeros(x),`

### Case 295 — `MA__bin_sizes__leading_zeros_powers_of_2`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `leading_zeros_powers_of_2`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `exp < 64,
        i == pow2(exp as int) as u64,`
- **Ground truth requires:** `i == pow2(exp as int),
        exp < 64`
- **Generated ensures:** `u64_leading_zeros(i) == 63 - exp,`
- **Ground truth ensures:** `u64_leading_zeros(i) == 64 - exp - 1,`

### Case 296 — `MA__bin_sizes__lemma_div_by_multiple`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `lemma_div_by_multiple`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `0 <= b,
        0 < d,`
- **Ensures:** `(b * d) / d == b,`

### Case 297 — `MA__bin_sizes__lemma_div_is_ordered`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `lemma_div_is_ordered`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `x <= y,
        z > 0,`
- **Ground truth requires:** `x <= y,
        0 < z,`
- **Generated ensures:** `x / z <= y / z,`
- **Ground truth ensures:** `x / z <= y / z`

### Case 298 — `MA__bin_sizes__log2`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `log2`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `i >= 1,`
- **Ground truth requires:** `i >= 1,`
- **Generated ensures:** `pow2(e as int) <= i,
        i < pow2((e + 1) as int),`
- **Ground truth ensures:** `pow2(e as int) <= i < pow2((e+1) as int),`

### Case 299 — `MA__bin_sizes__mul_assoc`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `mul_assoc`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `(x * y) * z == x * (y * z),`
- **Ground truth ensures:** `(x * y) * z == y * (x * z)`

### Case 300 — `MA__bin_sizes__mul_ordering`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `mul_ordering`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `x <= y,`
- **Ground truth requires:** `0 < x && 1 < y && 0 < z,
        x * y == z,`
- **Generated ensures:** `x * z <= y * z,`
- **Ground truth ensures:** `x < z,`

### Case 301 — `MA__bin_sizes__out_of_small_range`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `out_of_small_range`
- **Verification:** :x: Verification failed (`verification results:: 6 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `25 <= bin_idx <= BIN_HUGE,`
- **Ground truth requires:** `valid_bin_idx(bin_idx),
        size_of_bin(bin_idx) > SMALL_SIZE_MAX,`
- **Generated ensures:** `size_of_bin(bin_idx) > SMALL_SIZE_MAX,`
- **Ground truth ensures:** `pfd_lower(bin_idx) >= PAGES_DIRECT,`

### Case 302 — `MA__bin_sizes__pfd_lower_le_upper`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `pfd_lower_le_upper`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `valid_bin_idx(bin_idx),`
- **Ensures:** `pfd_lower(bin_idx) <= pfd_upper(bin_idx),`

### Case 303 — `MA__bin_sizes__pow2_adds`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `pow2_adds`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `pow2(e1 as int) * pow2(e2 as int) == pow2((e1 + e2) as int),`

### Case 304 — `MA__bin_sizes__pow2_positive`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `pow2_positive`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `pow2(e) > 0,`

### Case 305 — `MA__bin_sizes__pow2_properties`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `pow2_properties`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `pow2(0) == 1,`
- **Ground truth ensures:** `forall |e:int| pow2(e) > 0,
        forall |e:int| e > 0 ==> #[trigger] pow2(e) / 2 == pow2(e - 1),
        forall |e1, e2| 0 <= e1 < e2 ==> pow2(e1)`

### Case 306 — `MA__bin_sizes__pow2_subtracts`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `pow2_subtracts`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `e1 >= e2,`
- **Ground truth requires:** `e1 <= e2,`
- **Generated ensures:** `pow2((e1 - e2) as int) * pow2(e2 as int) == pow2(e1 as int),`
- **Ground truth ensures:** `pow2(e2 as int) / pow2(e1 as int) == pow2((e2 - e1) as int),`

### Case 307 — `MA__bin_sizes__result2_idx_in_range_has_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result2_idx_in_range_has_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `check2_idx_in_range_has_bin_size(bin_idx_start, bin_idx_end, wsize_start, wsize_end) ==>
            (forall |bin_idx, wsize| bin_idx_start <= bin_idx`
- **Ground truth ensures:** `check2_idx_in_range_has_bin_size(bin_idx_start, bin_idx_end, wsize_start, wsize_end) ==>
            (forall |bin_idx,wsize| bin_idx_start <= bin_idx`

### Case 308 — `MA__bin_sizes__result2_idx_out_of_range_has_different_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result2_idx_out_of_range_has_different_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `check2_idx_out_of_range_has_different_bin_size(bin_idx_start, bin_idx_end, wsize_start, wsize_end) ==>
            (forall |bin_idx, wsize| bin_idx_st`
- **Ground truth ensures:** `check2_idx_out_of_range_has_different_bin_size(bin_idx_start, bin_idx_end, wsize_start, wsize_end) ==>
            (forall |bin_idx,wsize| bin_idx_sta`

### Case 309 — `MA__bin_sizes__result_bin`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_bin`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `size_start <= size_end,`
- **Ground truth requires:** `size_start % 8 == 0,`
- **Generated ensures:** `check_bin(size_start, size_end),`
- **Ground truth ensures:** `check_bin(size_start, size_end) ==>
            (forall |size: int| size_start <= size < size_end && size % 8 == 0 ==>
                 #[trigger] id(`

### Case 310 — `MA__bin_sizes__result_bounds_for_smallest_bitting_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_bounds_for_smallest_bitting_size`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `size_start <= size_end,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `check_bounds_for_smallest_bitting_size(size_start, size_end),`
- **Ground truth ensures:** `check_bounds_for_smallest_bitting_size(size_start, size_end) ==>
            (forall |size| size_start <= size < size_end ==>
                 propert`

### Case 311 — `MA__bin_sizes__result_idx_in_range_has_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_idx_in_range_has_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 4 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `valid_bin_idx(bin_idx),
        wsize_start >= pfd_lower(bin_idx) as int,
        wsize_end <= pfd_upper(bin_idx) as int + 1,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `check_idx_in_range_has_bin_size(bin_idx, wsize_start, wsize_end),`
- **Ground truth ensures:** `check_idx_in_range_has_bin_size(bin_idx, wsize_start, wsize_end) ==>
            (forall |wsize| wsize_start <= wsize < wsize_end ==>`

### Case 312 — `MA__bin_sizes__result_idx_out_of_range_has_different_bin_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_idx_out_of_range_has_different_bin_size`
- **Verification:** :x: Verification failed (`verification results:: 4 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `valid_bin_idx(bin_idx),
        wsize_start <= wsize_end,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `check_idx_out_of_range_has_different_bin_size(bin_idx, wsize_start, wsize_end),`
- **Ground truth ensures:** `check_idx_out_of_range_has_different_bin_size(bin_idx, wsize_start, wsize_end) ==>
            (forall |wsize| wsize_start <= wsize < wsize_end ==>`

### Case 313 — `MA__bin_sizes__result_sbin`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_sbin`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `size_start <= size_end,
        check_sbin(size_start, size_end),`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `forall |i: int| size_start <= i < size_end ==> property_sbin(i),`
- **Ground truth ensures:** `check_sbin(size_start, size_end) ==>
            (forall |size| size_start <= size < size_end ==>
                 property_sbin(size)),`

### Case 314 — `MA__bin_sizes__result_sbin_bounds`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_sbin_bounds`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `size_start <= size_end,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `size_start >= size_end ==> check_sbin_bounds(size_start, size_end),`
- **Ground truth ensures:** `check_sbin_bounds(size_start, size_end) ==>
            (forall |size| size_start <= size < size_end ==>
                 property_sbin_bounds(size)),`

### Case 315 — `MA__bin_sizes__result_sbin_idx_smallest_sbin_fitting_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_sbin_idx_smallest_sbin_fitting_size`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `1 <= size_start <= size_end,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `check_sbin_idx_smallest_sbin_fitting_size(size_start, size_end),`
- **Ground truth ensures:** `check_sbin_idx_smallest_sbin_fitting_size(size_start, size_end) ==>
            (forall |size| size_start <= size < size_end ==>
                 prop`

### Case 316 — `MA__bin_sizes__result_smallest_bin_fitting_size_size_of_bin`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `result_smallest_bin_fitting_size_size_of_bin`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `1 <= bin_idx_start <= bin_idx_end <= BIN_HUGE as int + 1,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `check_smallest_bin_fitting_size_size_of_bin(bin_idx_start, bin_idx_end),`
- **Ground truth ensures:** `check_smallest_bin_fitting_size_size_of_bin(bin_idx_start, bin_idx_end) ==>
            (forall |bin_idx| bin_idx_start <= bin_idx < bin_idx_end ==>`

### Case 317 — `MA__bin_sizes__shift_is_div`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `shift_is_div`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `shift < 64,`
- **Ground truth requires:** `0 <= shift < 64,`
- **Generated ensures:** `(x >> shift) == x as nat / pow2(shift as int),`
- **Ground truth ensures:** `x >> shift == x as nat / pow2(shift as int),`

### Case 318 — `MA__bin_sizes__size_gt_8_implies_idx_gt_1`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `size_gt_8_implies_idx_gt_1`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `valid_bin_idx(bin_idx),
        size_of_bin(bin_idx) > 8,`
- **Ground truth requires:** `valid_bin_idx(bin_idx), size_of_bin(bin_idx) / 8 > 1,`
- **Generated ensures:** `bin_idx > 1,`
- **Ground truth ensures:** `bin_idx > 1,`

### Case 319 — `MA__bin_sizes__size_le_8_implies_idx_eq_1`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `size_le_8_implies_idx_eq_1`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `valid_bin_idx(bin_idx),
        size_of_bin(bin_idx) <= 8,`
- **Ground truth requires:** `valid_bin_idx(bin_idx), size_of_bin(bin_idx) / 8 <= 1,`
- **Generated ensures:** `bin_idx == 1,`
- **Ground truth ensures:** `bin_idx == 1,`

### Case 320 — `MA__bin_sizes__size_of_bin_bounds`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `size_of_bin_bounds`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `valid_bin_idx(b),`
- **Ground truth requires:** `valid_bin_idx(b),`
- **Generated ensures:** `size_of_bin(b) >= 8,
        size_of_bin(b) <= 8 * (524288 + 1),`
- **Ground truth ensures:** `size_of_bin(b) >= INTPTR_SIZE,`

### Case 321 — `MA__bin_sizes__size_of_bin_bounds_not_huge`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `size_of_bin_bounds_not_huge`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `1 <= b < BIN_HUGE,`
- **Ground truth requires:** `valid_bin_idx(b), b != BIN_HUGE,`
- **Generated ensures:** `size_of_bin(b) >= 8,
        size_of_bin(b) <= 4194304,`
- **Ground truth ensures:** `8 <= size_of_bin(b) <= 4194304`

### Case 322 — `MA__bin_sizes__size_of_bin_mult_word_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `size_of_bin_mult_word_size`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `valid_bin_idx(bin_idx),`
- **Ground truth requires:** `valid_bin_idx(bin_idx),`
- **Generated ensures:** `size_of_bin(bin_idx) as int % 8 == 0,`
- **Ground truth ensures:** `size_of_bin(bin_idx) % 8 == 0`

### Case 323 — `MA__bin_sizes__smallest_bin_fitting_size_size_of_bin`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `smallest_bin_fitting_size_size_of_bin`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `valid_bin_idx(bin_idx),`
- **Ensures:** `smallest_bin_fitting_size(size_of_bin(bin_idx) as int) == bin_idx,`

### Case 324 — `MA__bin_sizes__valid_sbin_idx_smallest_sbin_fitting_size`

- **Project:** Memory-Allocator
- **Module:** bin_sizes
- **Target function:** `valid_sbin_idx_smallest_sbin_fitting_size`
- **Verification:** :x: Verification failed (`verification results:: 8 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `1 <= i <= SLICES_PER_SEGMENT as int,`
- **Ground truth requires:** `0 <= i <= SLICES_PER_SEGMENT`
- **Generated ensures:** `valid_sbin_idx(smallest_sbin_fitting_size(i)),`
- **Ground truth ensures:** `valid_sbin_idx(smallest_sbin_fitting_size(i)),`

### Case 325 — `MA__commit_mask__impl__all_set`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `all_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `res == other@.subset_of(self@),`

### Case 326 — `MA__commit_mask__impl__any_set`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `any_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `res <==> !self@.disjoint(other@),`
- **Ground truth ensures:** `res == !self@.disjoint(other@)`

### Case 327 — `MA__commit_mask__impl__clear`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `clear`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `self@ == old(self)@.difference(other@),`

### Case 328 — `MA__commit_mask__impl__create`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `create`
- **Verification:** :x: Verification failed (`verification results:: 8 verified, 2 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `idx + count <= COMMIT_MASK_BITS,`
- **Ground truth requires:** `idx + count <= COMMIT_MASK_BITS,
            old(self)@ == Set::<int>::empty(),`
- **Generated ensures:** `self@ == Set::new(|i: int| idx <= i < idx + count),`
- **Ground truth ensures:** `self@ == Set::new(|i: int| idx <= i < idx + count),`

### Case 329 — `MA__commit_mask__impl__create_empty`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `create_empty`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `self@ == Set::<int>::empty(),`

### Case 330 — `MA__commit_mask__impl__create_full`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `create_full`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|i: int| 0 <= i < 512 ==> self@.contains(i),`
- **Ground truth ensures:** `self@ == Set::new(|i: int| 0 <= i < COMMIT_MASK_BITS),`

### Case 331 — `MA__commit_mask__impl__create_intersect`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `create_intersect`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `res@ == self@.intersect(other@),`

### Case 332 — `MA__commit_mask__impl__empty`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `empty`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `cm@ == Set::<int>::empty(),`

### Case 333 — `MA__commit_mask__impl__is_empty`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `is_empty`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 2 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `b == (self@ =~= Set::<int>::empty()),`

### Case 334 — `MA__commit_mask__impl__is_full`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `is_full`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `i < 8 && j < 64;
                }

                assert(COMMIT_MASK_BITS == 512)`
- **Generated ensures:** `b ==> (forall|j: int| 0 <= j < 512 ==> self@.contains(j)),`
- **Ground truth ensures:** `b == (self@ == Set::new(|i: int| 0 <= i < COMMIT_MASK_BITS))`

### Case 335 — `MA__commit_mask__impl__lemma_change_one_entry`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `lemma_change_one_entry`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `0 <= i < 8,
            forall|j: int| 0 <= j < 8 && j != i ==> self.mask[j] == other.mask[j],`
- **Ground truth requires:** `0 <= i < 8,
            self.mask[i] == 0,
            forall|j: int| 0 <= j < i ==> other.mask[j] == self.mask[j],
            forall|j: int| i < j <`
- **Generated ensures:** `forall|t: int| (t < i * 64 || t >= (i + 1) * 64) && 0 <= t < 512 ==> (self@.contains(t) <==> other@.contains(t)),`
- **Ground truth ensures:** `other@ == self@.union(Set::new(|b: usize| b < 64 && is_bit_set(other.mask[i], b)).map(|b: usize| 64 * i + b)),`

### Case 336 — `MA__commit_mask__impl__lemma_view`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `lemma_view`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|i: int| self@.contains(i) ==> 0 <= i < 512,`
- **Ground truth ensures:** `// forall|i: int| self@.contains(i) ==> i < 512,
        // TODO: this isn't currently used but probably will need it (-> check later)
        (forall`

### Case 337 — `MA__commit_mask__impl__next_run`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `next_run`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 5 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `idx <= COMMIT_MASK_BITS,`
- **Ground truth requires:** `0 <= idx < COMMIT_MASK_BITS,`
- **Generated ensures:** `res.0 >= idx,
            res.0 <= COMMIT_MASK_BITS,
            res.0 + res.1 <= COMMIT_MASK_BITS,
        // This should be true, but isn't strictly`
- **Ground truth ensures:** `(`

### Case 338 — `MA__commit_mask__impl__set`

- **Project:** Memory-Allocator
- **Module:** commit_mask.impl
- **Target function:** `set`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `self@ == old(self)@.union(other@),`

### Case 339 — `MA__commit_mask__lemma_bitmask_to_is_bit_set`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_bitmask_to_is_bit_set`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `o < 64,`
- **Ground truth requires:** `n < 64,
        o <= 64 - n,`
- **Generated ensures:** `is_bit_set(n, o) == ((n & (1usize << o)) == (1usize << o)),`
- **Ground truth ensures:** `(`

### Case 340 — `MA__commit_mask__lemma_is_bit_set`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_is_bit_set`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|a: usize, b: usize| b < 64 ==> (is_bit_set(a, b) <==> (a & (1usize << b)) == (1usize << b)),`
- **Ground truth ensures:** `forall|j: usize| j < 64 ==> !(#[trigger] is_bit_set(0, j)),
        forall|j: usize| is_bit_set(!0usize, j),
        forall|a: usize, b: usize, j: usi`

### Case 341 — `MA__commit_mask__lemma_map_distribute`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_map_distribute`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `s1.union(s2).map(f) == s1.map(f).union(s2.map(f)),`

### Case 342 — `MA__commit_mask__lemma_map_distribute_auto`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_map_distribute_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T|
            #[trigger] s1.union(s2).map(f) == s1.map(f).union(s2.map(f)),`
- **Ground truth ensures:** `forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T| s1.union(s2).map(f) == #[trigger] s1.map(f).union(s2.map(f))`

### Case 343 — `MA__commit_mask__lemma_obtain_bit_index_1`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_obtain_bit_index_1`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `a != 0,`
- **Ensures:** `b < 64,
        is_bit_set(a, b),`

### Case 344 — `MA__commit_mask__lemma_obtain_bit_index_1_aux`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_obtain_bit_index_1_aux`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `hi <= 64,
        a != 0,
        forall|j: u64| hi <= j < 64 ==> !#[trigger] is_bit_set(a as usize, j as usize),`
- **Ground truth requires:** `a != 0,
        hi <= 64,
        a >> hi == 0,`
- **Generated ensures:** `i < hi,
        is_bit_set(a as usize, i as usize),`
- **Ground truth ensures:** `i < hi,
        is_bit_set!(a, i),`

### Case 345 — `MA__commit_mask__lemma_obtain_bit_index_2`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_obtain_bit_index_2`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `a != 0,`
- **Ground truth requires:** `a != !0usize`
- **Generated ensures:** `b < 64,
        is_bit_set(a, b),`
- **Ground truth ensures:** `b < 64,
        !is_bit_set(a, b)`

### Case 346 — `MA__commit_mask__lemma_obtain_bit_index_3`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_obtain_bit_index_3`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `a & b != b,`
- **Ensures:** `i < 64,
        !is_bit_set(a, i),
        is_bit_set(b, i),`

### Case 347 — `MA__commit_mask__lemma_obtain_bit_index_3_aux`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `lemma_obtain_bit_index_3_aux`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `hi <= 64,
        a != b,
        a < (1u64 << hi),
        b < (1u64 << hi),`
- **Ground truth requires:** `a & b != b,
        hi <= 64,
        a >> hi == 0,
        b >> hi == 0,`
- **Generated ensures:** `i < hi,
        ((a >> i) & 1) != ((b >> i) & 1),`
- **Ground truth ensures:** `i < hi,
        !is_bit_set!(a, i),
        is_bit_set!(b, i),`

### Case 348 — `MA__commit_mask__set_int_range_commit_size`

- **Project:** Memory-Allocator
- **Module:** commit_mask
- **Target function:** `set_int_range_commit_size`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `mask@.contains(0)`
- **Generated ensures:** `forall|addr: int| mask.bytes(sid).contains(addr) ==>
            mask@.contains((addr - segment_start(sid)) / COMMIT_SIZE as int),`
- **Ground truth ensures:** `set_int_range(segment_start(sid), segment_start(sid) + COMMIT_SIZE) <= mask.bytes(sid)`

### Case 349 — `MA__config`

- **Project:** Memory-Allocator
- **Module:** 
- **Target function:** `config`
- **Verification:** :x: Verification failed (`verification results:: 34 verified, 1 errors`)
- **Status:** :warning: Target function not found in generated code
- **Note:** Code was extracted but the target function `config` was not found in it. The model may have generated specs for a different function or renamed it.

### Case 350 — `MA__layout__bitand_with_mask_gives_rounding`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `bitand_with_mask_gives_rounding`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `y > 0,`
- **Ground truth requires:** `y != 0, y & sub(y, 1) == 0,`
- **Generated ensures:** `(x as int) / (y as int) == (x as int) / (y as int),`
- **Ground truth ensures:** `x & !sub(y, 1) == (x / y) * y,`

### Case 351 — `MA__layout__block_ptr_aligned_to_word`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `block_ptr_aligned_to_word`
- **Verification:** :x: Verification failed (`verification results:: 25 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `is_block_ptr(ptr, block_id),`
- **Ground truth requires:** `b % c == 0, c != 0`
- **Generated ensures:** `ptr as int % INTPTR_SIZE as int == 0,`
- **Ground truth ensures:** `forall |p, block_id| is_block_ptr(p, block_id) ==>
        p as int % align_of::<Node>() as int == 0`

### Case 352 — `MA__layout__block_size_ge_word`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `block_size_ge_word`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `is_block_ptr1(ptr, block_id),`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `block_id.block_size >= size_of::<Node>(),`
- **Ground truth ensures:** `forall |p, block_id| is_block_ptr(p, block_id) ==>
        block_id.block_size >= size_of::<Node>()`

### Case 353 — `MA__layout__block_start_at_diff`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `block_start_at_diff`
- **Verification:** :x: Verification failed (`verification results:: 8 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `block_idx1 <= block_idx2,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `block_start_at(page_id, block_size as int, block_idx2 as int) -
        block_start_at(page_id, block_size as int, block_idx1 as int) ==
        (bloc`
- **Ground truth ensures:** `block_start_at(page_id, block_size as int, block_idx2 as int) ==
        block_start_at(page_id, block_size as int, block_idx1 as int) + (block_idx2 -`

### Case 354 — `MA__layout__get_block_start_from_is_block_ptr`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `get_block_start_from_is_block_ptr`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `is_block_ptr(ptr, block_id),`
- **Ensures:** `ptr as int == block_start(block_id),`

### Case 355 — `MA__layout__impl__align_down`

- **Project:** Memory-Allocator
- **Module:** layout.impl
- **Target function:** `align_down`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `y != 0,
        (x / y) * y <= usize::MAX,`
- **Ground truth requires:** `y != 0,`
- **Generated ensures:** `res == (x / y) * y,`
- **Ground truth ensures:** `res == (x as int / y as int) * y,
        res <= x < res + y,
        res % y == 0,
        (res / y * y) == res,`

### Case 356 — `MA__layout__impl__align_up`

- **Project:** Memory-Allocator
- **Module:** layout.impl
- **Target function:** `align_up`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `y != 0,
        x + y - 1 <= usize::MAX,`
- **Ground truth requires:** `y != 0,
        x + y - 1 <= usize::MAX,`
- **Generated ensures:** `res >= x,
        (res as int) % (y as int) == 0,
        res < x + y,`
- **Ground truth ensures:** `res == ((x + y - 1) / y as int) * y,
        x <= res <= x + y - 1,
        res % y == 0,
        (res / y * y) == res,`

### Case 357 — `MA__layout__impl__calculate_page_block_at`

- **Project:** Memory-Allocator
- **Module:** layout.impl
- **Target function:** `calculate_page_block_at`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 358 — `MA__layout__is_block_ptr_mult4`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `is_block_ptr_mult4`
- **Verification:** :x: Verification failed (`verification results:: 25 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `is_block_ptr(ptr, block_id),`
- **Ensures:** `ptr as int % 4 == 0,`

### Case 359 — `MA__layout__mod_mul-poly`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `mod_mul-poly`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`.`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 360 — `MA__layout__mod_trans`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `mod_trans`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 361 — `MA__layout__mul_mod_right`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `mul_mod_right`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 362 — `MA__layout__segment_start_mult8`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `segment_start_mult8`
- **Verification:** :x: Verification failed (`verification results:: 7 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `segment_start(segment_id) % 8 == 0,`

### Case 363 — `MA__layout__segment_start_mult_commit_size`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `segment_start_mult_commit_size`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `segment_start(segment_id) % (COMMIT_SIZE as int) == 0,`
- **Ground truth ensures:** `segment_start(segment_id) % COMMIT_SIZE as int == 0,`

### Case 364 — `MA__layout__sub_distribute`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `sub_distribute`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `a * (b - c) == a * b - a * c,`
- **Ground truth ensures:** `a * c - b * c == (a - b) * c,`

### Case 365 — `MA__layout__two_mul_with_bit0`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `two_mul_with_bit0`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `y1 == 2 * x1,`
- **Ground truth requires:** `y1 != 0,`
- **Generated ensures:** `y1 % 2 == 0,`
- **Ground truth ensures:** `(2 * x1) / (2 * y1) == x1 / y1`

### Case 366 — `MA__layout__two_mul_with_bit1`

- **Project:** Memory-Allocator
- **Module:** layout
- **Target function:** `two_mul_with_bit1`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `y1 != 0,`
- **Generated ensures:** `2 * x1 * y1 + y1 == (2 * x1 + 1) * y1,`
- **Ground truth ensures:** `(2 * x1 + 1) / (2 * y1) == x1 / y1`

### Case 367 — `MA__pigeonhole__lemma_nat_range`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `lemma_nat_range`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `lo <= hi,`
- **Ensures:** `set_nat_range(lo, hi).finite(),
        set_nat_range(lo, hi).len() == hi - lo,`

### Case 368 — `MA__pigeonhole__nat_set_size`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `nat_set_size`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|i: nat| s.contains(i) ==> i < bound,`
- **Ground truth requires:** `forall |i: nat| (0 <= i < bound <==> s.contains(i)),`
- **Generated ensures:** `s.finite(),
        s.len() <= bound,`
- **Ground truth ensures:** `s.finite(),
        s.len() == bound,`

### Case 369 — `MA__pigeonhole__pigeonhole_missing_idx_implies_double`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `pigeonhole_missing_idx_implies_double`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `len >= 2,
        forall |i: nat| (0 <= i < len <==> m.dom().contains(i)),
        forall |i: nat| (#[trigger] m.dom().contains(i) ==> (
            0`
- **Ground truth requires:** `forall |i: nat| (0 <= i < len <==> m.dom().contains(i)),
        forall |i: nat| (#[trigger] m.dom().contains(i) ==> (
            0 <= m[i] < len &&`
- **Generated ensures:** `r.0 != r.1,
        m.dom().contains(r.0),
        m.dom().contains(r.1),
        0 <= r.0 < len,
        0 <= r.1 < len,
        m[r.0] == m[r.1],`
- **Ground truth ensures:** `(`

### Case 370 — `MA__pigeonhole__pigeonhole_missing_idx_implies_double_helper`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `pigeonhole_missing_idx_implies_double_helper`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |i: nat| 0 <= i < len ==> m.contains_key(i),
        forall |i: nat| 0 <= i < len ==> 0 <= #[trigger] m[i] < len,
        0 <= missing < len,`
- **Ground truth requires:** `len >= 2,
        forall |i: nat| (0 <= i < len <==> m.dom().contains(i)),
        forall |i: nat| (#[trigger] m.dom().contains(i) ==> (
            0`
- **Generated ensures:** `0 <= dup2 < k,
        exists |dup1: nat| 0 <= dup1 < k && dup1 != dup2 && m[dup1] == m[dup2],`
- **Ground truth ensures:** `m.dom().contains(dup2),
        exists |dup1| #![auto] dup1 != dup2 && m.dom().contains(dup1) && 0 <= dup1 < len && m[dup1] == m[dup2],`

### Case 371 — `MA__pigeonhole__pigeonhole_too_many_elements_implies_double`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `pigeonhole_too_many_elements_implies_double`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `forall |i: nat| (0 <= i <= len <==> m.dom().contains(i)),
        forall |i: nat| (#[trigger] m.dom().contains(i) ==> 0 <= m[i] < len),`
- **Ground truth requires:** `forall |i: nat| (0 <= i < len + 1 <==> m.dom().contains(i)),
        forall |i: nat| #[trigger] m.dom().contains(i) ==> 0 <= m[i] < len,`
- **Generated ensures:** `(`
- **Ground truth ensures:** `(`

### Case 372 — `MA__pigeonhole__set_mismatch`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `set_mismatch`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s1.finite(),
        s2.finite(),
        s1.contains(missing),
        !s2.contains(missing),`
- **Ground truth requires:** `s1.finite(),
        s2.finite(),
        s1.len() == s2.len(),
        forall |elt| s2.contains(elt) ==> s1.contains(elt),
        s1.contains(missin`
- **Generated ensures:** `s1 != s2,`
- **Ground truth ensures:** `false,`

### Case 373 — `MA__pigeonhole__singleton_set_unique_elt`

- **Project:** Memory-Allocator
- **Module:** pigeonhole
- **Target function:** `singleton_set_unique_elt`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `s.len() == 1,
        s.contains(a),
        s.contains(b),`
- **Ground truth requires:** `s.finite(),
        s.len() == 1,
        s.contains(a),
        s.contains(b),`
- **Generated ensures:** `a == b,`
- **Ground truth ensures:** `a == b,`

### Case 374 — `NO__exec__utils__rids_match_add_none`

- **Project:** Node-Replication
- **Module:** exec.utils
- **Target function:** `rids_match_add_none`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),`
- **Ground truth requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        rids_match(bools, rids, bools_start, bools_en`
- **Generated ensures:** `rids_match(bools, rids, bools_start, bools_end, rids_start, rids_end)
            == rids_match(bools.push(None), rids, bools_start, bools_end, rids_s`
- **Ground truth ensures:** `rids_match(bools.push(Option::None), rids, bools_start, bools_end, rids_start, rids_end),`

### Case 375 — `NO__exec__utils__rids_match_add_rid`

- **Project:** Node-Replication
- **Module:** exec.utils
- **Target function:** `rids_match_add_rid`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        rids_match(bools, rids, bools_start, bools_en`
- **Ground truth requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        rids_match(bools, rids, bools_start, bools_en`
- **Generated ensures:** `rids_match(
            bools.push(Some(rid)),
            rids.push(rid),
            bools_start,
            (bools_end + 1) as nat,
            ri`
- **Ground truth ensures:** `rids_match(
            bools.push(Option::Some(rid)),
            rids.push(rid),
            bools_start,
            bools_end,
            rids_st`

### Case 376 — `NO__exec__utils__rids_match_pop`

- **Project:** Node-Replication
- **Module:** exec.utils
- **Target function:** `rids_match_pop`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        bools_end > bools_start,
        rids_match(b`
- **Ground truth requires:** `0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        rids_match(bools, rids, bools_start, bools_en`
- **Generated ensures:** `bools[bools_end - 1].is_Some() ==> (
            rids_end > rids_start
            && rids[rids_end - 1] == bools[bools_end - 1].get_Some_0()`
- **Ground truth ensures:** `bools_end == bools_start ==>`

### Case 377 — `NO__spec__cyclicbuffer__log_entry_alive_value_wrap_around`

- **Project:** Node-Replication
- **Module:** spec.cyclicbuffer
- **Target function:** `log_entry_alive_value_wrap_around`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `buffer_size > 0,
        i >= 0,`
- **Ground truth requires:** `buffer_size > 0,`
- **Generated ensures:** `log_entry_alive_value(i, buffer_size) == !log_entry_alive_value(i + buffer_size as int, buffer_size),`
- **Ground truth ensures:** `log_entry_alive_value(i, buffer_size) != log_entry_alive_value(
            i + (buffer_size as int),
            buffer_size,
        ),`

### Case 378 — `NO__spec__cyclicbuffer__log_entry_alive_wrap_around_helper`

- **Project:** Node-Replication
- **Module:** spec.cyclicbuffer
- **Target function:** `log_entry_alive_wrap_around_helper`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `buffer_size == LOG_SIZE,
        low <= high,
        high - low <= buffer_size,`
- **Ground truth requires:** `buffer_size == LOG_SIZE,
        forall|i: nat| i < buffer_size <==> alive_bits.contains_key(i),
        low <= high <= low + buffer_size,
        for`
- **Generated ensures:** `forall |i: nat| low <= i < high ==> 
            #[trigger] log_entry_alive_value(i as int, buffer_size) !=
            log_entry_alive_value((i + buf`
- **Ground truth ensures:** `forall|i: int|
            low + buffer_size <= i < high + buffer_size ==> !#[trigger] log_entry_is_alive(
                alive_bits,`

### Case 379 — `NO__spec__cyclicbuffer__map_min_value_smallest`

- **Project:** Node-Replication
- **Module:** spec.cyclicbuffer
- **Target function:** `map_min_value_smallest`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|i: nat| i <= idx ==> m.contains_key(i),`
- **Ground truth requires:** `forall|i| 0 <= i <= idx ==> m.contains_key(i),`
- **Generated ensures:** `forall|i: nat| i <= idx ==> map_min_value(m, idx) <= m.index(i),`
- **Ground truth ensures:** `forall|n| 0 <= n <= idx as nat ==> map_min_value(m, idx) <= m.index(n),
        map_contains_value(m, map_min_value(m, idx)),`

### Case 380 — `NO__spec__linearization__pop_rid`

- **Project:** Node-Replication
- **Module:** spec.linearization
- **Target function:** `pop_rid`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `t.len() > 0,`
- **Ground truth requires:** `!t.is_empty(),
        t.finite(),`
- **Generated ensures:** `t.contains(res.1),
        res.0 == t.remove(res.1),`
- **Ground truth ensures:** `res.0.len() < t.len(),
        t.contains(res.1),
        res.0 =~= t.remove(res.1),
        res.0.finite(),`

### Case 381 — `NO__spec__linearization__state_at_version_preserves`

- **Project:** Node-Replication
- **Module:** spec.linearization
- **Target function:** `state_at_version_preserves`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `i <= a.len(),`
- **Ground truth requires:** `b == a.push(x),
        i <= a.len(),
        i <= b.len(),`
- **Generated ensures:** `compute_nrstate_at_version::<DT>(a.push(x).add(b), i) == compute_nrstate_at_version::<DT>(a, i),`
- **Ground truth ensures:** `compute_nrstate_at_version::<DT>(a, i) == compute_nrstate_at_version::<DT>(b, i),`

### Case 382 — `NO__spec__linearization__trick_equiv`

- **Project:** Node-Replication
- **Module:** spec.linearization
- **Target function:** `trick_equiv`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `behavior_equiv(a, b),
        a.is_Stepped(),
        a2.is_Stepped(),
        a.get_Stepped_2() == a2.get_Stepped_2(),
        a.get_Stepped_1().is_I`
- **Ensures:** `behavior_equiv(a2, b),`

### Case 383 — `NO__spec__unbounded_log__LogRangeMatchesQueue_append`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeMatchesQueue_append`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `// The original log range matches the queue
        LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, node_id, updates),`
- **Ground truth requires:** `0 <= queueIndex <= queue.len(),
        logIndexLower <= logIndexUpper,
        log_entry.node_id == node_id,
        new_updates.contains_key(new_rid`
- **Generated ensures:** `// The new log range still matches the queue
        LogRangeMatchesQueue(queue, new_log, queueIndex, logIndexLower, logIndexUpper, node_id, new_updat`
- **Ground truth ensures:** `LogRangeMatchesQueue(
            queue.push(new_rid),
            new_log,
            queueIndex,
            logIndexLower,
            logIndexUpp`

### Case 384 — `NO__spec__unbounded_log__LogRangeMatchesQueue_append_other`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeMatchesQueue_append_other`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `// The log entry is from a different node
        log_entry.node_id != node_id,
        // The new log is the old log with the new entry appended at l`
- **Ground truth requires:** `0 <= queueIndex <= queue.len(),
        logIndexLower <= logIndexUpper <= logLen,
        log_entry.node_id != node_id,
        new_updates.contains_k`
- **Generated ensures:** `// Property is preserved with the new log
        LogRangeMatchesQueue(queue, new_log, queueIndex, logIndexLower, logIndexUpper, node_id, new_updates)`
- **Ground truth ensures:** `LogRangeMatchesQueue(
            queue,
            new_log,
            queueIndex,
            logIndexLower,
            logIndexUpper,`

### Case 385 — `NO__spec__unbounded_log__LogRangeMatchesQueue_append_other_augment`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeMatchesQueue_append_other_augment`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `// The original log range matches the queue
        LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, node_id, updates),`
- **Ground truth requires:** `0 <= queueIndex <= queue.len(),
        logIndexLower <= logIndexUpper,
        log_entry.node_id != node_id,
        new_updates.contains_key(new_rid`
- **Generated ensures:** `// The log range still matches the queue with the new log (same range)
        LogRangeMatchesQueue(queue, new_log, queueIndex, logIndexLower, logInde`
- **Ground truth ensures:** `LogRangeMatchesQueue(
            queue,
            new_log,
            queueIndex,
            logIndexLower,
            logIndexUpper + 1,`

### Case 386 — `NO__spec__unbounded_log__LogRangeMatchesQueue_update_change`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeMatchesQueue_update_change`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `logIndexLower <= logIndexUpper,
        LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, nodeId, updates1),
        forall|r`
- **Ground truth requires:** `0 <= queueIndex <= queue.len(),
        logIndexLower <= logIndexUpper,
        LogRangeMatchesQueue(
            queue,
            log,`
- **Generated ensures:** `LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, nodeId, updates2),`
- **Ground truth ensures:** `LogRangeMatchesQueue(
            queue,
            log,
            queueIndex,
            logIndexLower,
            logIndexUpper,
            no`

### Case 387 — `NO__spec__unbounded_log__LogRangeMatchesQueue_update_change_2`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeMatchesQueue_update_change_2`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `logIndexLower <= logIndexUpper,
        LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, nodeId, updates1),
        forall|i`
- **Ground truth requires:** `0 <= queueIndex <= queue.len(),
        logIndexLower <= logIndexUpper,
        LogRangeMatchesQueue(
            queue,
            log,`
- **Generated ensures:** `LogRangeMatchesQueue(queue, log, queueIndex, logIndexLower, logIndexUpper, nodeId, updates2),`
- **Ground truth ensures:** `LogRangeMatchesQueue(
            queue,
            log,
            queueIndex,
            logIndexLower,
            logIndexUpper,
            no`

### Case 388 — `NO__spec__unbounded_log__LogRangeNoNodeId_append_other`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `LogRangeNoNodeId_append_other`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `logIndexLower <= logIndexUpper,
        LogRangeNoNodeId::<DT>(log, logIndexLower, logIndexUpper, node_id),
        forall |i: nat| logIndexLower <= i`
- **Ground truth requires:** `logIndexLower <= logIndexUpper,
        log_entry.node_id != node_id,
        LogRangeNoNodeId(log, logIndexLower, logIndexUpper, node_id),
        ne`
- **Generated ensures:** `LogRangeNoNodeId::<DT>(new_log, logIndexLower, logIndexUpper, node_id),`
- **Ground truth ensures:** `LogRangeNoNodeId(new_log, logIndexLower, logIndexUpper + 1, node_id),`

### Case 389 — `NO__spec__unbounded_log__combiner_request_ids_finite`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `combiner_request_ids_finite`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `combiners.dom().finite(),`
- **Ensures:** `combiner_request_ids(combiners).finite(),`

### Case 390 — `NO__spec__unbounded_log__combiner_request_ids_not_contains`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `combiner_request_ids_not_contains`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `combiners.dom().finite(),
        combiner_request_id_fresh(combiners, rid),`
- **Ground truth requires:** `combiners.dom().finite(),`
- **Generated ensures:** `!combiner_request_ids(combiners).contains(rid),`
- **Ground truth ensures:** `combiner_request_id_fresh(combiners, rid) <==> !combiner_request_ids(combiners).contains(
            rid,
        ),`

### Case 391 — `NO__spec__unbounded_log__compute_nrstate_at_version_preserves`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `compute_nrstate_at_version_preserves`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|i: LogIdx| 0 <= i < version ==> a.contains_key(i),
        forall|i: LogIdx| 0 <= i < version ==> b.contains_key(i),
        forall|i: LogIdx|`
- **Ground truth requires:** `forall|i| 0 <= i < version ==> a.contains_key(i),
        forall|i| 0 <= i < version ==> a[i] == b[i],`
- **Generated ensures:** `compute_nrstate_at_version::<DT>(a, version) === compute_nrstate_at_version::<DT>(b, version),`
- **Ground truth ensures:** `compute_nrstate_at_version(a, version) == compute_nrstate_at_version(b, version),`

### Case 392 — `NO__spec__unbounded_log__concat_LogRangeNoNodeId_LogRangeMatchesQueue`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `concat_LogRangeNoNodeId_LogRangeMatchesQueue`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `a <= b <= c,
        LogContainsEntriesUpToHere(log, c),
        LogRangeNoNodeId(log, a, b, nodeId),
        LogRangeMatchesQueue(queue, log, queueIn`
- **Ground truth requires:** `a <= b <= c,
        0 <= queueIndex <= queue.len(),
        LogRangeNoNodeId(log, a, b, nodeId),
        LogRangeMatchesQueue(queue, log, queueIndex,`
- **Generated ensures:** `LogRangeMatchesQueue(queue, log, queueIndex, a, c, nodeId, updates),`
- **Ground truth ensures:** `LogRangeMatchesQueue(queue, log, queueIndex, a, c, nodeId, updates),`

### Case 393 — `NO__spec__unbounded_log__element_outside_set`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `element_outside_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `s.finite(),`
- **Ensures:** `!s.contains(r),`

### Case 394 — `NO__spec__unbounded_log__get_fresh_nat_not_in`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `get_fresh_nat_not_in`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `reqs.finite(),
        combiner.dom().finite(),`
- **Ensures:** `!reqs.contains(get_fresh_nat(reqs, combiner)),
        combiner_request_id_fresh(combiner, get_fresh_nat(reqs, combiner)),`

### Case 395 — `NO__spec__unbounded_log__max_of_set`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log
- **Target function:** `max_of_set`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s.finite(),
        s.len() > 0,`
- **Ground truth requires:** `s.finite(),`
- **Generated ensures:** `s.contains(r),
        forall|x: nat| s.contains(x) ==> x <= r,`
- **Ground truth ensures:** `forall|x: nat| #[trigger] s.contains(x) ==> x <= r,`

### Case 396 — `NO__spec__unbounded_log_refines_simplelog__state_at_version_refines`

- **Project:** Node-Replication
- **Module:** spec.unbounded_log_refines_simplelog
- **Target function:** `state_at_version_refines`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `idx <= gtail,
        s_log == interp_log::<DT>(gtail, i_log),
        forall|i: nat| i < gtail ==> i_log.contains_key(i),`
- **Ground truth requires:** `forall|i| 0 <= i < gtail ==> i_log.contains_key(i),
        0 <= idx <= s_log.len(),
        idx <= gtail,
        s_log == interp_log(gtail, i_log),`
- **Generated ensures:** `i_nrstate_at_version::<DT>(i_log, idx) == s_nrstate_at_version::<DT>(s_log, idx),`
- **Ground truth ensures:** `s_nrstate_at_version::<DT>(s_log, idx) == i_nrstate_at_version::<DT>(i_log, idx),`

### Case 397 — `NO__spec__utils__int_mod_less_than_same`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `int_mod_less_than_same`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `i >= 0,
        len > 0,`
- **Ground truth requires:** `0 <= i < len,
        len > 0,`
- **Generated ensures:** `i % len < len,`
- **Ground truth ensures:** `(i % len) == i,`

### Case 398 — `NO__spec__utils__map_new_rec_dom_finite`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `map_new_rec_dom_finite`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `map_new_rec(dom, val).dom().finite(),`
- **Ground truth ensures:** `map_new_rec(dom, val).dom().finite(),
        forall|n: nat| 0 <= n <= dom <==> map_new_rec(dom, val).contains_key(n),
        forall|n|
            (`

### Case 399 — `NO__spec__utils__seq_to_set_equal_rec`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `seq_to_set_equal_rec`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 400 — `NO__spec__utils__seq_to_set_is_finite`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `seq_to_set_is_finite`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `seq_to_set(seq).finite(),`

### Case 401 — `NO__spec__utils__seq_to_set_rec_contains`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `seq_to_set_rec_contains`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors"), but the postcondition isn't verified. Let me add more proof details to help the verifier:`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 402 — `NO__spec__utils__seq_to_set_rec_is_finite`

- **Project:** Node-Replication
- **Module:** spec.utils
- **Target function:** `seq_to_set_rec_is_finite`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `seq_to_set_rec(seq).finite(),`

### Case 403 — `NR__definitions_u__impl2__entry_base`

- **Project:** NRKernel
- **Module:** definitions_u.impl2
- **Target function:** `entry_base`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `a <= c,
        b <= d,`
- **Ensures:** `// Including `0 <=` here because it's used in a place where this is part of an overflow VC
        // and non-nonlinear z3 can't even deal with that.`

### Case 404 — `NR__definitions_u__impl2__next_entry_base`

- **Project:** NRKernel
- **Module:** definitions_u.impl2
- **Target function:** `next_entry_base`
- **Verification:** :x: Verification failed (`verification results:: 4 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `layer < self@.layers.len()`
- **Ensures:** `X86_MAX_ENTRY_SIZE * (X86_NUM_ENTRIES + 1) < 0x10000000000000000,
        MAX_BASE + X86_MAX_ENTRY_SIZE * (X86_NUM_ENTRIES + 1) < 0x10000000000000000,`

### Case 405 — `NR__definitions_u__impl3__lemma_entry_sizes_aligned`

- **Project:** NRKernel
- **Module:** definitions_u.impl3
- **Target function:** `lemma_entry_sizes_aligned`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.inv(),
            i <= j,
            j < self.layers.len(),`
- **Ensures:** `aligned(self.entry_size(i), self.entry_size(j)),`

### Case 406 — `NR__definitions_u__impl3__lemma_entry_sizes_aligned_auto`

- **Project:** NRKernel
- **Module:** definitions_u.impl3
- **Target function:** `lemma_entry_sizes_aligned_auto`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),`
- **Ground truth requires:** `(self.inv() && i <= j && j < self.layers.len());`
- **Generated ensures:** `forall|i: nat, j: nat|
                i <= j && j < self.layers.len() ==>
                aligned(self.entry_size(i), self.entry_size(j)),`
- **Ground truth ensures:** `forall|i: nat, j: nat|
                self.inv() && i <= j && j < self.layers.len() ==>
                aligned(self.entry_size(i), self.entry_size(j`

### Case 407 — `NR__definitions_u__impl3__lemma_entry_sizes_increase`

- **Project:** NRKernel
- **Module:** definitions_u.impl3
- **Target function:** `lemma_entry_sizes_increase`
- **Verification:** :x: Verification failed (`verification results:: 3 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `self.inv(),
            i <= j,
            j < self.layers.len(),`
- **Ground truth requires:** `self.inv(),
            i < j,
            j < self.layers.len(),`
- **Generated ensures:** `self.entry_size(i) >= self.entry_size(j),`
- **Ground truth ensures:** `self.entry_size(i) >= self.entry_size(j),`

### Case 408 — `NR__definitions_u__lemma_maxphyaddr_facts`

- **Project:** NRKernel
- **Module:** definitions_u
- **Target function:** `lemma_maxphyaddr_facts`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `32 <= MAX_PHYADDR_WIDTH <= 52,`
- **Ground truth ensures:** `0xFFFFFFFF <= MAX_PHYADDR <= 0xFFFFFFFFFFFFF`

### Case 409 — `NR__definitions_u__lemma_new_seq`

- **Project:** NRKernel
- **Module:** definitions_u
- **Target function:** `lemma_new_seq`
- **Verification:** :x: Verification failed (`verification results:: 1 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `new_seq(i, e).len() == i,
        forall|j: int| 0 <= j < i ==> new_seq(i, e)[j] == e,`
- **Ground truth ensures:** `new_seq(i, e).len() == i,
        forall|j: nat| j < i ==> new_seq(i, e).index(j as int) === e,`

### Case 410 — `NR__extra__aligned_transitive`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `aligned_transitive`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `aligned(a, b),
        aligned(b, c),
        b > 0,
        c > 0,`
- **Ground truth requires:** `0 < b,
        0 < c,
        aligned(a, b),
        aligned(b, c),`
- **Generated ensures:** `aligned(a, c),`
- **Ground truth ensures:** `aligned(a, c)`

### Case 411 — `NR__extra__aligned_transitive_auto`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `aligned_transitive_auto`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — missing requires clause
- **Generated requires:** `(none)`
- **Ground truth requires:** `0 < b,
        0 < c,
        aligned(a, b),
        aligned(b, c),`
- **Generated ensures:** `forall|a: nat, b: nat, c: nat|
            0 < b && 0 < c && aligned(a, b) && aligned(b, c) ==> aligned(a, c),`
- **Ground truth ensures:** `forall|a: nat, b: nat, c: nat| 0 < b && 0 < c && aligned(a, b) && aligned(b, c) ==> aligned(a, c),`

### Case 412 — `NR__extra__assert_maps_equal_contains_pair`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `assert_maps_equal_contains_pair`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall |k: K| m1.dom().contains(k) <==> m2.dom().contains(k),
        forall |k: K| m1.dom().contains(k) ==> m1[k] == m2[k],`
- **Ground truth requires:** `forall|k:K,v:V| m1.contains_pair(k, v) ==> m2.contains_pair(k, v),
        forall|k:K,v:V| m2.contains_pair(k, v) ==> m1.contains_pair(k, v),`
- **Generated ensures:** `m1 =~= m2,`
- **Ground truth ensures:** `m1 === m2`

### Case 413 — `NR__extra__lemma_aligned_iff_eq_mul_div`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `lemma_aligned_iff_eq_mul_div`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `b > 0,`
- **Ground truth requires:** `b > 0`
- **Generated ensures:** `aligned(a, b) <==> a == (a / b) * b,`
- **Ground truth ensures:** `aligned(a, b) <==> a == b * (a / b)`

### Case 414 — `NR__extra__lemma_set_of_first_n_nat_is_finite`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `lemma_set_of_first_n_nat_is_finite`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** _(none)_
- **Ensures:** `Set::new(|i: nat| i < n).finite(),`

### Case 415 — `NR__extra__leq_add_aligned_less`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `leq_add_aligned_less`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors"). The postcondition needs proof in the body. Let me add the proof:`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 416 — `NR__extra__mod_add_zero`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `mod_add_zero`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `c > 0,
        a % c == 0,
        b % c == 0,`
- **Ground truth requires:** `aligned(a, c), aligned(b, c), c > 0`
- **Generated ensures:** `(a + b) % c == 0,`
- **Ground truth ensures:** `aligned(a + b, c)`

### Case 417 — `NR__extra__mod_mult_zero_implies_mod_zero`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `mod_mult_zero_implies_mod_zero`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `b > 0,
        c > 0,
        a % (b * c) == 0,`
- **Ground truth requires:** `aligned(a, b * c), b > 0, c > 0`
- **Generated ensures:** `a % b == 0,`
- **Ground truth ensures:** `aligned(a, b)`

### Case 418 — `NR__extra__subtract_mod_eq_zero`

- **Project:** NRKernel
- **Module:** extra
- **Target function:** `subtract_mod_eq_zero`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `c > 0,
        a % c == 0,
        b % c == 0,
        a >= b,`
- **Ground truth requires:** `aligned(a, c), aligned(b, c), a <= b, c > 0`
- **Generated ensures:** `(a - b) % (c as int) == 0,`
- **Ground truth ensures:** `aligned((b - a) as nat, c)`

### Case 419 — `NR__impl_u__indexing__lemma_entry_base_from_index`

- **Project:** NRKernel
- **Module:** impl_u.indexing
- **Target function:** `lemma_entry_base_from_index`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `entry_size > 0,
        aligned(base, entry_size),`
- **Ground truth requires:** `0 < entry_size,`
- **Generated ensures:** `aligned(entry_base_from_index(base, idx, entry_size), entry_size),`
- **Ground truth ensures:** `entry_base_from_index(base, idx, entry_size) < next_entry_base_from_index(base, idx, entry_size),
        forall|idx2: nat|
            #![trigger ent`

### Case 420 — `NR__impl_u__indexing__lemma_entry_base_from_index_support`

- **Project:** NRKernel
- **Module:** impl_u.indexing
- **Target function:** `lemma_entry_base_from_index_support`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 421 — `NR__impl_u__indexing__lemma_index_from_base_and_addr`

- **Project:** NRKernel
- **Module:** impl_u.indexing
- **Target function:** `lemma_index_from_base_and_addr`
- **Verification:** :x: Verification failed (`verification results:: 0 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `addr >= base,
        entry_size > 0,
        num_entries > 0,
        between(addr, base, base + num_entries * entry_size),`
- **Ground truth requires:** `addr >= base,
        addr < entry_base_from_index(base, num_entries, entry_size),
        entry_size > 0,`
- **Generated ensures:** `index_from_base_and_addr(base, addr, entry_size) < num_entries,`
- **Ground truth ensures:** `(`

### Case 422 — `NR__impl_u__l1__impl2__lemma_empty_implies_interp_aux_empty`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_empty_implies_interp_aux_empty`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.well_formed(),
            self.empty(),`
- **Ground truth requires:** `self.inv(),
             self.empty(),`
- **Generated ensures:** `self.interp_aux(i) =~= Map::<nat, PTE>::empty(),`
- **Ground truth ensures:** `self.interp_aux(i) === Map::empty(),
            self.interp_aux(i).dom() === Set::empty(),`

### Case 423 — `NR__impl_u__l1__impl2__lemma_empty_implies_interp_empty`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_empty_implies_interp_empty`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.inv(),
            self.empty(),`
- **Ensures:** `self.interp() === Map::empty(),
            self.interp().dom() === Set::empty(),`

### Case 424 — `NR__impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_aux_equal`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_entries_interp_equal_implies_interp_aux_equal`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            other.inv(),
            self.entries.len() == other.entries.len(),
            forall|j: nat| i <= j && j < self.entries.len(`
- **Ground truth requires:** `self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,
            self.base_vaddr == other.`
- **Generated ensures:** `self.interp_aux(i) == other.interp_aux(i),`
- **Ground truth ensures:** `self.interp_aux(i) === other.interp_aux(i),`

### Case 425 — `NR__impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_equal`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_entries_interp_equal_implies_interp_equal`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,
            self.base_vaddr == other.`
- **Ensures:** `self.interp() === other.interp(),`

### Case 426 — `NR__impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_aux_insert`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_entries_interp_insert_implies_interp_aux_insert`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            other.inv(),
            i <= idx,
            idx < self.entries.len(),
            self.entries.len() == other.entries.len()`
- **Ground truth requires:** `idx < self.entries.len(),
            self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,`
- **Generated ensures:** `other.interp_aux(i).contains_pair(vaddr, pte),`
- **Ground truth ensures:** `if idx < i`

### Case 427 — `NR__impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_insert`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_entries_interp_insert_implies_interp_insert`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `idx < self.entries.len(),
            self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,`
- **Ensures:** `other.interp() === self.interp().insert(vaddr, pte),`

### Case 428 — `NR__impl_u__l1__impl2__lemma_entries_interp_remove_implies_interp_aux_remove`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_entries_interp_remove_implies_interp_aux_remove`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            other.inv(),
            idx < self.entries.len(),
            i <= idx,
            other.entries.len() == self.entries.len()`
- **Ground truth requires:** `idx < self.entries.len(),
            self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,`
- **Generated ensures:** `self.interp_aux(i).remove(vaddr) =~= other.interp_aux(i),`
- **Ground truth ensures:** `if idx < i`

### Case 429 — `NR__impl_u__l1__impl2__lemma_interp_aux_between`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_aux_between`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `i <= self.entries.len(),
            #[trigger] self.interp_aux(i).contains_pair(va, pte),
            #[trigger] self.inv(),`
- **Ground truth requires:** `#[trigger] self.inv(),
            #[trigger] self.interp_aux(i).contains_pair(va, pte),`
- **Generated ensures:** `self.entry_base(i) <= va < self.upper_vaddr(),
            self.entry_base(i) < va + pte.frame.size <= self.upper_vaddr(),`
- **Ground truth ensures:** `self.entry_base(i) <= va < self.upper_vaddr(),
            self.entry_base(i) < va + self.interp_aux(i)[va].frame.size <= self.upper_vaddr(),`

### Case 430 — `NR__impl_u__l1__impl2__lemma_interp_aux_contains_implies_interp_of_entry_contains`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_aux_contains_implies_interp_of_entry_contains`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            j < self.entries.len(),`
- **Ground truth requires:** `self.inv(),`
- **Generated ensures:** `forall|vaddr: nat| self.interp_of_entry(j).contains_key(vaddr) ==> self.interp_aux(0).contains_key(vaddr),`
- **Ground truth ensures:** `forall|base: nat, pte: PTE|
                self.interp_aux(j).contains_pair(base, pte) ==>
                exists|i: nat| #![auto] j <= i < self.num_`

### Case 431 — `NR__impl_u__l1__impl2__lemma_interp_contains_implies_interp_of_entry_contains`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_contains_implies_interp_of_entry_contains`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors". This confirms the spec is syntactically correct. The verification failed because the proof body is empty (which is expected for spec generation).`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 432 — `NR__impl_u__l1__impl2__lemma_interp_entries_insert_implies_interp_insert`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_entries_insert_implies_interp_insert`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — requires clause differs
- **Generated requires:** `self.inv(),
            j < self.num_entries(),
            self.update(j, n).inv(),
            self.interp_of_entry(j).insert(base, pte) == self.upd`
- **Ground truth requires:** `self.inv(),
            j < self.num_entries(),
            // !self.interp_aux(i).contains_key(base),
            self.update(j, n).inv(),`
- **Generated ensures:** `self.interp().insert(base, pte) == self.update(j, n).interp(),`
- **Ground truth ensures:** `self.interp().insert(base, pte) == self.update(j, n).interp(),`

### Case 433 — `NR__impl_u__l1__impl2__lemma_interp_entries_remove_implies_interp_remove`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_entries_remove_implies_interp_remove`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `idx < self.entries.len(),
            self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,`
- **Ground truth requires:** `self.inv(),
            other.inv(),
            self.arch == other.arch,
            self.layer == other.layer,
            self.base_vaddr == other.`
- **Generated ensures:** `other.interp() === self.interp().remove(vaddr),`
- **Ground truth ensures:** `other.interp() == self.interp().remove(vaddr),`

### Case 434 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_between`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_between`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `#[trigger] self.inv(),
            #[trigger] self.interp_of_entry(i).contains_pair(va, pte),`
- **Ground truth requires:** `i < self.entries.len(),
            #[trigger] self.interp_of_entry(i).contains_pair(va, pte),
            #[trigger] self.inv(),`
- **Generated ensures:** `self.entry_base(i) <= va < self.next_entry_base(i),
            self.entry_base(i) < va + self.interp_of_entry(i)[va].frame.size <= self.next_entry_ba`
- **Ground truth ensures:** `self.entry_base(i) <= va < self.next_entry_base(i),
            self.entry_base(i) < va + pte.frame.size <= self.next_entry_base(i),
            //i <`

### Case 435 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_contains_mapping_implies_interp_aux_contains_mapping`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_contains_mapping_implies_interp_aux_contains_mapping`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.inv(),
            i <= j,
            j < self.entries.len(),`
- **Ground truth requires:** `self.inv(),
             i <= j,
             j < self.entries.len(),`
- **Generated ensures:** `forall|va: nat, pte: PTE|
                self.interp_of_entry(j).contains_pair(va, pte)
                ==> self.interp_aux(i).contains_pair(va, pte)`
- **Ground truth ensures:** `forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp_aux(i).contains_pair(va, pte),
            forall|va`

### Case 436 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.inv(),
            j < self.entries.len(),`
- **Ground truth requires:** `self.inv(),
             j < self.entries.len(),`
- **Generated ensures:** `forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp().contains_pair(va, pte),
            forall|va: nat`
- **Ground truth ensures:** `forall|va: nat| #![auto] self.interp_of_entry(j).contains_key(va) ==> self.interp().contains_key(va),
            forall|va: nat, pte: PTE| #![auto] s`

### Case 437 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_disjoint_mappings`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_disjoint_mappings`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            i < self.entries.len(),
            j < self.entries.len(),
            i != j,`
- **Ground truth requires:** `i < j < self.entries.len(),
            self.inv(),`
- **Generated ensures:** `self.interp_of_entry(i).dom().disjoint(self.interp_of_entry(j).dom()),`
- **Ground truth ensures:** `forall|va, pte| self.interp_of_entry(i).contains_pair(va, pte) ==> !self.interp_of_entry(j).contains_pair(va, pte),
            forall|va| self.interp`

### Case 438 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_insert_implies_interp_aux_insert`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_insert_implies_interp_aux_insert`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 439 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_insert_implies_interp_insert`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_insert_implies_interp_insert`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `self.inv(),
            j < self.num_entries(),
            self.update(j, n).inv(),
            self.interp_of_entry(j).insert(base, pte) == self.upd`
- **Ensures:** `self.interp().insert(base, pte) == self.update(j, n).interp(),`

### Case 440 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_insert_page_implies_interp_insert_page`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_insert_page_implies_interp_insert_page`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 441 — `NR__impl_u__l1__impl2__lemma_interp_of_entry_key_between`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_interp_of_entry_key_between`
- **Verification:** :x: Verification failed (`verification results:: 9 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `i < self.entries.len(),
            #[trigger] self.interp_of_entry(i).contains_key(va),
            #[trigger] self.inv(),`
- **Ensures:** `self.entry_base(i) <= va < self.next_entry_base(i),`

### Case 442 — `NR__impl_u__l1__impl2__lemma_new_empty_dir`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_new_empty_dir`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `self.inv(),
            entry < self.num_entries(),
            self.layer + 1 < self.arch.layers.len(),`
- **Ground truth requires:** `self.inv(),
            entry < self.num_entries(),
            self.layer + 1 < self.arch.layers.len(),`
- **Generated ensures:** `self.new_empty_dir(entry).inv(),`
- **Ground truth ensures:** `self.new_empty_dir(entry).inv(),
            self.new_empty_dir(entry).entries.len() == self.arch.num_entries((self.layer + 1) as nat),
            fo`

### Case 443 — `NR__impl_u__l1__impl2__lemma_nonempty_implies_interp_contains`

- **Project:** NRKernel
- **Module:** impl_u.l1.impl2
- **Target function:** `lemma_nonempty_implies_interp_contains`
- **Verification:** :x: Verification failed (`verification results:: 10 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.inv(),
            !self.empty(),`
- **Ground truth requires:** `self.inv(),
            self.no_empty_directories(),
            !self.empty(),`
- **Generated ensures:** `exists|va: nat| self.interp().contains_key(va),`
- **Ground truth ensures:** `exists|b: nat, pte: PTE|
                self.interp().contains_pair(b, pte)
                && self.arch.contains_entry_size_at_index_atleast(pte.fra`

### Case 444 — `NR__impl_u__l2_impl__impl0__address`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `address`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `(none)`
- **Ground truth requires:** `self.layer() <= 3,
            self@ is Page ==> 0 < self.layer(),
            self.hp_pat_is_zero(),
            self.all_mb0_bits_are_zero(),`
- **Generated ensures:** `res == self.entry & MASK_ADDR,`
- **Ground truth ensures:** `res == match self@`

### Case 445 — `NR__impl_u__l2_impl__impl0__lemma_addr_mask_when_hp_pat_is_zero`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `lemma_addr_mask_when_hp_pat_is_zero`
- **Verification:** :x: Verification failed (`verification results:: 19 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.hp_pat_is_zero(),
            self@ is Page,
            self.layer == 1 || self.layer == 2,`
- **Ground truth requires:** `self.hp_pat_is_zero(),
            self.all_mb0_bits_are_zero(),
            self@ is Page,`
- **Generated ensures:** `self.layer == 1 ==> self.entry & MASK_ADDR == self.entry & MASK_L1_PG_ADDR,
            self.layer == 2 ==> self.entry & MASK_ADDR == self.entry & MAS`
- **Ground truth ensures:** `self.layer == 1 ==> self.entry & MASK_L1_PG_ADDR == self.entry & MASK_ADDR,
            self.layer == 2 ==> self.entry & MASK_L2_PG_ADDR == self.entry`

### Case 446 — `NR__impl_u__l2_impl__impl0__lemma_new_entry_addr_mask_is_address`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `lemma_new_entry_addr_mask_is_address`
- **Verification:** :x: Verification failed (`verification results:: 19 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `1 <= layer <= 3,
            addr_is_zero_padded(layer as nat, address, is_page),`
- **Ground truth requires:** `layer <= 3,
            if is_page`
- **Generated ensures:** `address & MASK_ADDR == address,`
- **Ground truth ensures:** `(`

### Case 447 — `NR__impl_u__l2_impl__impl0__lemma_new_entry_mb0_bits_are_zero`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `lemma_new_entry_mb0_bits_are_zero`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 448 — `NR__impl_u__l2_impl__impl0__lemma_zero_entry_facts`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `lemma_zero_entry_facts`
- **Verification:** :x: Verification failed (`verification results:: 21 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `self.entry == 0,`
- **Ground truth requires:** `self.entry & MASK_NEG_DIRTY_ACCESS == 0,
            self.layer@ <= 3,`
- **Generated ensures:** `self@ == GPDE::Invalid,`
- **Ground truth ensures:** `self@ is Invalid,
            self.all_mb0_bits_are_zero(),`

### Case 449 — `NR__impl_u__l2_impl__impl0__new_entry`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl.impl0
- **Target function:** `new_entry`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 450 — `NR__impl_u__l2_impl__lemma_aligned_addr_mask_facts`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl
- **Target function:** `lemma_aligned_addr_mask_facts`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `addr <= MAX_PHYADDR,
        aligned(addr as nat, PAGE_SIZE as nat),`
- **Ground truth requires:** `addr % 0x40000000usize == 0,
                    32 <= max_width;
        }
    };
    assert(aligned(addr as nat, L2_ENTRY_SIZE as nat) ==> (addr & M`
- **Generated ensures:** `addr & MASK_DIR_ADDR == addr,
        addr & MASK_ADDR == addr,`
- **Ground truth ensures:** `aligned(addr as nat, L1_ENTRY_SIZE as nat) ==> (addr & MASK_L1_PG_ADDR == addr & MASK_ADDR),
        aligned(addr as nat, L2_ENTRY_SIZE as nat) ==> (a`

### Case 451 — `NR__impl_u__l2_impl__lemma_bitvector_facts`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl
- **Target function:** `lemma_bitvector_facts`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors". The postconditions aren't being satisfied because we need to use `assert_by_compute` or similar to establish bitvector facts. Let me fix this:`)
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 452 — `NR__impl_u__l2_impl__lemma_bitvector_facts_simple`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl
- **Target function:** `lemma_bitvector_facts_simple`
- **Verification:** :x: Verification failed (`verification results:: 2 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated ensures:** `MASK_DIRTY_ACCESS == 0x60,
        MASK_NEG_DIRTY_ACCESS == !0x60usize,
        MASK_DIRTY_ACCESS & MASK_NEG_DIRTY_ACCESS == 0,
        MASK_DIRTY_ACC`
- **Ground truth ensures:** `bit!(0usize) == 1,
        0 & MASK_NEG_DIRTY_ACCESS == 0,
        1usize << 0 == 1,
        0usize & 1 == 0,`

### Case 453 — `NR__impl_u__l2_impl__lemma_page_aligned_implies_mask_dir_addr_is_identity`

- **Project:** NRKernel
- **Module:** impl_u.l2_impl
- **Target function:** `lemma_page_aligned_implies_mask_dir_addr_is_identity`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `aligned(addr as nat, PAGE_SIZE as nat),
        addr <= MAX_PHYADDR,`
- **Ground truth requires:** `(none)`
- **Generated ensures:** `addr & MASK_DIR_ADDR == addr,`
- **Ground truth ensures:** `forall|addr: usize| addr <= MAX_PHYADDR && #[trigger] aligned(addr as nat, PAGE_SIZE as nat) ==> addr & MASK_DIR_ADDR == addr,`

### Case 454 — `NR__impl_u__os_refinement__bounds_applied_mappings`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `bounds_applied_mappings`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 455 — `NR__impl_u__os_refinement__extra_mappings_inserted`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_inserted`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 456 — `NR__impl_u__os_refinement__extra_mappings_preserved`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_preserved`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 457 — `NR__impl_u__os_refinement__extra_mappings_preserved_effective_mapping_inserted`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_preserved_effective_mapping_inserted`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 458 — `NR__impl_u__os_refinement__extra_mappings_preserved_effective_mapping_removed`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_preserved_effective_mapping_removed`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 459 — `NR__impl_u__os_refinement__extra_mappings_removed`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_removed`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 460 — `NR__impl_u__os_refinement__extra_mappings_submap`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `extra_mappings_submap`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 461 — `NR__impl_u__os_refinement__interp_vmem_subrange`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `interp_vmem_subrange`
- **Verification:** :x: Verification failed (`verification results:: 5 verified, 1 errors`)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `s.applied_mappings().contains_pair(base, pte),
        no_overlaps(s.applied_mappings()),
        base <= vaddr,
        vaddr + size <= base + pte.fr`
- **Ground truth requires:** `no_overlaps(s.applied_mappings()),
        s.applied_mappings().dom().contains(base),
        s.applied_mappings()[base] == pte,
        base <= vaddr`
- **Generated ensures:** `s.interp_vmem(c).subrange(vaddr, vaddr + size) =~= s.mmu@.phys_mem.subrange(pte.frame.base + (vaddr - base), pte.frame.base + (vaddr - base) + size),`
- **Ground truth ensures:** `(`

### Case 462 — `NR__impl_u__os_refinement__interp_vmem_update_range`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `interp_vmem_update_range`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `no_overlaps(s.applied_mappings()),
        no_overlaps_pmem(s.applied_mappings()),
        bounds(c, s.applied_mappings()),
        s.applied_mappings`
- **Ground truth requires:** `no_overlaps(s.applied_mappings()),
        no_overlaps_pmem(s.applied_mappings()),
        bounds(c, s.applied_mappings()),
        s.mmu@.phys_mem.le`
- **Generated ensures:** `forall|i: int| 0 <= i < vaddr ==> s.interp_vmem(c)[i] == s.interp_vmem(c)[i],`
- **Ground truth ensures:** `(`

### Case 463 — `NR__impl_u__os_refinement__lemma_effective_mappings_unaffected_if_thread_state_constant`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `lemma_effective_mappings_unaffected_if_thread_state_constant`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 464 — `NR__impl_u__os_refinement__lemma_inflight_unmap_vaddr_equals_hl_unmap`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `lemma_inflight_unmap_vaddr_equals_hl_unmap`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 465 — `NR__impl_u__os_refinement__lemma_inflight_vaddr_implies_hl_unmap_or_map`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `lemma_inflight_vaddr_implies_hl_unmap_or_map`
- **Verification:** :x: Verification failed (`verification results:: 11 verified, 1 errors`)
- **Status:** :x: Different — ensures clause differs
- **Generated requires:** `s.inv_basic(c),`
- **Ground truth requires:** `s.inv_basic(c),`
- **Generated ensures:** `forall|v_addr| s.inflight_vaddr().contains(v_addr)
            ==> exists|thread_state|`
- **Ground truth ensures:** `forall|v_addr| s.inflight_unmap_vaddr().contains(v_addr)
            ==> exists|thread_state|`

### Case 466 — `NR__impl_u__os_refinement__lemma_map_soundness_equality`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `lemma_map_soundness_equality`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 467 — `NR__impl_u__os_refinement__lemma_unmap_soundness_equality`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `lemma_unmap_soundness_equality`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 468 — `NR__impl_u__os_refinement__monotonic_candidate_mapping_overlaps_existing_vmem`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `monotonic_candidate_mapping_overlaps_existing_vmem`
- **Verification:** :white_check_mark: Verified (0 errors)
- **Status:** :x: Different — both requires and ensures differ
- **Generated requires:** `forall|b: nat| mappings1.contains_key(b) ==> mappings2.contains_key(b) && mappings2[b] == mappings1[b],
        candidate_mapping_overlaps_existing_vm`
- **Ground truth requires:** `mappings1.submap_of(mappings2)`
- **Generated ensures:** `candidate_mapping_overlaps_existing_vmem(mappings2, base, pte),`
- **Ground truth ensures:** `candidate_mapping_overlaps_existing_vmem(mappings1, base, pte)
        ==> candidate_mapping_overlaps_existing_vmem(mappings2, base, pte)`

### Case 469 — `NR__impl_u__os_refinement__next_step_refines_hl_next_step`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `next_step_refines_hl_next_step`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 470 — `NR__impl_u__os_refinement__no_overlaps_applied_mappings`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `no_overlaps_applied_mappings`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 471 — `NR__impl_u__os_refinement__no_overlaps_interp_pt_mem`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `no_overlaps_interp_pt_mem`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 472 — `NR__impl_u__os_refinement__no_overlaps_pmem_applied_mappings`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `no_overlaps_pmem_applied_mappings`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 473 — `NR__impl_u__os_refinement__os_init_refines_hl_init`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `os_init_refines_hl_init`
- **Verification:** :x: Verification failed (`verification results:: 8 verified, 1 errors`)
- **Status:** :white_check_mark: Semantically Equivalent
- **Requires:** `crate::os::init(c, s),`
- **Ensures:** `crate::hlspec::init(c.interp(), s.interp(c)),`

### Case 474 — `NR__impl_u__os_refinement__os_next_refines_hl_next`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `os_next_refines_hl_next`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 475 — `NR__impl_u__os_refinement__relevant_mem_preserved`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `relevant_mem_preserved`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 476 — `NR__impl_u__os_refinement__step_MapEnd_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_MapEnd_refines`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 477 — `NR__impl_u__os_refinement__step_MapOpChange_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_MapOpChange_refines`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 478 — `NR__impl_u__os_refinement__step_MapStart_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_MapStart_refines`
- **Verification:** :warning: Not verified
- **Status:** :warning: No code extracted from generation
- **Note:** The generated output did not contain extractable Verus code blocks. The model may have produced only natural language analysis without a complete code solution.

### Case 479 — `NR__impl_u__os_refinement__step_MemOp_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_MemOp_refines`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 480 — `NR__impl_u__os_refinement__step_UnmapEnd_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_UnmapEnd_refines`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 481 — `NR__impl_u__os_refinement__step_UnmapOpChange_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_UnmapOpChange_refines`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 482 — `NR__impl_u__os_refinement__step_UnmapStart_refines`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `step_UnmapStart_refines`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 483 — `NR__impl_u__os_refinement__unmap_vaddr_set_le_extra_mappings_dom`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `unmap_vaddr_set_le_extra_mappings_dom`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 484 — `NR__impl_u__os_refinement__vaddr_distinct`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `vaddr_distinct`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 485 — `NR__impl_u__os_refinement__vaddr_mapping_is_being_modified_from_vaddr_unmap`

- **Project:** NRKernel
- **Module:** impl_u.os_refinement
- **Target function:** `vaddr_mapping_is_being_modified_from_vaddr_unmap`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 486 — `NR__impl_u__wrapped_token__impl1__lemma_interps_match`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl1
- **Target function:** `lemma_interps_match`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 487 — `NR__impl_u__wrapped_token__impl1__lemma_interps_match_aux1`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl1
- **Target function:** `lemma_interps_match_aux1`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 488 — `NR__impl_u__wrapped_token__impl1__lemma_interps_match_aux2`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl1
- **Target function:** `lemma_interps_match_aux2`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 489 — `NR__impl_u__wrapped_token__impl2__allocate`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `allocate`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 490 — `NR__impl_u__wrapped_token__impl2__finish_map_and_release_lock`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `finish_map_and_release_lock`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 491 — `NR__impl_u__wrapped_token__impl2__lemma_regions_derived_from_view_after_write`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `lemma_regions_derived_from_view_after_write`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 492 — `NR__impl_u__wrapped_token__impl2__new`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `new`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 493 — `NR__impl_u__wrapped_token__impl2__read`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `read`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 494 — `NR__impl_u__wrapped_token__impl2__register_failed_map`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `register_failed_map`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 495 — `NR__impl_u__wrapped_token__impl2__write_change`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `write_change`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 496 — `NR__impl_u__wrapped_token__impl2__write_stutter`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl2
- **Target function:** `write_stutter`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 497 — `NR__impl_u__wrapped_token__impl3__deallocate`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl3
- **Target function:** `deallocate`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 498 — `NR__impl_u__wrapped_token__impl3__finish_unmap_and_release_lock`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl3
- **Target function:** `finish_unmap_and_release_lock`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 499 — `NR__impl_u__wrapped_token__impl3__lemma_regions_derived_from_view_after_write`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl3
- **Target function:** `lemma_regions_derived_from_view_after_write`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 500 — `NR__impl_u__wrapped_token__impl3__new`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token.impl3
- **Target function:** `new`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 501 — `NR__impl_u__wrapped_token__impl3_read`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token
- **Target function:** `impl3_read`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 502 — `NR__impl_u__wrapped_token__impl3_write_change`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token
- **Target function:** `impl3_write_change`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 503 — `NR__impl_u__wrapped_token__impl3_write_stutter`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token
- **Target function:** `impl3_write_stutter`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 504 — `NR__impl_u__wrapped_token__start_map_and_acquire_lock`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token
- **Target function:** `start_map_and_acquire_lock`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 505 — `NR__impl_u__wrapped_token__start_unmap_and_acquire_lock`

- **Project:** NRKernel
- **Module:** impl_u.wrapped_token
- **Target function:** `start_unmap_and_acquire_lock`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 506 — `NR__spec_t__hlproof__insert_map_preserves_unique`

- **Project:** NRKernel
- **Module:** spec_t.hlproof
- **Target function:** `insert_map_preserves_unique`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 507 — `NR__spec_t__hlproof__insert_non_map_preserves_unique`

- **Project:** NRKernel
- **Module:** spec_t.hlproof
- **Target function:** `insert_non_map_preserves_unique`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 508 — `NR__spec_t__hlproof__map_end_preserves_inv`

- **Project:** NRKernel
- **Module:** spec_t.hlproof
- **Target function:** `map_end_preserves_inv`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 509 — `NR__spec_t__hlproof__map_start_preserves_inv`

- **Project:** NRKernel
- **Module:** spec_t.hlproof
- **Target function:** `map_start_preserves_inv`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 510 — `NR__spec_t__hlproof__unmap_start_preserves_inv`

- **Project:** NRKernel
- **Module:** spec_t.hlproof
- **Target function:** `unmap_start_preserves_inv`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 511 — `NR__spec_t__hlspec__next_step_preserves_inv`

- **Project:** NRKernel
- **Module:** spec_t.hlspec
- **Target function:** `next_step_preserves_inv`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 512 — `NR__spec_t__mmu__defs__MAX_PHYADDR`

- **Project:** NRKernel
- **Module:** spec_t.mmu.defs
- **Target function:** `MAX_PHYADDR`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

### Case 513 — `NR__spec_t__mmu__defs__x86_arch_exec`

- **Project:** NRKernel
- **Module:** spec_t.mmu.defs
- **Target function:** `x86_arch_exec`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 514 — `NR__spec_t__mmu__defs__x86_arch_spec_upper_bound`

- **Project:** NRKernel
- **Module:** spec_t.mmu.defs
- **Target function:** `x86_arch_spec_upper_bound`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 515 — `NR__spec_t__mmu__pt_mem__impl0__lemma_pt_walk`

- **Project:** NRKernel
- **Module:** spec_t.mmu.pt_mem.impl0
- **Target function:** `lemma_pt_walk`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 516 — `NR__spec_t__mmu__pt_mem__impl0__lemma_pt_walk_agrees_in_frame`

- **Project:** NRKernel
- **Module:** spec_t.mmu.pt_mem.impl0
- **Target function:** `lemma_pt_walk_agrees_in_frame`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 517 — `NR__spec_t__mmu__pt_mem__impl0__lemma_write_seq`

- **Project:** NRKernel
- **Module:** spec_t.mmu.pt_mem.impl0
- **Target function:** `lemma_write_seq`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 518 — `NR__spec_t__mmu__pt_mem__impl0__lemma_write_seq_first`

- **Project:** NRKernel
- **Module:** spec_t.mmu.pt_mem.impl0
- **Target function:** `lemma_write_seq_first`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** `timeout`

### Case 519 — `NR__spec_t__mmu__pt_mem__impl0__lemma_write_seq_idle`

- **Project:** NRKernel
- **Module:** spec_t.mmu.pt_mem.impl0
- **Target function:** `lemma_write_seq_idle`
- **Status:** :no_entry: Empty (no generation produced)
- **Error:** N/A

---

## Summary

### Overall Statistics

| Category | Count | % of Total | % of Comparable |
|----------|-------|------------|-----------------|
| Total cases | 520 | 100% | — |
| **Semantically Equivalent** | **108** | **20.8%** | **29.8%** |
| Different (ensures only) | 80 | 15.4% | 22.0% |
| Different (requires only) | 38 | 7.3% | 10.5% |
| Different (both) | 135 | 26.0% | 37.2% |
| Different (missing requires) | 2 | 0.4% | 0.6% |
| **Total Semantically Different** | **255** | **49.0%** | **70.2%** |
| No code extracted | 30 | 5.8% | — |
| Target fn not found in gen | 13 | 2.5% | — |
| Empty (no generation) | 114 | 21.9% | — |

_"% of Comparable" is computed over the 363 cases where both sides' target function was successfully located._

### Verification Results

| Category | Verified OK | Verification Failed | Not Verified |
|----------|-------------|---------------------|--------------|
| Semantically Equivalent (108) | 28 (26%) | 80 (74%) | 0 (0%) |
| Different (ensures) (80) | 27 (34%) | 53 (66%) | 0 (0%) |
| Different (requires) (38) | 5 (13%) | 33 (87%) | 0 (0%) |
| Different (both) (135) | 34 (25%) | 101 (75%) | 0 (0%) |
| Fn not found in gen (13) | 5 (38%) | 8 (62%) | 0 (0%) |
| No code extracted (30) | 0 (0%) | 7 (23%) | 23 (77%) |

### By Project

| Project | Total | Equivalent | Different | No Code | Fn Not Found | Empty | Equiv Rate (comparable) |
|---------|-------|-----------|-----------|---------|--------------|-------|------------------------|
| Anvil | 104 | 44 | 55 | 0 | 3 | 2 | 44.4% |
| Anvil-Advanced | 63 | 1 | 3 | 7 | 1 | 51 | 25.0% |
| IronKV | 118 | 26 | 75 | 3 | 8 | 6 | 25.7% |
| Memory-Allocator | 89 | 21 | 63 | 3 | 1 | 1 | 25.0% |
| NRKernel | 117 | 10 | 38 | 16 | 0 | 53 | 20.8% |
| Node-Replication | 29 | 6 | 21 | 1 | 0 | 1 | 22.2% |

### Key Findings

1. **Semantic equivalence rate:** 108 out of 363 comparable cases (29.8%) produced specs that are **semantically equivalent** to the ground truth. This is the rate when we strip natural language reasoning and compare only the `requires`/`ensures` clauses.
2. **Most common difference type:** The most frequent type of semantic difference was **both requires and ensures differing** (135 cases, 37.2%), followed by **only ensures differing** (80 cases, 22.0%) and **only requires differing** (38 cases, 10.5%).
3. **Generation failures:** 114 cases (21.9%) produced no output (mostly due to timeout). An additional 30 cases generated only natural language text without extractable code.
4. **Verification vs semantic correctness:** Of the 108 semantically equivalent specs, only 28 (26%) passed verification. This suggests that even when the spec (requires/ensures) is correct, the **proof body** often differs or is incomplete.
5. **Best performing project:** **Anvil** had the highest number of semantically equivalent specs (44 out of 299 comparable), while **Anvil-Advanced** had the lowest (only 1 equivalent).
6. **Natural language is not the issue:** The previous analysis flagged 99.5% of generated outputs as contaminated with natural language. After stripping NL and extracting just the code, 29.8% of specs are semantically correct — showing that the model's reasoning text is separate from its code output.
