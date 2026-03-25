# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__impl_kill_thread__impl0__kill_scheduled_thread.rs`
**Date:** 2026-03-24T07:51:18Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property that `owned_cpus` only contains in-range CPU indices is directly entailed by the reverse direction of `container_cpu_wf`, which explicitly includes `0 <= cpu_i < NUM_CPUS` in its consequent. This is a desirable, expected invariant — not a spec gap.

## All Candidates

### φ1: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The external_body `len()` ensures `l == self.value_list_len` unconditionally (not guarded by `self.wf()`), so `spec_len` which maps to `len` returns `value_list_len` even without well-formedness — but `spec_len` is defined as `self@.len()` which reads from `spec_seq`, creating a potential inconsistency between the two definitions when wf doesn't hold

### φ2: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Neither `container_perms_wf` nor `container_tree_wf` constrains `owned_threads` disjointness across containers — two distinct containers can claim the same thread via their unconstrained Ghost fields

### φ3: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `owned_endpoints` Ghost field on Container has no cross-container disjointness constraint in any well-formedness predicate, allowing two containers to simultaneously claim the same endpoint

### φ4: get_proc_leaks_container_dom_without_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** get_proc's external_body ensures `self.wf() ==> self.container_dom().contains(ret.owning_container)` guarded by wf, but the weaker preconditions (proc_perms_wf + process_fields_wf) may allow the SMT solver to derive the container_dom membership unconditionally from the implication when wf is false — the ensures should hold vacuously only if wf is genuinely needed

### φ5: container_cpu_wf_no_cpu_upper_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The reverse direction of `container_cpu_wf` (from container's owned_cpus to cpu_list) only triggers when `self.get_container(c_ptr).owned_cpus@.contains(cpu_i)` but the ensures clause requires `0 <= cpu_i < NUM_CPUS` — if the Ghost set contains an out-of-range CpuId, the spec may not catch it since the quantifier body starts from the trigger rather than bounding cpu_i first
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The reverse direction of `container_cpu_wf` explicitly ensures `0 <= cpu_i < NUM_CPUS` as part of its consequent — the quantifier body states that when `self.get_container(c_ptr).owned_cpus@.contains(cpu_i)` and `c_ptr` is in the container domain, then `0 <= cpu_i < NUM_CPUS` (among other things). This is exactly the intended design: owned_cpus must only contain valid CPU indices, and the biconditional between cpu_list and owned_cpus enforces this bound.

