# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_next_delegate/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: delegate_output_nonempty → `next_delegate`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** next_delegate mandates out == empty; if non-empty output is entailed the spec is inconsistent

### φ2: authorized_no_increment → `next_delegate`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When source is authorized the counter must increment by 1; staying the same means delegation bookkeeping is lost

### φ3: unauthorized_changes_hashtable → `next_delegate`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An unauthorized source must not alter the hashtable; entailment here means the spec allows unauthenticated mutation

### φ4: none_value_invalid → `valid_optional_value`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** None should be a valid optional value; declaring it invalid would reject legitimate absent-value messages

### φ5: update_domain_drops_new_key → `bulk_update_domain`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A key present in the update map and inside the delegated range must appear in the result; exclusion means delegated data is silently lost

