# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_sbuf_facts.rs`
**Date:** 2026-03-24T14:20:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Layer 0 never producing Page and layer 3 never producing Directory are correct by construction of `PDE::view`'s layer-based branching. Walk vbase ≤ vaddr follows from downward alignment arithmetic. Depth-4 walks producing 4KB pages is correct by the path-length-to-entry-size mapping in `Walk::result`.

## All Candidates

### φ1: pde_layer0_never_page
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Layer 0 (PML4) entries should never be Page — they are always Directory or Invalid; if a Page entry appears at layer 0, the walk semantics would be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In `PDE::view`, layer 0 always produces `GPDE::Directory` when P bit is set and `all_mb0_bits_are_zero()` holds — the code has `if self.layer == 0 { GPDE::Directory { ... } }`. There is no Page branch for layer 0. This is correct by construction of the PDE view function.

### φ2: pde_layer3_never_directory
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Layer 3 (PT) entries should never be Directory — they are always Page or Invalid; if a Directory entry appears at layer 3, the walk would recurse beyond 4 levels
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In `PDE::view`, layer 3 always produces `GPDE::Page` when P bit is set and `all_mb0_bits_are_zero()` holds — the code for layer 3 has only the Page variant. This is correct by construction matching x86-64 semantics where PT entries are always leaf pages.

### φ3: walk_result_vbase_leq_vaddr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `vbase = align_to_usize(vaddr, entry_size)` rounds down, so `vbase <= vaddr` — if this fails, the alignment function is incorrectly rounding up
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `align_to_usize(a, b) = sub(a, a % b) = a - (a % b)`. Since `a % b >= 0`, the result is `<= a`. For path lengths 2-4, `vbase = align_to_usize(vaddr, entry_size) <= vaddr`. This is correct by the definition of downward alignment.

### φ4: walk_result_size_matches_depth
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A 4-level walk (ending at L3) should produce a 4KB page — tests that the depth-to-size mapping is correct; wrong sizes would break physical address translation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `path.len() == 4`, `Walk::result` selects the branch `(align_to_usize(self.vaddr, L3_ENTRY_SIZE), path[3].1->Page_addr, L3_ENTRY_SIZE)`, and the PTE frame size is set to `L3_ENTRY_SIZE = 4096`. This is correct by construction — 4-level walks map to 4KB pages.

### φ5: pde_layer_geq4_always_invalid
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Layers ≥ 4 don't exist in x86-64 — any PDE at layer 4 should be Invalid; if it's classified as Directory or Page, the walk could exceed the 4-level hierarchy

