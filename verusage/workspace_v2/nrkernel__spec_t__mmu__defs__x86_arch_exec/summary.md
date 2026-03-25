# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__defs/spec_t__mmu__defs__x86_arch_exec.rs`
**Date:** 2026-03-24T13:25:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The x86-64 architecture constants are correctly specified: `L0_ENTRY_SIZE` fits in 64-bit usize, all layers have 512 entries matching hardware, entry sizes are correctly ordered top-down, and the fixed 4-layer structure matches standard x86-64 paging.

## All Candidates

### φ1: l0_entry_size_overflow_check
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `L0_ENTRY_SIZE = 512 * 512 * L2_ENTRY_SIZE = 512^3 * 4096 = 549755813888` which exceeds `usize::MAX` on 32-bit but fits in 64-bit — tests that the constant computation doesn't overflow given `size_of usize == 8`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property. `L0_ENTRY_SIZE` equals `512^3 * 4096 = 549755813888` which fits in 64-bit usize. The `global size_of usize == 8` declaration ensures 64-bit, so no overflow occurs. This is the intended value.

### φ2: arch_layers_all_same_num_entries
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** All four layers have identical `num_entries` (512) — if different layers should have different fan-outs, this uniformity could mask a missing architectural constraint
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** All x86-64 page table layers indeed have 512 entries each (9 bits per level). This is the correct hardware specification — each level of the 4-level page table has exactly 512 entries because each page table page is 4KB and each entry is 8 bytes.

### φ3: arch_exec_view_roundtrip_layer_count
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `ArchExec` uses a fixed `[ArchLayerExec; 4]` array — the view always has exactly 4 layers, preventing architectures with fewer page table levels (e.g., 3-level for 32-bit PAE)

### φ4: entry_sizes_strictly_decreasing
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Entry sizes are strictly decreasing from L0 to L3 — tests that the layer ordering matches the expected top-down hierarchy (L0=512G, L1=1G, L2=2M, L3=4K)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct and desirable. The x86-64 page table hierarchy has strictly decreasing entry sizes: L0=512G, L1=1G, L2=2M, L3=4K. Each layer maps a smaller region, which is the fundamental property of hierarchical page tables.

### φ5: arch_exec_fixed_size_array
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `ArchExec` uses `[ArchLayerExec; 4]` so every `ArchExec` instance always has exactly 4 layers — prevents representing 5-level paging (LA57) or non-standard page table depths
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `[ArchLayerExec; 4]` fixed-size array is an intentional design choice matching the x86-64 4-level page table architecture. LA57 (5-level paging) support would require a separate architectural definition. This is correct scoping, not a spec gap.

