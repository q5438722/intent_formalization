# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlspec/spec_t__hlspec__next_step_preserves_inv.rs`
**Date:** 2026-03-24T13:24:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: the `overlap` zero-size reflexivity issue (minor boundary defect) and the missing physical memory bounds check in `step_Map_enabled` (acknowledged by a TODO comment in the source). Three false positives: the `choose` arbitrary branch is unreachable due to contradictory preconditions, `update_range` with empty sequence is correct, and excluding 16-byte operations is an intentional scoping decision.

## True Positives (Spec Issues)

### overlap_zero_size_reflexive
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

### is_in_mapped_region_phys_mem_bound_not_in_enabled
- **Confidence:** high
- **Reasoning:** The source code explicitly has a TODO comment acknowledging this gap: "This should arguably be something we require in step_Map_enabled so we'd know all mapped memory is valid." Without the check in `step_Map_enabled`, mappings can be created where the physical frame extends beyond physical memory, causing some virtual addresses within the mapping to fail `is_in_mapped_region` despite being in a valid mapping's virtual range.

## All Candidates

### φ1: vaddr_mapping_modified_choose_arbitrary_on_idle
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If `choose` picks an Idle thread (which shouldn't satisfy the exists but the `choose` is unconstrained when multiple witnesses exist), `vaddr_mapping_is_being_modified_choose` falls into the `arbitrary()` branch returning an unspecified result
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The preconditions are contradictory in practice. If `vaddr_mapping_is_being_modified(c, va)` holds, there exists a thread with a Map or Unmap state covering `va`. The `choose` operator picks a witness satisfying the predicate, so it cannot pick an Idle thread. The precondition forcing `choose_thread == thread` where `thread` is Idle creates a vacuously satisfiable but practically unreachable scenario — `choose` is deterministic and will select a valid witness.

### φ2: update_range_length_mismatch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `update_range` with an empty `new` sequence should be identity — tests that the subrange arithmetic correctly handles the zero-length case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `update_range` with an empty `new` sequence correctly returns the original sequence — `s.subrange(0, 0) + Seq::empty() + s.subrange(0, s.len())` equals `s`. This is correct arithmetic behavior, not a spec gap.

### φ3: overlap_zero_size_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping via `region1.base == region2.base` — empty intervals should not overlap
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

### φ4: memop_valid_size_excludes_16
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `valid_op_size` only allows 1, 2, 4, 8 byte operations — 16-byte SSE/AVX memory operations are excluded, potentially preventing verification of SIMD memory accesses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is an intentional design choice. The spec models standard x86 scalar memory operations (1, 2, 4, 8 bytes). SIMD/AVX operations are out of scope for this MMU verification. Excluding 16-byte operations is a deliberate scoping decision, not a spec gap.

### φ5: is_in_mapped_region_phys_mem_bound_not_in_enabled
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The TODO comment in `is_in_mapped_region` notes that `pte.frame.base + (vaddr - base) < phys_mem_size` should be checked in `step_Map_enabled` — without it, mappings can be created that reference physical memory beyond bounds, causing some virtual addresses within the mapping to fail `is_in_mapped_region`
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The source code explicitly has a TODO comment acknowledging this gap: "This should arguably be something we require in step_Map_enabled so we'd know all mapped memory is valid." Without the check in `step_Map_enabled`, mappings can be created where the physical frame extends beyond physical memory, causing some virtual addresses within the mapping to fail `is_in_mapped_region` despite being in a valid mapping's virtual range.

