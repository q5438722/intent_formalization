# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_share_pages__impl0__share_mapping/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: same_proc_loses_src_mapping → `share_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If sharing within the same process causes the pre-existing source VA mapping to vanish, the source loses access to its own page

### φ2: target_as_empty_after_share → `share_mapping`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the target address space becomes empty after inserting a shared mapping, all previously existing mappings of the target process are destroyed

### φ3: other_page_mapping_affected → `share_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If sharing a mapping mutates the reverse-mapping set of an unrelated physical page, page tracking is corrupted and could lead to use-after-free

### φ4: shared_entry_gains_write → `share_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a read-only shared mapping is silently promoted to writable, the target process gains unauthorized write access to the shared page

### φ5: refcount_double_increment → `share_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the reference counter increases by two or more for a single share operation, the page can never be fully freed, causing a permanent memory leak

