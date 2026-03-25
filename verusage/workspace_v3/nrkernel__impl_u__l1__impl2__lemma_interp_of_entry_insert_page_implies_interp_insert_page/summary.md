# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_insert_page_implies_interp_insert_page/original.rs`
**Date:** 2026-03-25 04:11:09
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both properties expose genuine weaknesses in `Arch::inv()`. The empty-layers case reveals a missing `layers.len() > 0` guard, allowing a vacuously valid zero-layer architecture. The degenerate single-layer case reveals missing lower bounds on `entry_size` (should be ≥ 4096 for x86 page granularity) and possibly `num_entries`, permitting architectures that don't correspond to any real x86 paging structure. These are spec-level gaps rather than soundness holes — the rest of the system likely never constructs such architectures in practice — but they weaken the invariant's ability to rule out nonsensical configurations.

## True Positives (Spec Issues)

### empty_arch_satisfies_inv
- **Confidence:** medium
- **Reasoning:** `Arch::inv()` uses a vacuously true universal quantifier when `layers.len() == 0`, so an empty architecture passes the invariant. This is a genuine spec weakness — a zero-layer arch represents no valid page table structure, yet `inv()` accepts it. The spec should require `self.layers.len() > 0`. This targets `Arch::inv()` which is a spec function, but it's the core invariant governing all executable operations on page tables.

### degenerate_arch_satisfies_inv
- **Confidence:** medium
- **Reasoning:** An architecture with `entry_size=1` and `num_entries=1` satisfies all constraints in `Arch::inv()` — entry_size is in (0, X86_MAX_ENTRY_SIZE], num_entries is in (0, 512], and the single-layer arch trivially satisfies `entry_size_is_next_layer_size`. Yet a 1-byte, 1-entry page table layer is nonsensical for x86. The spec lacks a lower bound on entry_size (should be ≥ 4096) and arguably on num_entries, allowing degenerate architectures.

## All Candidates

### φ1: empty_arch_satisfies_inv
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with zero layers should not satisfy inv, as it represents no valid page table structure
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Arch::inv()` uses a vacuously true universal quantifier when `layers.len() == 0`, so an empty architecture passes the invariant. This is a genuine spec weakness — a zero-layer arch represents no valid page table structure, yet `inv()` accepts it. The spec should require `self.layers.len() > 0`. This targets `Arch::inv()` which is a spec function, but it's the core invariant governing all executable operations on page tables.

### φ2: entry_base_zero_neq_base
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The base address of the first entry (idx=0) should equal the directory base; inequality would mean a gap or offset error

### φ3: update_changes_layer
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Updating a single entry should preserve the directory's layer; a change would corrupt the tree structure

### φ4: degenerate_arch_satisfies_inv
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An architecture with entry_size=1 byte and a single entry per layer is nonsensical for page tables and should be rejected by inv
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** An architecture with `entry_size=1` and `num_entries=1` satisfies all constraints in `Arch::inv()` — entry_size is in (0, X86_MAX_ENTRY_SIZE], num_entries is in (0, 512], and the single-layer arch trivially satisfies `entry_size_is_next_layer_size`. Yet a 1-byte, 1-entry page table layer is nonsensical for x86. The spec lacks a lower bound on entry_size (should be ≥ 4096) and arguably on num_entries, allowing degenerate architectures.

### φ5: all_invalid_implies_nonempty_interp
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A directory whose entries are all Invalid should have an empty interpretation; a non-empty map would mean phantom mappings

