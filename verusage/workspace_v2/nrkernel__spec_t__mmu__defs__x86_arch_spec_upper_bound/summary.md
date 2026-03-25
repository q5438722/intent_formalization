# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__defs/spec_t__mmu__defs__x86_arch_spec_upper_bound.rs`
**Date:** 2026-03-24T13:26:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `Arch::inv` allows an architecture with zero layers, which is likely unintended since a functional page table requires at least one level. Three false positives: single-layer architectures are valid, the last layer's vacuous `entry_size_is_next_layer_size` is by design, and `MAX_BASE` equaling `upper_vaddr(0, 0)` is intentional and explicitly verified.

## True Positives (Spec Issues)

### arch_inv_allows_empty_layers
- **Confidence:** medium
- **Reasoning:** An architecture with zero layers represents a page table that maps nothing — `upper_vaddr(0, 0)` would access out-of-bounds on the empty sequence. While `inv` uses `<=` for `layers.len()`, allowing zero layers is likely unintended since a page table must have at least one level to function. The invariant should require `self.layers.len() > 0`.

## All Candidates

### φ1: arch_inv_allows_single_layer
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Arch::inv` allows architectures with only 1 layer — `entry_size_is_next_layer_size` is vacuously true when there's no next layer, permitting degenerate single-level page tables
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `Arch` structure is designed to be generic over different page table configurations. A single-layer page table is a valid (if simple) architecture. The `inv` invariant correctly constrains entry sizes and counts — a single layer with valid parameters is not a spec gap.

### φ2: arch_inv_allows_empty_layers
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Arch::inv` allows an architecture with zero layers — the forall over an empty domain is vacuously true, resulting in a page table with no levels
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** An architecture with zero layers represents a page table that maps nothing — `upper_vaddr(0, 0)` would access out-of-bounds on the empty sequence. While `inv` uses `<=` for `layers.len()`, allowing zero layers is likely unintended since a page table must have at least one level to function. The invariant should require `self.layers.len() > 0`.

### φ3: upper_vaddr_not_canonical
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `upper_vaddr` computes to 256TB (512^2 * 1G = 2^48) which equals the 48-bit canonical address boundary — addresses at or above this are non-canonical on x86-64 without LA57

### φ4: entry_size_is_next_layer_vacuous_at_last
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `entry_size_is_next_layer_size` at the last layer (i=3) is vacuously true — no constraint on the leaf layer's entry size relationship, allowing arbitrary leaf sizes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by design. The last layer has no "next layer" to relate to, so the implication `i + 1 < self.layers.len() ==> ...` is vacuously true. The leaf layer's entry size is independently constrained by `0 < self.entry_size(i) <= X86_MAX_ENTRY_SIZE` in `inv`.

### φ5: max_base_equals_upper_vaddr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `MAX_BASE` is defined as `X86_MAX_ENTRY_SIZE * 512` which equals `upper_vaddr(0, 0)` — if these should represent different bounds (e.g., physical vs virtual), their equality could mask a missing distinction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This equality is intentional and verified by `x86_arch_spec_upper_bound`. `MAX_BASE` represents the upper bound of the virtual address space, which is exactly `upper_vaddr(0, 0)` — the total addressable space from the root of the page table. These aren't meant to be different bounds.

