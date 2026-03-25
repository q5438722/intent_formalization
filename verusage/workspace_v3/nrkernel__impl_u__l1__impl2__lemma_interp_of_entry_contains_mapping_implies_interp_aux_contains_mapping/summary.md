# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_contains_mapping_implies_interp_aux_contains_mapping/original.rs`
**Date:** 2026-03-25 04:09:37
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two of three candidates are true positives revealing that `Arch::inv()` is overly permissive. First, it admits zero-layer architectures (vacuously true quantifier), which cannot represent any valid page table. Second, it admits architectures with arbitrarily small entry sizes (down to 1 byte), violating the x86 invariant that the smallest page is 4 KiB. Both suggest `inv()` should add `self.layers.len() > 0` and a minimum entry-size bound at the leaf layer. The `aligned(addr, 1)` property is a false positive — it's a mathematical tautology unrelated to any spec gap.

## True Positives (Spec Issues)

### empty_arch_satisfies_inv
- **Confidence:** high
- **Reasoning:** `Arch::inv()` uses a universally quantified formula over layers — when `layers.len() == 0`, the quantifier is vacuously true and `0 <= X86_NUM_LAYERS` holds, so `inv()` is satisfied. A zero-layer architecture is physically meaningless (can't map anything) and likely should be excluded by requiring `self.layers.len() > 0`.

### byte_granularity_arch_valid
- **Confidence:** medium
- **Reasoning:** An architecture with a single layer mapping a 1-byte entry is accepted by `Arch::inv()`, but x86 page tables never operate at byte granularity — the minimum page size is 4 KiB. The invariant lacks a lower bound on entry sizes (e.g., `entry_size(last_layer) >= 4096`), allowing nonsensical architectures. This is the same class of weakness as the empty-arch case: the spec is too permissive.

## All Candidates

### φ1: empty_arch_satisfies_inv
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A zero-layer architecture cannot map any addresses and should not satisfy the invariant
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` uses a universally quantified formula over layers — when `layers.len() == 0`, the quantifier is vacuously true and `0 <= X86_NUM_LAYERS` holds, so `inv()` is satisfied. A zero-layer architecture is physically meaningless (can't map anything) and likely should be excluded by requiring `self.layers.len() > 0`.

### φ2: aligned_trivial_size_one
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Alignment to size 1 is vacuously true for every address, providing no meaningful constraint
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `aligned(addr, 1)` reducing to `addr % 1 == 0` is a basic mathematical tautology, not a spec weakness. The `aligned` function is a general-purpose predicate; it's expected and correct that alignment to 1 holds for all addresses. No executable function's specification is implicated.

### φ3: byte_granularity_arch_valid
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An arch with 1-byte entry size and single entry covers only 1 byte, far below any valid x86 page size
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** An architecture with a single layer mapping a 1-byte entry is accepted by `Arch::inv()`, but x86 page tables never operate at byte granularity — the minimum page size is 4 KiB. The invariant lacks a lower bound on entry sizes (e.g., `entry_size(last_layer) >= 4096`), allowing nonsensical architectures. This is the same class of weakness as the empty-arch case: the spec is too permissive.

### φ4: zero_width_entry_region
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Equal entry_base and next_entry_base means a zero-width address region, so distinct entries would alias

### φ5: wellformed_implies_zero_entries
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A well-formed directory with zero entries would make the page table structure degenerate and unmappable

