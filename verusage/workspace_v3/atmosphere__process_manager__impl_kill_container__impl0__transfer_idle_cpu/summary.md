# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__process_manager__impl_kill_container__impl0__transfer_idle_cpu/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: preconditions_unsatisfiable → `transfer_idle_cpu`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the preconditions are contradictory and transfer_idle_cpu can never be called.

### φ2: self_parent_loop → `transfer_idle_cpu`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, a container can be its own parent, making the CPU transfer a no-op and breaking the tree structure.

### φ3: parent_not_in_domain → `transfer_idle_cpu`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the parent container is not in the domain, so inserting the CPU into the parent would access invalid memory.

### φ4: depth_exceeds_container_count → `transfer_idle_cpu`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, tree depth exceeds the number of containers, indicating a malformed container tree invariant.

### φ5: cpu_owner_mismatch → `transfer_idle_cpu`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, cpu_list and container owned_cpus are inconsistent—the CPU is listed under a container that the cpu_list disagrees with.

