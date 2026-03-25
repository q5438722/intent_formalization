# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__impl3__lemma_entry_sizes_increase.rs`
**Date:** 2026-03-24T12:20:27Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The monotonicity properties are correct consequences of the fully verified `lemma_entry_sizes_increase`. The degenerate configuration cases (`num_entries == 1`, non-power-of-two single layer) are permitted by design — `Arch::inv()` is intentionally a generic structural invariant, not an x86-specific one.

## All Candidates

### φ1: entry_size_decreases_with_depth
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Higher layers have larger-or-equal entry sizes — direct consequence of the verified lemma, tests that the invariant enforces monotonicity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Monotonically non-increasing entry sizes across layers is the correct and intended property of a hierarchical page table. The fully verified `lemma_entry_sizes_increase` proves this from the invariant.

### φ2: adjacent_entry_size_geq
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Adjacent layers satisfy monotonicity — the base case of the induction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the verified lemma at adjacent layers. This is the base case and a correct property — parent entry size must be at least child entry size.

### φ3: entry_size_equal_when_num_entries_one
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When a layer has exactly 1 entry, parent and child entry sizes are equal — the invariant allows `num_entries == 1` which means a layer maps the same size as its child, creating a redundant nesting level
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** When `num_entries(i+1) == 1`, the invariant gives `entry_size(i) == entry_size(i+1) * 1`. While this creates a redundant nesting level, the `Arch` type is a generic architecture description and `inv()` intentionally permits degenerate configurations. The x86-specific `x86_arch_spec` would never have `num_entries == 1`.

### φ4: inv_allows_non_power_of_two_entries
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The invariant permits non-power-of-two entry sizes and num_entries — x86 page tables always use powers of two, but `inv()` allows arbitrary factorizations like 6 = 2 * 3

### φ5: inv_single_layer_any_entry_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A single-layer architecture with arbitrary entry_size satisfies `inv()` — `entry_size_is_next_layer_size` is vacuously true for the only layer, so the sole constraint is `0 < entry_size <= X86_MAX_ENTRY_SIZE`
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** A single-layer architecture with non-standard entry size satisfying `inv()` is expected — `inv()` is a generic structural invariant, not an x86-specific constraint. The only bounds are `0 < entry_size <= X86_MAX_ENTRY_SIZE` and `0 < num_entries <= X86_NUM_ENTRIES`, which `{7, 3}` satisfies.

