# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__syscall_io_mmap__impl0__syscall_mmap_to_iommu_table/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: usize2pa_always_zero → `syscall_io_mmap`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If usize2pa always yields 0, every IO-mapped page in syscall_io_mmap would resolve to physical address 0, making IO mapping useless.

### φ2: mem_valid_implies_zero → `syscall_io_mmap`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If only address 0 satisfies MEM_valid, then usize2pa's postcondition MEM_valid(ret) forces all physical addresses to zero, breaking syscall_io_mmap's allocated page addresses.

### φ3: zero_entry_appears_present → `syscall_io_mmap`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a zero raw value decodes to a present page entry, PageEntry::is_empty would be unsound and free IO-space VA range checks in syscall_io_mmap could misidentify empty slots as occupied.

### φ4: perm_never_writable → `syscall_io_mmap`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If write permission is never set regardless of input, IO-mapped pages from syscall_io_mmap would always be read-only, defeating the purpose of device IO mapping.

### φ5: free_list_len_always_zero → `syscall_io_mmap`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a well-formed non-empty free list reports length 0, syscall_io_mmap's quota assertion `free_pages_4k.len() >= va_range.len * 4` would fail even when pages are available, blocking all IO mappings.

