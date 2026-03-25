# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_kill_proc__impl0__kill_process_none_root/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: parent_also_removed → `kill_process_none_root`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Killing a leaf process should never remove its parent from the process domain

### φ2: parent_children_fully_cleared → `kill_process_none_root`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Parent should only lose the killed child, not have all its children cleared

### φ3: container_dom_shrinks → `kill_process_none_root`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Killing a process must never remove any container from the container domain

### φ4: endpoint_dom_shrinks → `kill_process_none_root`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Killing a threadless childless process should never destroy any endpoint

### φ5: returned_page_differs_from_proc → `kill_process_none_root`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The freed page should be the process's own backing page, not some unrelated page

