# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__impl__empty.rs`
**Date:** 2026-03-24T11:04:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: is_bit_set_not0_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_bit_set(!0usize, j)` is axiomatized for ALL `j: usize` with no `j < 64` guard — for `j >= 64`, `1usize << j` wraps, so the axiom asserts properties about out-of-range bit positions

### φ2: or_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The OR biconditional has no `j < 64` guard — for out-of-range positions where `1usize << j` wraps, this axiom produces conclusions about wrapped bit operations

### φ3: and_bit_set_no_bound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The AND biconditional has no `j < 64` guard — for out-of-range bit positions, `1usize << j` wraps and the axiom asserts equivalences that don't correspond to real bit semantics

### φ4: not0_and_zero_inconsistency
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Combining unbounded `is_bit_set(!0, j)` with unbounded AND biconditional yields `is_bit_set(0, j)` for `j >= 64` — but `!0usize & 0usize == 0`, so this asserts a set bit in zero, potentially deriving false

### φ5: lemma_view_no_range_guard
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view`'s first ensures decomposes any `i` in `self@` without guarding `0 <= a < 8` — for `i >= 512` or `i < 0`, `a` falls outside `[0,8)` making `self.mask[a]` an out-of-bounds spec access

