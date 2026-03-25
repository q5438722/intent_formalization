# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__interp_vmem_subrange.rs`
**Date:** 2026-03-24T13:09:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One candidate evaluated: `vmem_unmapped_is_zero` is a false positive. The zero-fill for unmapped virtual addresses is an explicit, intentional design choice in the `vmem_apply_mappings` spec function, following the common convention that unmapped memory reads as zero.

## All Candidates

### φ1: ptmem_view_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no specification — since it's completely unspecified, Verus could derive anything including that writes don't change the view

### φ2: rl3_interp_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec — completely opaque with no postconditions, so any two states could have the same interpretation

### φ3: extra_mappings_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `extra_mappings` is `external_body` with no specification — since it's completely unspecified, the SMT solver could assume it's always empty

### φ4: applied_mappings_no_overlap_not_enforced
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `no_overlaps` is only a precondition of `interp_vmem_subrange` but never enforced as an invariant of `applied_mappings` — overlapping mappings could exist in the OS state

### φ5: vmem_unmapped_is_zero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Unmapped virtual addresses silently return 0 rather than being undefined or causing an error — could mask bugs where unmapped memory is accidentally read as valid zero data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intentional design of `vmem_apply_mappings` — the `else` branch explicitly returns `0` for unmapped addresses. This is a standard modeling choice for virtual memory semantics where unmapped pages read as zero, not a spec gap.

