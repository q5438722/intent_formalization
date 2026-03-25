# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping/original.rs`
**Date:** 2026-03-25 04:09:59
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One true positive found: `Arch::inv()` is vacuously satisfied by an `Arch` with zero layers because both the layer-count bound (`0 <= 4`) and the universally-quantified layer properties hold trivially on an empty sequence. This is a genuine spec weakness — the invariant should include a `self.layers.len() > 0` conjunct (or similar) to rule out degenerate empty architectures that no real x86 page table would have. Any downstream proof or executable code assuming `arch.inv()` implies the existence of at least one translation layer is relying on an unwritten assumption.

## True Positives (Spec Issues)

### arch_inv_vacuous_empty
- **Confidence:** high
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers — when `layers.len() == 0`, the quantifier is vacuously true and the only other conjunct is `0 <= X86_NUM_LAYERS`, which trivially holds. This means an empty `Arch` (no layers at all) satisfies `inv()`, yet it represents no valid x86 page-table structure. Any code relying on `arch.inv()` to guarantee at least one layer of translation is unsound against this edge case. The spec targets `Arch::inv`, which is a pure spec function, but it is the central invariant consumed by executable/proof code throughout, so the gap is real.

## All Candidates

### φ1: arch_inv_vacuous_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An arch with zero layers should not satisfy inv, as it represents no meaningful page-table structure
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `Arch::inv()` uses a universal quantifier over layers — when `layers.len() == 0`, the quantifier is vacuously true and the only other conjunct is `0 <= X86_NUM_LAYERS`, which trivially holds. This means an empty `Arch` (no layers at all) satisfies `inv()`, yet it represents no valid x86 page-table structure. Any code relying on `arch.inv()` to guarantee at least one layer of translation is unsound against this edge case. The spec targets `Arch::inv`, which is a pure spec function, but it is the central invariant consumed by executable/proof code throughout, so the gap is real.

### φ2: entry_base_collision
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Consecutive entry bases being equal would mean distinct indices map overlapping address regions

### φ3: dir_inv_wrong_page_size
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a page frame size can differ from the layer's entry size under inv, address translation would be incorrect

### φ4: well_formed_allows_empty
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A well-formed directory with zero entries would make the page table level degenerate and break address space coverage

### φ5: interp_page_mapping_lost
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A page mapping present in an entry's interpretation but absent from the directory's full interpretation would silently drop valid mappings

