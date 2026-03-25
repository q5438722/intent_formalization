# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_unmapping.rs`
**Date:** 2026-03-24T14:21:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `all_mb0_bits_are_zero` is an external_body predicate that gates PDE classification — since it's opaque, Verus cannot verify it correctly identifies reserved bits, potentially misclassifying present entries. Four false positives: directory address masking, layer 1 page masking, L0 path entry recording, and path length bounds are all correct by construction from the open spec definitions.

## True Positives (Spec Issues)

### all_mb0_external_body_gates_all_present
- **Confidence:** medium
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` — it gates whether a present entry (P bit set) is classified as Directory/Page vs Invalid. Since the predicate is opaque, Verus cannot verify it actually checks the correct reserved bits for each layer, potentially causing valid entries to be misclassified as Invalid or vice versa.

## All Candidates

### φ1: pde_directory_addr_masked
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Directory addresses should be extracted via `entry & MASK_ADDR` — tests that the address field extraction is consistent across all layers that produce Directory entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In `PDE::view`, all Directory branches extract the address as `(v & MASK_ADDR) as usize` regardless of layer (layers 0, 1, 2 all use `MASK_ADDR` for Directory). This is correct by construction of the view function.

### φ2: pde_layer1_page_uses_different_mask
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Layer 1 (PDPT) Page entries use `MASK_L1_PG_ADDR` (bits 30+) instead of `MASK_ADDR` (bits 12+) — if wrong, 1GB pages would have incorrect physical base addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In `PDE::view`, layer 1 Page entries use `(v & MASK_L1_PG_ADDR) as usize` for the address. This is correct x86-64 semantics — 1GB pages have a 30-bit aligned physical base, so bits 30..MAX_PHYADDR_WIDTH-1 are the address field. The ensures matches the definition exactly.

### φ3: pt_walk_path_l0_entry_is_pde_view
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The first path entry's GPDE should match PDE::view of the raw entry read from pml4 + l0_index*8 — tests that pt_walk correctly records the PDE interpretation at layer 0
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pt_walk` computes `l0e = PDE { entry: self.read(l0_addr), layer: Ghost(0) }` and stores `l0e@` as the first path entry's GPDE in all branches. The ensures clause restates this construction exactly.

### φ4: all_mb0_external_body_gates_all_present
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When `all_mb0_bits_are_zero` returns false for a present entry, PDE::view falls through to Invalid — but since `all_mb0_bits_are_zero` is `external_body`, Verus cannot verify it actually checks the right bits, potentially misclassifying valid entries
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `all_mb0_bits_are_zero` is `external_body` with `unimplemented!()` — it gates whether a present entry (P bit set) is classified as Directory/Page vs Invalid. Since the predicate is opaque, Verus cannot verify it actually checks the correct reserved bits for each layer, potentially causing valid entries to be misclassified as Invalid or vice versa.

### φ5: walk_path_len_bounded_1_to_4
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The walk should always produce 1-4 path entries (L0 invalid → 1, L0→L1 invalid → 2, etc.) — tests that no branch of pt_walk creates paths outside these bounds
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pt_walk` has exactly 4 nesting levels of match on GPDE variants. The shortest path is 1 (L0 Invalid/Page), the longest is 4 (all Directory then L3 entry). Each branch constructs a path with `seq!` of 1-4 elements. This is correct by exhaustive case analysis of the function structure.

