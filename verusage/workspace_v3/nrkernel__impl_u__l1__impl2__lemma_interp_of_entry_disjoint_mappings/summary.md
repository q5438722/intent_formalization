# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_disjoint_mappings/original.rs`
**Date:** 2026-03-25 04:10:25
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two of three candidates are true positives. The empty-layers issue (`arch_inv_empty_layers`) reveals that `Arch::inv()` vacuously holds for zero-layer architectures, which should be excluded since they represent nonsensical page table configurations. The unaligned-base issue (`wellformed_unaligned_base`) exposes a commented-out alignment check in `Directory::well_formed()`, allowing directories with misaligned base addresses that could undermine the soundness of address-range disjointness proofs (especially those in `external_body` lemmas). The nonstandard entry size for the last layer (`nonstandard_entry_size_last_layer`) is a false positive — the vacuous implication for the last layer is intentional in this generic `Arch` abstraction, and concrete x86 constraints are applied at instantiation time.

## True Positives (Spec Issues)

### arch_inv_empty_layers
- **Confidence:** high
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers — when `layers.len() == 0`, the quantifier is vacuously true and `0 <= X86_NUM_LAYERS` holds, so the empty arch satisfies `inv()`. This is a real spec weakness: an architecture with no layers is meaningless for address translation and should be excluded (e.g., by requiring `self.layers.len() > 0`).

### wellformed_unaligned_base
- **Confidence:** high
- **Reasoning:** The alignment check `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` is explicitly commented out in `well_formed()`. This allows directories with unaligned base addresses (like `base_vaddr == 1`) to satisfy `well_formed()` and `inv()`. This is a real spec gap — unaligned base addresses break the arithmetic properties that the page table interpretation relies on (e.g., disjointness of mapped regions depends on alignment assumptions hidden in `external_body` lemmas).

## All Candidates

### φ1: aligned_zero_size
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Size-zero alignment should not hold for nonzero addresses; indicates missing guard on size > 0

### φ2: arch_inv_empty_layers
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers maps nothing and should not satisfy the invariant
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers — when `layers.len() == 0`, the quantifier is vacuously true and `0 <= X86_NUM_LAYERS` holds, so the empty arch satisfies `inv()`. This is a real spec weakness: an architecture with no layers is meaningless for address translation and should be excluded (e.g., by requiring `self.layers.len() > 0`).

### φ3: nonstandard_entry_size_last_layer
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A non-power-of-two entry size (7 bytes) should not pass the layer-size consistency check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `entry_size_is_next_layer_size` is an implication `i + 1 < self.layers.len() ==> ...`, which is vacuously true for the last layer. The invariant only constrains entry sizes to be in `(0, X86_MAX_ENTRY_SIZE]` — it deliberately does not require power-of-two sizes. The last layer's entry size being unconstrained beyond the range bound is by design; the real x86 constraint (4K pages) is enforced elsewhere at instantiation, not in this generic `Arch` spec.

### φ4: wellformed_unaligned_base
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A directory with base_vaddr=1 (unaligned) should not be well-formed; alignment check is commented out
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The alignment check `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` is explicitly commented out in `well_formed()`. This allows directories with unaligned base addresses (like `base_vaddr == 1`) to satisfy `well_formed()` and `inv()`. This is a real spec gap — unaligned base addresses break the arithmetic properties that the page table interpretation relies on (e.g., disjointness of mapped regions depends on alignment assumptions hidden in `external_body` lemmas).

### φ5: entry_base_equals_next_entry_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Consecutive entry bases being equal would collapse distinct entries into zero-width regions

