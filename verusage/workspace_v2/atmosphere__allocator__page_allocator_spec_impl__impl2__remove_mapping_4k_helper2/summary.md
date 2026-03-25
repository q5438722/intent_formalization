# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper2.rs`
**Date:** 2026-03-24T06:30:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four of five candidates are true positives. The three container_map findings (4k/2m/1g) confirm a systematic gap: `wf()` never constrains `container_map_*` entries to reference only pages in the corresponding `allocated_pages_*` set, allowing containers to claim non-allocated pages. The mapped-page-no-mappings finding reveals that `mapped_pages_4k_wf` tracks state transitions but not mapping cardinality — a page can be in Mapped4k state with zero actual virtual address mappings. The free-and-allocated overlap is a false positive because page_array state exclusivity (Free4k vs Allocated4k) makes the preconditions unsatisfiable.

## True Positives (Spec Issues)

### container_map_4k_not_subset_allocated
- **Confidence:** high
- **Reasoning:** `container_map_4k` tracks per-container page ownership but `wf()` does not require these pages to be in `allocated_pages_4k`. The preconditions are simultaneously satisfiable — a container can claim a page that is free or in any other state, breaking the containment invariant.

### container_map_2m_not_subset_allocated
- **Confidence:** high
- **Reasoning:** Same pattern as 4k — `wf()` never constrains `container_map_2m` entries to be a subset of `allocated_pages_2m`. Containers can reference non-allocated 2M pages.

### container_map_1g_not_subset_allocated
- **Confidence:** high
- **Reasoning:** Same pattern — `container_map_1g` is unconstrained relative to `allocated_pages_1g` in `wf()`. Containers can reference non-allocated 1G pages.

### mapped_page_no_mappings_tracked
- **Confidence:** medium
- **Reasoning:** `mapped_pages_4k_wf` only links the mapped set to `PageState::Mapped4k` in the page array — it never requires `mappings@` to be non-empty. A page can transition to Mapped4k state and remain there even after all its virtual address mappings are removed, creating a stale "mapped" page with no actual mappings.

## All Candidates

### φ1: container_map_4k_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k tracks per-container page ownership but wf() may not require these pages to actually be in allocated_pages_4k — containers could claim unallocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_map_4k` tracks per-container page ownership but `wf()` does not require these pages to be in `allocated_pages_4k`. The preconditions are simultaneously satisfiable — a container can claim a page that is free or in any other state, breaking the containment invariant.

### φ2: container_map_2m_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_2m pages may not be required to exist in allocated_pages_2m — 2M container ownership could reference non-allocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same pattern as 4k — `wf()` never constrains `container_map_2m` entries to be a subset of `allocated_pages_2m`. Containers can reference non-allocated 2M pages.

### φ3: container_map_1g_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_1g pages may not be required to exist in allocated_pages_1g — 1G container ownership could reference non-allocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same pattern — `container_map_1g` is unconstrained relative to `allocated_pages_1g` in `wf()`. Containers can reference non-allocated 1G pages.

### φ4: free_and_allocated_pages_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A page could simultaneously be in the free list and the allocated set if wf() doesn't enforce state exclusivity between Free4k and Allocated4k at the page_array level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `free_pages_4k_wf` ensures free pages have state `Free4k`, and `allocated_pages_4k_wf` ensures allocated pages have state `Allocated4k`. Since each page has exactly one state in `page_array`, a page cannot simultaneously satisfy both `Free4k` and `Allocated4k`. The preconditions are unsatisfiable under `wf()`.

### φ5: mapped_page_no_mappings_tracked
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A page can be in mapped_pages_4k state yet have an empty mappings set — the spec may not require mapped pages to have at least one mapping recorded
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mapped_pages_4k_wf` only links the mapped set to `PageState::Mapped4k` in the page array — it never requires `mappings@` to be non-empty. A page can transition to Mapped4k state and remain there even after all its virtual address mappings are removed, creating a stale "mapped" page with no actual mappings.

