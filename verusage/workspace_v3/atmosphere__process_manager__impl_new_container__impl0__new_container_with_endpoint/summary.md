# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_new_container__impl0__new_container_with_endpoint/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: container_ptr_in_proc_dom → `new_container_with_endpoint`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the container page_ptr_1 appeared in proc_dom, it would violate domain separation between containers and processes

### φ2: parent_children_len_unchanged → `new_container_with_endpoint`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the parent container's children length did not increase after adding a child, the child was silently dropped

### φ3: new_container_in_own_subtree → `new_container_with_endpoint`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A newly created leaf container appearing in its own subtree would break the container tree acyclicity invariant

### φ4: new_thread_endpoint_none → `new_container_with_endpoint`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the new thread's first endpoint descriptor were None, the child container would be created without access to the shared endpoint

### φ5: old_proc_gets_new_pcid → `new_container_with_endpoint`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If an existing process acquired the new pcid, it would violate pcid uniqueness and corrupt address space isolation

