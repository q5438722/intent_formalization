# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__definitions_u__impl3__lemma_entry_sizes_aligned_auto/original.rs`
**Date:** 2026-03-25 04:07:55
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives were identified. The `inv` predicate is too permissive in three ways: (1) it admits empty architectures with zero layers via vacuous quantification, (2) it allows physically meaningless entry sizes as small as 1 byte when the modeled x86 hardware has a 4096-byte minimum page granularity, and (3) for single-layer architectures the inter-layer size constraint is vacuously satisfied, allowing completely arbitrary entry_size/num_entries combinations. Two properties were false positives: zero being aligned to everything is mathematically correct and not a spec issue, and allowing single-entry layers is a reasonable generic design choice.

## True Positives (Spec Issues)

### empty_arch_satisfies_inv
- **Confidence:** high
- **Reasoning:** The invariant uses a vacuously-true universal quantifier when `layers.len() == 0`, so an empty Arch trivially satisfies `inv()`. An empty architecture represents no address translation and is almost certainly not a valid configuration the spec should admit. The `inv` should require `self.layers.len() > 0`.

### inv_allows_unit_entry_size
- **Confidence:** medium
- **Reasoning:** The invariant only requires `0 < entry_size <= X86_MAX_ENTRY_SIZE`, so an entry size of 1 byte is permitted. For x86 page tables the minimum meaningful granularity is 4096 bytes; allowing sub-page entry sizes means the spec is weaker than the hardware reality it models. The invariant should enforce a minimum entry size (e.g., 4096).

### entry_size_next_layer_vacuous_single
- **Confidence:** high
- **Reasoning:** The `entry_size_is_next_layer_size` constraint is an implication that is vacuously true when there is no next layer. For a single-layer architecture, this means `entry_size` and `num_entries` are completely unconstrained relative to each other (e.g., entry_size=3, num_entries=2 is valid). This undermines the structural consistency the invariant is supposed to enforce — a single-layer arch should still have a meaningful relationship between its parameters.

## All Candidates

### φ1: empty_arch_satisfies_inv
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers should not satisfy the invariant, as it represents no valid address translation
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The invariant uses a vacuously-true universal quantifier when `layers.len() == 0`, so an empty Arch trivially satisfies `inv()`. An empty architecture represents no address translation and is almost certainly not a valid configuration the spec should admit. The `inv` should require `self.layers.len() > 0`.

### φ2: zero_aligned_to_everything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Address 0 being aligned to every nonzero size could mask null-pointer or zero-base-address bugs in callers
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % size == 0` for any nonzero size is a basic mathematical fact. The `aligned` function is a pure spec function, and address 0 being aligned is standard — callers that need to exclude zero addresses should do so in their own preconditions, not in the alignment definition.

### φ3: inv_allows_unit_entry_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An entry size of 1 byte is physically meaningless for page tables; inv should enforce a minimum granularity such as 4096
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The invariant only requires `0 < entry_size <= X86_MAX_ENTRY_SIZE`, so an entry size of 1 byte is permitted. For x86 page tables the minimum meaningful granularity is 4096 bytes; allowing sub-page entry sizes means the spec is weaker than the hardware reality it models. The invariant should enforce a minimum entry size (e.g., 4096).

### φ4: inv_allows_single_entry_layer
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A layer with only 1 entry is degenerate and does not meaningfully partition the address space
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** While a single-entry layer is degenerate, the spec is designed to be generic over architectures, and `num_entries >= 1` is a reasonable lower bound. A layer with 1 entry is not inherently unsound — it just maps a single contiguous region, which could be a valid (if unusual) architectural choice. This is a design decision, not a spec gap.

### φ5: entry_size_next_layer_vacuous_single
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** For a single-layer arch the next-layer constraint is vacuously true, allowing entry_size and num_entries to be completely unrelated, potentially hiding inconsistencies
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `entry_size_is_next_layer_size` constraint is an implication that is vacuously true when there is no next layer. For a single-layer architecture, this means `entry_size` and `num_entries` are completely unconstrained relative to each other (e.g., entry_size=3, num_entries=2 is valid). This undermines the structural consistency the invariant is supposed to enforce — a single-layer arch should still have a meaningful relationship between its parameters.

