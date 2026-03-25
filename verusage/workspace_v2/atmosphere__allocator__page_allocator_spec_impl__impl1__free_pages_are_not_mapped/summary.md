# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/allocator/allocator__page_allocator_spec_impl__impl1__free_pages_are_not_mapped.rs`
**Date:** 2026-03-24T05:00:42Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive from the unconstrained external_body axiom `lemma_usize_u64` (no requires clause, claims all u64 roundtrip through usize without proof). One false positive is a specific instantiation with no independent trust surface.

## True Positives (Spec Issues)

### usize_u64_roundtrip_unconstrained
- **Confidence:** high
- **Reasoning:** `lemma_usize_u64` is external_body with no requires, claiming all u64 values roundtrip through usize. This is an unverified axiom; while likely true on 64-bit targets, it has no precondition guard and no proof.

## All Candidates

### φ1: usize_u64_roundtrip_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** lemma_usize_u64 is external_body with no requires — claims ALL u64 values roundtrip through usize, which is unsound on 32-bit platforms where usize is 32 bits
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_usize_u64` is external_body with no requires, claiming all u64 values roundtrip through usize. This is an unverified axiom; while likely true on 64-bit targets, it has no precondition guard and no proof.

### φ2: large_u64_roundtrip
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** u64::MAX (2^64-1) cannot roundtrip through usize on 32-bit — the unconstrained external_body axiom enables this unsound claim
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of φ1 at u64::MAX. No independent trust surface beyond the already-flagged external_body axiom.

### φ3: wf_external_body_opaque
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::wf() is external_body closed spec — completely opaque, any property could be vacuously true if wf() is never satisfiable

### φ4: len_external_body_consistency
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len() is external_body — unverified that value_list_len matches the ghost spec_seq length

### φ5: usize_u64_enables_overflow
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** MAX_USIZE+1 (~31GB+1) roundtrip claimed without proof — the external_body lemma applies to values beyond the system's intended address range

