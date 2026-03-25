# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_share_pages__impl0__create_entry_and_share/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: share_flips_write_perm → `create_entry_and_share`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If sharing flips write permissions, a read-only page could become writable in the target, breaking memory safety

### φ2: same_proc_src_va_corrupted → `create_entry_and_share`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When src_proc_ptr == target_proc_ptr, the original source VA mapping must be preserved; corruption would silently remap existing memory

### φ3: page_mapping_drops_source → `create_entry_and_share`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If inserting the new share target drops the original source tracking entry, the kernel would lose the ability to revoke or unmap the source's access

### φ4: refcount_not_incremented → `create_entry_and_share`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the physical page reference counter does not increase after sharing, premature page reclamation could cause use-after-free

### φ5: target_gains_extra_mappings → `create_entry_and_share`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If sharing introduces unintended extra VA mappings in the target, the target process gains access to memory it should not see

