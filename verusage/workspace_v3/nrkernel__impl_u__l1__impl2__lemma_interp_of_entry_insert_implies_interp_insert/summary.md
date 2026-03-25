# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_insert_implies_interp_insert/original.rs`
**Date:** 2026-03-25 04:10:46
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two of three candidates are true positives. The empty-arch invariant (φ1) reveals that `Arch::inv()` lacks a `layers.len() > 0` guard, admitting degenerate zero-layer architectures. The unaligned base_vaddr finding (φ2) confirms a known gap: the alignment constraint in `well_formed` is commented out, so directories can have arbitrary base addresses. The all-Invalid directory (φ3) is a false positive — empty page table regions are legitimate and expected in any MMU model.

## True Positives (Spec Issues)

### empty_arch_satisfies_inv
- **Confidence:** medium
- **Reasoning:** `Arch::inv()` with an empty layers sequence vacuously satisfies all quantified conditions, yet such an architecture is meaningless for x86 page tables. The spec likely intends `layers.len() > 0` (or a specific layer count). This is a genuine spec weakness — the invariant is too permissive at the boundary.

### unaligned_base_vaddr_well_formed
- **Confidence:** high
- **Reasoning:** The commented-out alignment check `//&&& aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed` is clearly intentional but disabled, allowing directories with arbitrary unaligned base addresses. This is a real spec gap — the code acknowledges alignment should be checked but currently doesn't enforce it.

## All Candidates

### φ1: empty_arch_satisfies_inv
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers vacuously satisfies inv, allowing a degenerate page table that cannot map any address space
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Arch::inv()` with an empty layers sequence vacuously satisfies all quantified conditions, yet such an architecture is meaningless for x86 page tables. The spec likely intends `layers.len() > 0` (or a specific layer count). This is a genuine spec weakness — the invariant is too permissive at the boundary.

### φ2: unaligned_base_vaddr_well_formed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A directory with base_vaddr=1 (unaligned to any page boundary) passes well_formed because the alignment check is commented out in the source
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The commented-out alignment check `//&&& aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed` is clearly intentional but disabled, allowing directories with arbitrary unaligned base addresses. This is a real spec gap — the code acknowledges alignment should be checked but currently doesn't enforce it.

### φ3: all_invalid_entries_satisfy_inv
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A directory where every entry is Invalid vacuously satisfies all inv sub-conditions, meaning the spec permits a page table that maps nothing at all
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An all-Invalid directory representing an empty (unmapped) page table region is perfectly valid and expected in a real MMU. Not every page table entry needs to map something — unmapped regions are the norm. This is not a spec gap; it's correct by design.

### φ4: update_wrong_page_size_preserves_inv
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a page with mismatched frame size can be inserted while preserving inv, the pages_match_entry_size invariant is ineffective

### φ5: entry_size_equal_across_layers
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Two consecutive layers with identical entry_size (via num_entries=1) means a layer adds no address-space subdivision, violating the x86 expectation that each layer partitions into multiple entries

