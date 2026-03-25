# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__add_mapping_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mapping_overwrites_existing → `add_mapping_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean add_mapping_4k discards all pre-existing mappings, keeping only the newly inserted one

### φ2: free_pages_drained → `add_mapping_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean adding a mapping always empties the 4k free list, corrupting allocator state

### φ3: target_also_in_allocated → `add_mapping_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean target_ptr is simultaneously mapped and allocated, violating page state exclusivity

### φ4: mapping_leaks_to_other_page → `add_mapping_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean the new (pcid, va) mapping leaks to an unrelated page, violating isolation

### φ5: mapped_set_gains_new_page → `add_mapping_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean add_mapping_4k introduces a spurious mapped page, breaking the frame condition on the mapped set

