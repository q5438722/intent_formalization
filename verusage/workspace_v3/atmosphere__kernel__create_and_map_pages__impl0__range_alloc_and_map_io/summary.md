# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_map_pages__impl0__range_alloc_and_map_io/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pa_always_zero → `range_alloc_and_map_io`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If PA masking zeroes all addresses, range_alloc_and_map_io would map every IO page to physical address 0

### φ2: nonzero_produces_empty_entry → `range_alloc_and_map_io`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If non-zero usize values decode to empty page entries, IO page table entries created during mapping would all appear unmapped

### φ3: present_implies_write → `range_alloc_and_map_io`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If every present page entry is writable, read-only IO mappings are impossible and memory protection for device MMIO regions is broken

### φ4: pa_preserves_all_bits → `range_alloc_and_map_io`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If PA masking is identity, permission and flag bits leak into physical addresses used for IO mappings, corrupting device memory targets

### φ5: present_implies_execute_disable → `range_alloc_and_map_io`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If all present entries have execute-disable set, executable device MMIO regions mapped by range_alloc_and_map_io become non-executable

