# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_new_empty_dir/original.rs`
**Date:** 2026-03-25 04:11:37
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives were identified. First, `Arch::inv()` admits zero-layer architectures that cannot perform any address translation, suggesting a missing `layers.len() > 0` lower bound. Second, `Directory::well_formed()` has a commented-out alignment check for `base_vaddr`, allowing directories with misaligned bases to satisfy the invariant — a clear spec gap visible in the source. The remaining three properties are false positives: non-power-of-2 entry sizes reflect intentional genericity, all-Invalid directories are the expected empty state (used by `new_empty_dir`), and vacuous satisfaction of `pages_match_entry_size` when no pages exist is logically sound for non-leaf page table levels.

## True Positives (Spec Issues)

### arch_inv_zero_layers
- **Confidence:** medium
- **Reasoning:** An empty Arch with zero layers satisfies `inv()` because the universal quantifier over layers is vacuously true, yet such an architecture cannot support any address translation. The spec is missing a `self.layers.len() > 0` lower bound, which means downstream code must guard against this degenerate case separately rather than relying on `inv()` alone.

### well_formed_unaligned_base
- **Confidence:** high
- **Reasoning:** The source code has a commented-out alignment check (`//&&& aligned(self.base_vaddr, self.entry_size() * self.num_entries())`) in `well_formed`. This means directories with misaligned base addresses satisfy the invariant, which is incorrect for real page tables where base addresses must be aligned to the region they cover. This is a known spec gap explicitly visible in the code.

## All Candidates

### φ1: arch_inv_zero_layers
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An empty architecture with no layers satisfies inv(), yet it cannot describe any valid address translation
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** An empty Arch with zero layers satisfies `inv()` because the universal quantifier over layers is vacuously true, yet such an architecture cannot support any address translation. The spec is missing a `self.layers.len() > 0` lower bound, which means downstream code must guard against this degenerate case separately rather than relying on `inv()` alone.

### φ2: arch_inv_nonstandard_entry_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A single-layer arch with non-power-of-2 entry_size (7) and arbitrary num_entries (3) satisfies inv() since entry_size_is_next_layer_size is vacuously true for the last layer
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `Arch` spec is intentionally parameterized — it models a generic multi-level page table, not exclusively x86. Allowing non-power-of-2 entry sizes in a single-layer arch is a design choice; `entry_size_is_next_layer_size` is vacuously true for the last layer by definition, and the upper bounds (`<= X86_MAX_ENTRY_SIZE`, `<= X86_NUM_ENTRIES`) still hold. Constraining to powers of 2 would be an x86-specific policy, not a spec bug.

### φ3: well_formed_unaligned_base
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A directory with base_vaddr=1 (misaligned to any realistic entry_size) satisfies well_formed because the alignment check is commented out
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The source code has a commented-out alignment check (`//&&& aligned(self.base_vaddr, self.entry_size() * self.num_entries())`) in `well_formed`. This means directories with misaligned base addresses satisfy the invariant, which is incorrect for real page tables where base addresses must be aligned to the region they cover. This is a known spec gap explicitly visible in the code.

### φ4: dir_inv_all_invalid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A directory with every entry Invalid satisfies inv() vacuously, meaning an entirely empty page table at any layer is considered valid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An all-Invalid directory represents a valid, empty page table level — no mappings exist at that level. This is the expected state for freshly created directories (see `new_empty_dir` and `lemma_new_empty_dir` which construct exactly this). The invariant holding for empty directories is correct and intentional.

### φ5: pages_match_vacuous_no_pages
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** pages_match_entry_size is vacuously true when no entry is a Page, so a directory of only Directory/Invalid entries trivially satisfies it without any frame size guarantee
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A directory containing only Directory or Invalid entries has no page mappings, so `pages_match_entry_size` being vacuously true is logically correct — there are no frames to check. This is standard behavior for non-leaf levels of a page table hierarchy and is not a spec weakness.

