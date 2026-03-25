# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_interp_aux_contains_implies_interp_of_entry_contains.rs`
**Date:** 2026-03-24T12:45:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: interp_aux_key_from_entry
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Every key in the full interpretation comes from some entry — direct consequence of the verified lemma

### φ2: interp_aux_pair_from_entry
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Every key-value pair in the interpretation comes from some entry — tests that union_prefer_right doesn't fabricate new pairs

### φ3: page_entry_contains_base_key
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A page entry always contributes its entry_base as a key — tests the open spec singleton map definition

### φ4: invalid_entry_empty_interp
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Invalid entries contribute nothing to the interpretation — tests the open spec definition

### φ5: interp_aux_superset_of_entry
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Every entry's keys appear in the full interpretation — tests that union_prefer_right preserves all keys from earlier entries

