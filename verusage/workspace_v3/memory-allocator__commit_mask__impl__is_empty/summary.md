# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__is_empty/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: nonzero_mask_empty_view → `is_empty`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A nonzero mask entry should mean at least one bit is represented in the view, so the view must not be empty

### φ2: view_contains_negative → `is_empty`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** View elements are a*64+b with a in [0,8) and b in [0,64), so no element should ever be negative

### φ3: view_always_empty → `is_empty`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the view is always empty regardless of mask contents, is_empty trivially always returns true and the abstraction is useless

### φ4: all_ones_mask_is_empty → `is_empty`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A mask with every bit set in every field should have the fullest possible view, never empty

### φ5: bit0_not_in_view → `is_empty`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If bit 0 of mask[0] is set, element 0*64+0=0 must appear in the view; its absence would break the bit-to-set mapping

