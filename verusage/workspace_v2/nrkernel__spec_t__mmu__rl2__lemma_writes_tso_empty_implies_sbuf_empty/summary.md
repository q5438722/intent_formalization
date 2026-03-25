# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_writes_tso_empty_implies_sbuf_empty.rs`
**Date:** 2026-03-24T14:12:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. TSO-empty-implies-sbuf-empty is a correct consequence of the subset invariant. Sbuf uniqueness injectivity is a contrapositive restatement. The history independence test is a tautology (`P ==> P || Q`) that doesn't actually probe the mutual exclusion question it claims to test.

## All Candidates

### φ1: tso_empty_implies_all_sbufs_empty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When TSO write set is empty, the writer's own sbuf should also be empty — tests that `writer_sbuf_subset_tso_writes` correctly constrains the writer core too
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property. `writer_sbuf_subset_tso_writes` ensures every sbuf entry's address is in `writes.tso`. If `tso` is empty, no entries can exist, so the sbuf must be empty. The verified proof in `lemma_writes_tso_empty_implies_sbuf_empty` demonstrates this explicitly.

### φ2: contains_fst_empty_seq
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty sequence should not contain any first-element — tests that `contains_fst` handles the empty case correctly

### φ3: sbuf_unique_implies_no_duplicate_contains
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Address uniqueness in the writer sbuf should mean each address appears exactly once — tests that the forall quantifier correctly captures injectivity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `writer_sbuf_entries_are_unique` states that for `i1 != i2`, the addresses differ. The contrapositive is: if addresses are equal, then `i1 == i2`. This is a direct logical restatement of the uniqueness invariant, not a spec gap.

### φ4: valid_core_external_body_no_constraints
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `valid_core` is `external_body` — it may depend on fields beyond `node_count`/`core_count` (like range_ptmem), but being opaque means Verus can't verify this relationship

### φ5: history_pending_maps_unmaps_independent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pending_maps` and `pending_unmaps` are independent Map fields with no mutual exclusion constraint — a vaddr could be in both maps simultaneously, which would be inconsistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `P || Q` where `P` is already in the requires is a tautology — it's trivially true regardless of `Q`. This doesn't actually test whether both maps can contain the same key simultaneously; it just proves `P ==> P || Q`.

