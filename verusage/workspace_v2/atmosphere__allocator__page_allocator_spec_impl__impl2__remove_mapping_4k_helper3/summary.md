# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper3.rs`
**Date:** 2026-03-24T06:31:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four of five candidates are true positives. The three container_map findings (4k/2m/1g) reveal a systematic gap: `wf()` never constrains `container_map_*` fields, allowing containers to claim pages regardless of allocation state. The mapped-page-no-mappings finding shows `mapped_pages_4k_wf` tracks state transitions but not mapping cardinality. The merged-2m-unavailable finding is a false positive — `Unavailable2m` is explicitly listed as a valid parent state in `merged_pages_wf`, representing legitimate hardware reservation scenarios.

## True Positives (Spec Issues)

### container_map_4k_not_subset_allocated
- **Confidence:** high
- **Reasoning:** The visible `wf()` sub-specs (`page_array_wf`, `free_pages_4k_wf`, `allocated_pages_4k_wf`, `mapped_pages_4k_wf`, `merged_pages_wf`) never mention `container_map_4k`. Since `container_map_4k` is a `Ghost` field with no constraining predicate in `wf()`, containers can claim pages in any state — free, mapped, or non-existent.

### container_map_2m_not_subset_allocated
- **Confidence:** high
- **Reasoning:** Same systematic gap — `container_map_2m` is never constrained by any visible `wf()` sub-predicate. No relationship between container ownership and allocation state is enforced for 2M pages.

### container_map_1g_not_subset_allocated
- **Confidence:** high
- **Reasoning:** Same pattern — `container_map_1g` is unconstrained by `wf()`. The container ownership tracking for 1G pages is completely decoupled from the allocation state machine.

### mapped_page_no_mappings_tracked
- **Confidence:** medium
- **Reasoning:** `mapped_pages_4k_wf` links the mapped set to `PageState::Mapped4k` in the page array but never requires `mappings@` to be non-empty. A page can be in Mapped4k state with zero virtual address mappings recorded, which is semantically inconsistent — a "mapped" page should have at least one mapping.

## All Candidates

### φ1: container_map_4k_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_4k tracks per-container page ownership but wf() may not require these pages to actually be in allocated_pages_4k — containers could claim unallocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The visible `wf()` sub-specs (`page_array_wf`, `free_pages_4k_wf`, `allocated_pages_4k_wf`, `mapped_pages_4k_wf`, `merged_pages_wf`) never mention `container_map_4k`. Since `container_map_4k` is a `Ghost` field with no constraining predicate in `wf()`, containers can claim pages in any state — free, mapped, or non-existent.

### φ2: container_map_2m_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_2m pages may not be required to exist in allocated_pages_2m — 2M container ownership could reference non-allocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same systematic gap — `container_map_2m` is never constrained by any visible `wf()` sub-predicate. No relationship between container ownership and allocation state is enforced for 2M pages.

### φ3: container_map_1g_not_subset_allocated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_1g pages may not be required to exist in allocated_pages_1g — 1G container ownership could reference non-allocated pages
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same pattern — `container_map_1g` is unconstrained by `wf()`. The container ownership tracking for 1G pages is completely decoupled from the allocation state machine.

### φ4: mapped_page_no_mappings_tracked
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A page can be in Mapped4k state yet have an empty mappings set — the spec may not require mapped pages to have at least one mapping recorded
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mapped_pages_4k_wf` links the mapped set to `PageState::Mapped4k` in the page array but never requires `mappings@` to be non-empty. A page can be in Mapped4k state with zero virtual address mappings recorded, which is semantically inconsistent — a "mapped" page should have at least one mapping.

### φ5: merged_2m_parent_could_be_unavailable
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** merged_pages_wf allows a Merged2m page's parent to be Unavailable2m — a merged sub-page of an unavailable 2M page is a contradictory state that should not be allowed
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** `merged_pages_wf` explicitly lists `Unavailable2m` as a valid parent state for `Merged2m` pages. This is intentional — "unavailable" 2M pages represent reserved/hardware-claimed regions where the sub-pages are merged into the 2M granule but not available for allocation. The state is not contradictory; it represents a valid hardware reservation scenario.

