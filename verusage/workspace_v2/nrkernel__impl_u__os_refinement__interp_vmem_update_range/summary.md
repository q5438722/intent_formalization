# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__interp_vmem_update_range.rs`
**Date:** 2026-03-24T13:10:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One candidate evaluated: the `union_prefer_right` shadowing behavior is a false positive. The code comment explicitly documents this as intentional — `interp_pt_mem` is preferred over `extra_mappings` at conflicting base addresses by design.

## All Candidates

### φ1: ptmem_view_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — completely unspecified, so two PTMem instances with different underlying memory could have identical PTE views

### φ2: rl3_interp_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec — completely opaque with no postconditions, so any two rl3 states could have identical rl2 interpretations

### φ3: extra_mappings_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `extra_mappings` is `external_body` with no specification — the SMT solver could assume it's always empty, silently dropping pending map/unmap state

### φ4: write_does_not_change_view
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Since `PTMem::view` is `external_body` with no connection to `mem`, writes to page table memory may have no observable effect on the PTE interpretation

### φ5: applied_mappings_union_prefer_right_shadows_extra
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `union_prefer_right` means `interp_pt_mem` always shadows `extra_mappings` at overlapping keys — extra_mappings entries at conflicting bases are silently dropped
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intentional design of `applied_mappings` — the source code comment explicitly states "Prefer interp_pt_mem because there might be a situation where we have something in the MapStart state which conflicts with something in interp_pt_mem." Using `union_prefer_right` to let `interp_pt_mem` shadow `extra_mappings` at conflicting keys is a deliberate design choice, not a spec gap.

