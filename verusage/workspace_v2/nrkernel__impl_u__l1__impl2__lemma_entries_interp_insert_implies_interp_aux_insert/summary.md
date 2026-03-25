# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_aux_insert.rs`
**Date:** 2026-03-24T12:41:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_interp_of_entry_disjoint_mappings` is an `external_body` axiom trusting that different entry indices in a well-formed directory produce disjoint virtual address mappings. The other three are false positives — a verified downstream consequence, a redundant symmetric direction of the same axiom, and a trivial open spec unfolding.

## True Positives (Spec Issues)

### disjoint_mappings_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_disjoint_mappings` is `external_body` with `unimplemented!()` body — the key property that different entry indices produce disjoint virtual address mappings is trusted without proof. The insert propagation lemma depends on this for correctness.

## All Candidates

### φ1: disjoint_mappings_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_disjoint_mappings` is `external_body` — disjointness of different entries' virtual address mappings is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_disjoint_mappings` is `external_body` with `unimplemented!()` body — the key property that different entry indices produce disjoint virtual address mappings is trusted without proof. The insert propagation lemma depends on this for correctness.

### φ2: insert_propagates_to_interp_aux
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Single-entry insertion propagates to interp_aux — the verified proof depends on the external_body disjointness lemma for the `idx > i` case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of the `lemma_entries_interp_insert_implies_interp_aux_insert` proof. The trust gap is already captured by φ1; this adds no new unverified assumption.

### φ3: disjoint_symmetric
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Symmetric direction of disjointness — also depends on the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just the symmetric direction from the same `external_body` ensures clause already flagged by φ1. Both directions are part of the same axiom — no additional trust gap.

### φ4: page_entry_singleton_key
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A page entry's interpretation contains exactly one key at its entry_base — tests the singleton map structure
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec `NodeEntry::interp` for `Page(p)` which returns `map![base => p]`. Correct by definition — a singleton map contains its key and has domain length 1.

### φ5: disjoint_page_entries_distinct_bases
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Two page entries at different indices must have different entry_bases — consequence of the external_body disjointness, but the base distinctness should follow from arithmetic alone

