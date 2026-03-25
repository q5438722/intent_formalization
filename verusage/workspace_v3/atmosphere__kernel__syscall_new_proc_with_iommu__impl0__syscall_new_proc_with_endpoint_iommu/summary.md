# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__syscall_new_proc_with_iommu__impl0__syscall_new_proc_with_endpoint_iommu/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: preconditions_unsatisfiable → `syscall_new_proc_with_endpoint_iommu`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the combined preconditions are contradictory, making the spec vacuously correct and the function uncallable.

### φ2: va_range_len_forced_zero → `syscall_new_proc_with_endpoint_iommu`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the well-formedness invariant forces zero-length ranges, meaning no pages can ever be mapped by this syscall.

### φ3: no_valid_endpoint_index → `syscall_new_proc_with_endpoint_iommu`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, MAX_NUM_ENDPOINT_DESCRIPTORS is zero (or the bound is contradictory), meaning no valid endpoint index exists and the syscall can never be invoked.

### φ4: va_range_len_under_two → `syscall_new_proc_with_endpoint_iommu`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the spec constrains va_range to at most 1 page, severely limiting the syscall to trivial cases and preventing meaningful address-space sharing.

### φ5: wf_forces_overflow_conflict → `syscall_new_proc_with_endpoint_iommu`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, va_range.wf() alone implies the overflow guard can never be satisfied, making the preconditions of the syscall mutually contradictory.

