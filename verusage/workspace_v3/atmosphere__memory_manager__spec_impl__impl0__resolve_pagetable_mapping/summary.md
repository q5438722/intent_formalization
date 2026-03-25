# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__memory_manager__spec_impl__impl0__resolve_pagetable_mapping/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_returns_none → `resolve_pagetable_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean no valid VA ever resolves to a mapping, making the page table useless

### φ2: always_returns_some → `resolve_pagetable_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean every valid VA is always mapped, indicating the spec cannot distinguish mapped from unmapped addresses

### φ3: mapped_addr_always_zero → `resolve_pagetable_mapping`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean all resolved mappings point to physical address zero, collapsing all virtual memory to a single frame

### φ4: write_perm_always_false → `resolve_pagetable_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean no mapping is ever writable, so the spec cannot express writable memory regions

### φ5: result_ignores_pcid → `resolve_pagetable_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean different PCIDs always resolve the same VA identically, destroying address-space isolation between processes

