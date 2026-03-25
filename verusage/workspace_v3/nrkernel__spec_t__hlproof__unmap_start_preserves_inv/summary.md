# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__hlproof__unmap_start_preserves_inv/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment to size 0 is satisfied for any nonzero address, the aligned predicate is vacuously permissive for a degenerate input.

### φ2: x86_arch_inv_false → `inv`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the concrete x86 architecture constant fails its own invariant, every downstream proof relying on inv is vacuously true.

### φ3: entry_base_zero_idx_not_base → `entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Indexing at 0 should recover the original base; if not, address calculations for the first entry in every directory are wrong.

### φ4: upper_vaddr_zero_at_root → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A zero upper virtual address at layer 0 with base 0 would mean the entire x86-64 address space has zero size, making all vaddr checks trivially fail.

### φ5: entry_size_same_across_layers → `entry_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Layer 0 maps 512 GiB regions while layer 3 maps 4 KiB pages; equal entry sizes would collapse the page-table hierarchy into a single flat level.

