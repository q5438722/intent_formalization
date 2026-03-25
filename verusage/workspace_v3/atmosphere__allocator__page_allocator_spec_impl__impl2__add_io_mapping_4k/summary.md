# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__add_io_mapping_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: io_mapping_replaces_all → `add_io_mapping_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, insert acts as full replacement—all pre-existing IO mappings on the target page are silently lost.

### φ2: io_leaks_to_regular_mappings → `add_io_mapping_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, adding an IO mapping also contaminates the regular (non-IO) mapping set, breaking mapping-type isolation.

### φ3: other_page_io_corrupted → `add_io_mapping_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the IO mapping addition bleeds into a different mapped page's IO mapping set, violating spatial isolation.

### φ4: set_ref_count_clears_io → `set_ref_count`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, set_ref_count silently clears IO mappings despite its spec claiming to preserve them, causing mapping data loss.

### φ5: set_io_mapping_corrupts_neighbor → `set_io_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, set_io_mapping at one page index corrupts the page state at a different index, breaking per-page isolation.

