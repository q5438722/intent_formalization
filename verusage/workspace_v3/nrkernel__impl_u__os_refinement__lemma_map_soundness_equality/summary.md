# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__os_refinement__lemma_map_soundness_equality/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned with size=0 should not be satisfiable (division by zero), indicating a missing precondition

### φ2: upper_vaddr_no_progress → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** upper_vaddr should always exceed base under inv(), otherwise the layer maps a zero-size address range

### φ3: entry_base_non_monotonic → `entry_base`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** entry_base must be strictly monotonic in idx; non-monotonicity would cause overlapping page table entries

### φ4: inv_allows_empty_arch → `inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** inv() should require at least one layer; an empty architecture cannot perform any address translation

### φ5: deeper_layer_larger_entries → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** in a valid page table hierarchy, deeper layers must have smaller-or-equal entry sizes; larger would invert the address decomposition

