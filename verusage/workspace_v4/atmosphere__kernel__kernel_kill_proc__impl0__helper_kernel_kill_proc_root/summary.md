# Spec Consistency Report

**Source:** `verusage/workspace_v4/atmosphere__kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_root/original.rs`
**Date:** 2026-03-26T01:06:10Z

## Stats

- Candidates generated: 14
- Entailed (verified): 10
- Tautologies filtered: 0
- True positives: 9
- False positives: 1

## Summary

Of the 10 candidate properties, 9 are true positives identifying genuine spec incompleteness in `helper_kernel_kill_proc_root`. The issues cluster into four categories: (1) **resource reclamation gaps**—the spec never reflects freed 4k pages returning to the free pool or PCIDs returning to the available pool (φ page_not_freed, freed_page_not_in_ensures, pcid_not_recycled, pcid_not_freed); (2) **missing frame conditions**—the spec fails to constrain 2m/1g page state, page_mappings for unrelated pages, and surviving processes' subtree_sets across the call (φ spurious_2m_1g_modification, page_mapping_unrelated_modified, page_mapping_preservation_not_ensured, subtree_set_unconstrained); (3) **dangling references**—container_dom is preserved verbatim while the root process is removed, creating potential stale references (φ dangling_container_ref). The sole false positive is φ vacuous_subtree_update, which correctly observes that a root process has no ancestors, making the ancestor-subtree postcondition vacuously true—this is expected behavior, not a spec defect.

## True Positives

### page_not_freed
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Killing a process must deallocate its owned 4k pages back to the free pool. The spec's ensures clause never constrains `free_pages_4k()` to grow, so callers cannot reason about memory reclamation. This is a genuine exec-level incompleteness: the implementation frees pages but the spec doesn't promise it, leading to unbounded memory leakage from the caller's perspective.

### pcid_not_recycled
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** PCIDs are a finite hardware resource (typically 4096 values). The spec never states that the killed process's PCID is returned to an available pool. Callers creating new processes cannot prove PCID availability, leading to eventual exhaustion. This is exec-level incompleteness—the body likely recycles the PCID but the spec doesn't reflect it.

### spurious_2m_1g_modification
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** A root-process kill operating only on 4k pages should leave 2m/1g mapped/allocated sets unchanged. The spec provides no frame condition for `mapped_pages_2m()`, `mapped_pages_1g()`, `allocated_pages_2m()`, or `allocated_pages_1g()`, allowing a conforming implementation to corrupt 2m/1g page state. This is a missing frame condition for exec-visible state.

### dangling_container_ref
- **Confidence:** medium
- **Filter:** incompleteness
- **Reasoning:** The spec explicitly preserves `container_dom` verbatim while removing `proc_ptr` from `proc_dom`. If any container internally references the killed root process, this creates a dangling reference the spec does not address. The spec should either update the container's internal reference or document the contract for how containers handle root-process death.

### page_mapping_unrelated_modified
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** The spec lacks a frame condition preserving `page_mappings(p)` for pages unrelated to the killed process. A conforming implementation could silently add mappings to other processes' pages, breaking memory isolation. This is a critical exec-level incompleteness—callers of unrelated pages cannot rely on their mappings being stable across a process kill.

### subtree_set_unconstrained
- **Confidence:** medium
- **Filter:** incompleteness
- **Reasoning:** If `processes_fields_unchanged` (which is opaque/closed) does not cover `subtree_set`, surviving processes' subtree_sets are unconstrained by the spec. Since this φ passed the tautology filter, `subtree_set` is evidently not protected, allowing silent corruption of the process tree for surviving processes. This is an exec-level incompleteness in the frame condition.

### freed_page_not_in_ensures
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** This reinforces φ page_not_freed from a body-aware perspective. The implementation frees pages (moving them from allocated to free), but the ensures clause permits a post-state where `free_pages_4k()` and `allocated_pages_4k()` are completely unchanged. Callers cannot prove that a subsequent allocation will succeed, making the spec insufficient for reasoning about resource availability.

