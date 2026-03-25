# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_lemmas/vreplicaset_controller__proof__helper_lemmas__matching_pods_equal_to_matching_pod_entries_values.rs`
**Date:** 2026-03-24T02:49:41Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single verified candidate is a false positive. The monotonicity of label selector matching (if selector matches labels L1 and L1 ⊆ L2, then selector matches L2) is a correct, expected consequence of the subset-containment definition used in `LabelSelectorView::matches`. This reflects real Kubernetes behavior where adding labels to a pod never causes it to stop matching a selector it previously matched.

## All Candidates

### φ1: selector_none_equals_empty_some
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** None and Some(empty) both vacuously match all labels — if entailed, it reveals the spec cannot distinguish "no selector" from "empty selector", which may be unintended

### φ2: owned_match_implies_name_is_some
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** owned_selector_match_is checks namespace, owner_references, labels, deletion_timestamp, and kind — but never requires the pod's name to be Some, so this should NOT be entailed

### φ3: matching_pods_cardinality_equals_entries
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** matching_pod_entries maps keys to values so multiple keys can map to the same DynamicObjectView — the entries domain can be larger than the deduplicated values set

### φ4: different_vrs_disjoint_matching_pods
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** a pod's owner_references is a Seq that can contain multiple entries — a pod could have owner refs for both controllers, so different VRS do NOT imply disjoint matching pod sets

### φ5: selector_match_is_transitive_on_labels
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** while logically this looks like transitivity of subset containment (match_labels ⊆ l1 ⊆ l2), the forall-based spec may not give the SMT solver enough triggers to chain the reasoning — if it IS entailed, it confirms the spec is monotone in label superset which may over-match
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property. Label selector matching is intentionally monotone: if a selector matches a label set, it should match any superset. The spec defines matching as "all selector pairs are present in labels," which is naturally preserved when adding more labels. This is standard Kubernetes semantics.

