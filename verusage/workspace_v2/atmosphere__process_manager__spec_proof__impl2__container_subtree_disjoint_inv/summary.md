# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__spec_proof__impl2__container_subtree_disjoint_inv.rs`
**Date:** 2026-03-24T08:13:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The `get_thread` property correctly follows from the `wf()` precondition — cross-domain membership of thread owners is an intended invariant. The `get_proc` property's `owned_threads.wf()` is directly entailed by the `process_fields_wf()` precondition regardless of full `wf()` status, so the external_body is not leaking extra guarantees beyond what the open spec already provides.

## All Candidates

### φ1: get_thread_ensures_without_wf
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The `get_thread` external_body ensures `proc_dom.contains(ret.owning_proc)` and `container_dom.contains(ret.owning_container)` but neither `threads_cpu_wf` nor any visible predicate in `wf()` actually constrains these cross-domain membership properties — they are asserted purely by the trusted external_body
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `get_thread` external_body requires `self.wf()` as a precondition and ensures cross-domain membership as a postcondition. This is the intended design: `wf()` is the top-level well-formedness predicate that subsumes all sub-predicates, and the external_body serves as a convenience lemma extracting useful consequences. The cross-domain properties are desirable invariants that should hold under `wf()`.

### φ2: owned_threads_not_subset_thread_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Container's `owned_threads` is a Ghost<Set<ThreadPtr>> with no predicate requiring it to be a subset of `thread_dom` — a container could claim ownership of nonexistent threads

### φ3: scheduler_entries_in_thread_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `scheduler` StaticLinkedList holds ThreadPtrs but `container_fields_wf` only requires `scheduler.wf()` and `scheduler.unique()` — no predicate ensures scheduler entries are valid thread pointers in `thread_dom`

### φ4: owned_endpoints_not_subset_endpoint_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Container's `owned_endpoints` is a Ghost<Set<EndpointPtr>> with no predicate constraining it to be a subset of `endpoint_dom` — a container could claim ownership of endpoints that don't exist

### φ5: get_proc_owned_threads_wf_without_process_fields
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The `get_proc` external_body unconditionally ensures `ret.owned_threads.wf()` regardless of whether `wf()` holds — this is stronger than the spec-level `process_fields_wf` which derives the same conclusion, suggesting the external_body leaks extra guarantees beyond what the open specs establish
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `get_proc` external_body requires `proc_perms_wf()` and `process_fields_wf()` — and `process_fields_wf` explicitly states that for all procs in dom, `owned_threads.wf()` holds. The external_body's unconditional `ret.owned_threads.wf()` ensure simply mirrors what `process_fields_wf` already guarantees at the spec level. The `!pm.wf()` precondition is irrelevant since the needed sub-predicates are directly required.