### pcid_not_freed
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** This reinforces φ pcid_not_recycled from a body-aware perspective. The implementation recycles the PCID but the ensures clause permits available_pcids to remain unchanged. Callers creating new processes have no spec-level guarantee that PCIDs freed by kills are reusable, making PCID availability unprovable.

### page_mapping_preservation_not_ensured
- **Confidence:** high
- **Filter:** incompleteness
- **Reasoning:** Complementary to φ page_mapping_unrelated_modified: the spec also allows silently *removing* existing mappings for unrelated pages. Without a frame condition like `forall|p| unrelated(p) ==> new_alloc.page_mappings(p) == old_alloc.page_mappings(p)`, callers cannot rely on mapping stability, breaking isolation guarantees in both directions.

## All Candidates

### φ1: vacuous_subtree_update → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** A vacuously true postcondition lets a conforming implementation leave stale proc_ptr entries in any process's subtree_set without violating the spec
- **Verdict:** FALSE_POSITIVE (high)

### φ2: page_not_freed → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** Leaking one 4k page per process kill leads to unbounded memory loss over time since the spec never requires the freed page to re-enter the free pool
- **Verdict:** TRUE_POSITIVE (high)

### φ3: pcid_not_recycled → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** Permanently consuming PCIDs without recycling leads to PCID exhaustion since hardware PCID space is finite (typically 12 bits)
- **Verdict:** TRUE_POSITIVE (high)

### φ4: spurious_2m_1g_modification → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** Allowing spurious 2m/1g page state changes can corrupt memory isolation since the spec never constrains these sets to remain unchanged across root process kills
- **Verdict:** TRUE_POSITIVE (high)

### φ5: dangling_container_ref → `helper_kernel_kill_proc_root`
- **Type:** boundary | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** A container holding a dangling reference to a killed root process can cause use-after-free or stale lookups when the container later tries to access its root process
- **Verdict:** TRUE_POSITIVE (medium)

### φ6: page_mapping_unrelated_modified → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** Silently adding or removing page mappings for other processes breaks page-level isolation guarantees that callers depend on.
- **Verdict:** TRUE_POSITIVE (high)

### φ7: subtree_set_unconstrained → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** spec_only
- **Entailed:** ✅
- **Why flagged:** If subtree_set is not protected by processes_fields_unchanged, the process tree invariant can be silently corrupted for surviving processes.
- **Verdict:** TRUE_POSITIVE (medium)

### φ8: freed_page_not_in_ensures → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** Callers cannot prove a subsequent 4k allocation will succeed because the spec never reflects the freed page returning to the free pool.
- **Verdict:** TRUE_POSITIVE (high)

### φ9: pcid_not_freed → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** Pcids are a finite resource; callers creating new processes cannot reason about pcid availability if the spec never reflects freed pcids returning to the available pool.
- **Verdict:** TRUE_POSITIVE (high)

### φ10: page_mapping_preservation_not_ensured → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ✅
- **Why flagged:** Other processes' pages may become incorrectly unmapped because the spec does not guarantee mapping preservation when killing a process with an empty page table.
- **Verdict:** TRUE_POSITIVE (high)

### φ11: large_page_sets_remain_empty → `helper_kernel_kill_proc_root`
- **Type:** boundary | **Source:** body_aware
- **Entailed:** ❌
- **Why flagged:** The ensures clause never constrains large-page sets, so callers cannot confirm the system operates in a 4k-only regime or prove absence of 2m/1g TLB entries.

### φ12: container_maps_domain_consistent → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ❌
- **Why flagged:** The ensures preserves container_dom equality but never exposes its link to the page allocator's container maps, blocking container-level page accounting by callers.

### φ13: mapping_wf_preserved → `helper_kernel_kill_proc_root`
- **Type:** behavioral | **Source:** body_aware
- **Entailed:** ❌
- **Why flagged:** The closed wf() postcondition cannot be decomposed by callers, so mapping consistency between page tables and page allocator is not individually derivable.

### φ14: page_closure_accounting → `helper_kernel_kill_proc_root`
- **Type:** logical | **Source:** body_aware
- **Entailed:** ❌
- **Why flagged:** The fundamental page-accounting equation (no leaked or double-counted pages) is hidden behind closed specs, making global leak-freedom unprovable by callers across create/kill sequences.

