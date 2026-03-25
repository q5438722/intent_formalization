# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_root/original.rs`
**Date:** 2026-03-25 04:03:48
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The φ `stale_child_reference` identifies a genuine spec gap: `helper_kernel_kill_proc_root` preserves all surviving processes' `children` lists unchanged while removing `proc_ptr` from the process domain. The precondition requires `depth == 0` and `children@ == Seq::empty()`, but there is no public precondition ensuring no other process lists `proc_ptr` as a child. The relevant invariant likely lives inside the opaque `internal_wf()` spec, meaning the public-facing specification cannot be independently verified to prevent dangling child references. This is a real weakness — the soundness of the operation depends entirely on the hidden `internal_wf` predicate, which is an `external_body` trusted assumption.

## True Positives (Spec Issues)

### stale_child_reference
- **Confidence:** high
- **Reasoning:** The `helper_kernel_kill_proc_root` postcondition ensures `self.get_proc(p_ptr).children == old(self).get_proc(p_ptr).children` for all surviving processes, meaning parent children lists are preserved verbatim. Combined with the precondition `depth == 0` (root process with no parent in the process tree), this might seem safe — but the spec does NOT require `depth == 0` to mean "no parent." The `Process` struct has an explicit `parent: Option<ProcPtr>` field, and a process with `depth == 0` could still appear in another process's `children` list if the internal invariant (`internal_wf`) doesn't rule it out. Since `internal_wf` is `external_body` and opaque, the public spec cannot guarantee that killing a depth-0 process won't leave a stale reference in some other process's children list.

## All Candidates

### φ1: collateral_proc_removal
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Killing one process should never remove a different process from the domain

### φ2: stale_child_reference
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Children-preservation ensures could leave a dangling reference to the killed process in a surviving process's children list
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `helper_kernel_kill_proc_root` postcondition ensures `self.get_proc(p_ptr).children == old(self).get_proc(p_ptr).children` for all surviving processes, meaning parent children lists are preserved verbatim. Combined with the precondition `depth == 0` (root process with no parent in the process tree), this might seem safe — but the spec does NOT require `depth == 0` to mean "no parent." The `Process` struct has an explicit `parent: Option<ProcPtr>` field, and a process with `depth == 0` could still appear in another process's `children` list if the internal invariant (`internal_wf`) doesn't rule it out. Since `internal_wf` is `external_body` and opaque, the public spec cannot guarantee that killing a depth-0 process won't leave a stale reference in some other process's children list.

### φ3: container_dom_loses_element
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Killing a root process should never cause a container to disappear from the container domain

### φ4: subtree_over_removal
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Removing proc_ptr from an ancestor's subtree_set should not accidentally remove any other process from that set

### φ5: thread_dom_gains_element
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Killing a process should never introduce a new thread into the thread domain

