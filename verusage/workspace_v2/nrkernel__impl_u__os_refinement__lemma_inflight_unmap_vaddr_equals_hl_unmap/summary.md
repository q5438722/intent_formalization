# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__lemma_inflight_unmap_vaddr_equals_hl_unmap.rs`
**Date:** 2026-03-24T13:11:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. Overlap reflexivity and self-overlap detection are correct, desirable properties of the memory region overlap definitions — they ensure the system properly detects conflicting mappings at identical base addresses.

## All Candidates

### φ1: ptmem_view_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — completely unspecified, so distinct page table memories could have identical PTE interpretations

### φ2: rl3_interp_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec — any two rl3 states could map to the same rl2 state since the function is completely opaque

### φ3: overlap_reflexive
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A non-empty region overlaps itself — tests that `overlap` correctly handles the reflexive case via the `region1.base == region2.base` branch
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A non-empty region overlapping itself is correct and expected behavior. The `overlap` function's first branch handles `region1.base == region2.base` returning true, which is the correct mathematical definition of overlapping memory regions.

### φ4: candidate_mapping_overlaps_self
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A mapping at an existing base always overlaps — `candidate_mapping_overlaps_existing_vmem` triggers with witness `b == base` since `overlap` is reflexive at equal bases regardless of sizes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by design — if a mapping already exists at `base`, any new mapping at the same base would overlap it. This is exactly the behavior `candidate_mapping_overlaps_existing_vmem` is meant to detect, preventing double-mapping at the same virtual address.

### φ5: write_does_not_change_view
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Since `PTMem::view` is `external_body` with no connection to `mem`, writes to page table memory may have no observable effect on the PTE interpretation

