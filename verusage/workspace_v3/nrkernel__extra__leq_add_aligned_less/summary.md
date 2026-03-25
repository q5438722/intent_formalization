# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__leq_add_aligned_less/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: trivial_alignment → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every address is aligned to every positive size, the alignment spec is vacuously true and useless

### φ2: alignment_shift_by_one → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment is preserved when the address is incremented by one, the modular constraint is broken

### φ3: alignment_only_zero → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If only zero can be aligned to any size greater than one, the spec incorrectly excludes all positive multiples

### φ4: alignment_implies_unit_size → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If any positive aligned address forces the size to be 1, the spec collapses nontrivial alignments

### φ5: misaligned_offset_accepted → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If an address offset by size-1 remains aligned, the spec fails to distinguish aligned from misaligned addresses within a block

