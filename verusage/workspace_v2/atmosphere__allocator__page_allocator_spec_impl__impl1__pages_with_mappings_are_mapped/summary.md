# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl1__pages_with_mappings_are_mapped.rs`
**Date:** 2026-03-24T05:02:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives. The `lemma_usize_u64` external_body axiom is an unconstrained unverified assumption. More significantly, `container_wf` contains a likely copy-paste bug where `container_map_2m` and `container_map_1g` domains are both checked against `allocated_pages_4k` instead of their respective `allocated_pages_2m` and `allocated_pages_1g` sets — this means the spec does not actually enforce that 2m/1g container mappings reference correctly-sized allocated pages.

## True Positives (Spec Issues)

### usize_u64_roundtrip_unconstrained
- **Confidence:** high
- **Reasoning:** `lemma_usize_u64` is external_body with no requires, claiming all u64 values roundtrip through usize without proof. Unverified trust assumption.

### container_map_domain_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` — the 2m container map's domain is checked against 4k allocated pages instead of `allocated_pages_2m@`. This is almost certainly a copy-paste bug; the commented-out code above suggests the spec was still being developed.

### container_map_1g_domain_subset_allocated_4k
- **Confidence:** high
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`. This means the spec fails to enforce that 1g container mappings reference valid 1g-allocated pages.

## All Candidates

### φ1: usize_u64_roundtrip_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** lemma_usize_u64 is external_body with no requires — claims ALL u64 values roundtrip through usize without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_usize_u64` is external_body with no requires, claiming all u64 values roundtrip through usize without proof. Unverified trust assumption.

### φ2: free_pages_not_mapped
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** tests that the free_pages_are_not_mapped lemma holds — if wf() is vacuously unsatisfiable due to external_body StaticLinkedList::wf(), this is vacuously true

### φ3: container_map_domain_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_2m domain is asserted to be subset of allocated_pages_4k (not 2m) — likely a spec bug where 2m containers should track 2m allocations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `container_wf` asserts `container_map_2m@.dom().subset_of(allocated_pages_4k@)` — the 2m container map's domain is checked against 4k allocated pages instead of `allocated_pages_2m@`. This is almost certainly a copy-paste bug; the commented-out code above suggests the spec was still being developed.

### φ4: container_map_1g_domain_subset_allocated_4k
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** container_map_1g domain is asserted to be subset of allocated_pages_4k (not 1g) — likely a copy-paste spec bug where 1g containers should track 1g allocations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same copy-paste bug: `container_map_1g@.dom().subset_of(allocated_pages_4k@)` should be `allocated_pages_1g@`. This means the spec fails to enforce that 1g container mappings reference valid 1g-allocated pages.

### φ5: sll_wf_external_body_opaque
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::wf() is external_body closed spec — completely opaque, any property could be vacuously true if wf() is never satisfiable

