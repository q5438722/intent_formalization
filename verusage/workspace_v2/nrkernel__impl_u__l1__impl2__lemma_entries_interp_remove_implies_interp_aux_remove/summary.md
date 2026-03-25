# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_entries_interp_remove_implies_interp_aux_remove.rs`
**Date:** 2026-03-24T12:43:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `lemma_interp_of_entry_key_between` and `lemma_interp_aux_contains_implies_interp_of_entry_contains` are both `external_body` axioms trusting key structural properties of the page table interpretation without proof — VA key bounding and key decomposition respectively. The other two are false positives — a verified downstream proof and a trivial open spec unfolding.

## True Positives (Spec Issues)

### interp_of_entry_key_between_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_key_between` is `external_body` with `unimplemented!()` body — the property that all VA keys in an entry's interpretation fall within `[entry_base(i), next_entry_base(i))` is trusted without proof. This is a critical axiom used by the remove propagation proof for disjointness reasoning.

### interp_aux_contains_implies_entry_contains_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_aux_contains_implies_interp_of_entry_contains` is `external_body` with `unimplemented!()` body — the decomposition that every key in `interp_aux(j)` comes from some `interp_of_entry(i)` with `j <= i` is trusted without proof. Used in the remove proof's contradiction argument.

## All Candidates

### φ1: interp_of_entry_key_between_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_key_between` is `external_body` — the bound that VA keys in an entry's interp fall within [entry_base, next_entry_base) is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_key_between` is `external_body` with `unimplemented!()` body — the property that all VA keys in an entry's interpretation fall within `[entry_base(i), next_entry_base(i))` is trusted without proof. This is a critical axiom used by the remove propagation proof for disjointness reasoning.

### φ2: interp_aux_contains_implies_entry_contains_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_aux_contains_implies_interp_of_entry_contains` is `external_body` — the decomposition of interp_aux keys into individual entry contributions is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_aux_contains_implies_interp_of_entry_contains` is `external_body` with `unimplemented!()` body — the decomposition that every key in `interp_aux(j)` comes from some `interp_of_entry(i)` with `j <= i` is trusted without proof. Used in the remove proof's contradiction argument.

### φ3: distinct_entries_disjoint_ranges
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Two applications of the external_body key-between axiom derive entry disjointness — if the bounds axiom were wrong, overlapping entries could silently shadow each other

### φ4: remove_propagates_to_interp_aux
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Remove propagation depends on both external_body axioms — the verified proof uses key-between and contains-implies-entry-contains as unverified foundations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of `lemma_entries_interp_remove_implies_interp_aux_remove`, which has a full proof body. The trust gaps are already captured by φ1 and φ2; this adds no new unverified assumption.

### φ5: page_entry_key_equals_base
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A page entry's singleton map has exactly one key (entry_base) — if va is in the map, it must equal the base; tests that the open spec unfolds correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct unfolding of the open spec `NodeEntry::interp` for `Page(p)` which returns `map![base => p]`. The singleton map contains only `base` as key, so `va == entry_base(i)`. Correct by definition.

