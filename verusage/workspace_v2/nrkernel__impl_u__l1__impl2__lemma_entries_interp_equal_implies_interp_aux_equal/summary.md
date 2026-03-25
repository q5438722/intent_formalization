# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_aux_equal.rs`
**Date:** 2026-03-24T12:39:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains a fully verified lemma proving interp_aux extensional equality, with no external_body or trust gaps. The other candidates test open spec definitions directly. The directory-ignores-base observation is interesting but intentional — subdirectories carry their own `base_vaddr` set by the `directories_are_in_next_layer` invariant.

## All Candidates

### φ1: interp_aux_equal_from_entries
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two directories with same entry interpretations have the same interp — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the fully verified `lemma_entries_interp_equal_implies_interp_aux_equal`. Correct extensional equality property proved by induction.

### φ2: interp_aux_tail_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `interp_aux` past the end of entries returns empty map — tests the base case of the recursive definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec `interp_aux` base case (`i >= entries.len() => map![]`). Correct by definition.

### φ3: page_entry_singleton_map
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A page entry interprets as a singleton map keyed by its base — tests the open spec definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec `NodeEntry::interp` for the `Page` variant. Correct by definition.

### φ4: invalid_entry_empty_map
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An invalid entry interprets as the empty map — tests the open spec definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec `NodeEntry::interp` for the `Invalid` variant. Correct by definition.

### φ5: directory_entry_ignores_base
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A directory entry's interpretation ignores the `base` parameter — `NodeEntry::Directory(d)` calls `d.interp_aux(0)` discarding `base`, which may cause the parent's entry_base to be silently dropped
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `Directory` variant calls `d.interp_aux(0)` which uses `d.base_vaddr` internally — the directory carries its own base address. The `directories_are_in_next_layer` invariant ensures `d.base_vaddr` is correctly set to the parent's entry_base, so the `base` parameter is intentionally unused. This is correct design.

