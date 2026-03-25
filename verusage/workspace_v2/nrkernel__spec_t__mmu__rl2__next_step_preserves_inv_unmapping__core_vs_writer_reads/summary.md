# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_unmapping__core_vs_writer_reads.rs`
**Date:** 2026-03-24T14:29:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `valid_core` is `external_body` and completely unconstrained — structural equality of Core (which has only `node_id` and `core_id`) makes the proof go through, confirming the predicate is opaque and unverified. Four false positives: `write_seq_idle` correctly preserves reads at untouched addresses, `combine` flag intersection is correct x86 semantics, `memories_disjoint` strict ordering is definitional, and `contains_fst` existential instantiation works via concrete witness.

## True Positives (Spec Issues)

### valid_core_external_body_unconstrained
- **Confidence:** medium
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` body. Since it's opaque, Verus treats it as an uninterpreted predicate. Two `Core` values with identical `node_id` and `core_id` are structurally equal (Core has only those two fields), so Verus can prove `core1 == core2` and substitute. This actually verifies via structural equality, not the external_body — but it confirms `valid_core` is completely unconstrained, accepting any property derivable from its opaqueness.

## All Candidates

### φ1: write_seq_idle_allows_arbitrary_read_on_missing_key
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_idle` guarantees untouched addresses read the same — but `read` on an address not in `mem.dom()` returns an arbitrary value from the total Map; the lemma preserves this arbitrary value faithfully, which is correct but tests the interaction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq_idle` correctly preserves the read value for untouched addresses. The fact that `read` on an out-of-domain address returns an arbitrary (but deterministic per Map axioms) value is standard vstd Map semantics. The lemma faithfully preserves this — no spec gap.

### φ2: flags_combine_chain_loses_writable
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `combine` uses AND for `is_writable` — a single non-writable entry in the chain kills writability for all deeper levels; tests that the most restrictive flag propagation is correct
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` defines `is_writable: self.is_writable && other.is_writable`. With `!f2.is_writable`, `f1.combine(f2).is_writable` is `true && false == false`, and `false && f3.is_writable == false`. This is correct x86 semantics — permission flags are intersected down the page table hierarchy.

### φ3: memories_disjoint_strict_ordering
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `memories_disjoint` uses strict ordering `range_mem.1 < range_ptmem.0` — tests that physical memory and page table memory regions are truly non-overlapping; if this were `<=` instead, zero-size gaps could allow aliasing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `memories_disjoint` is defined with strict ordering `range_mem.0 < range_mem.1 < range_ptmem.0 < range_ptmem.1`. The ensures directly follow from unfolding this conjunction. This is correct — strict ordering ensures non-empty, non-overlapping regions.

### φ4: valid_core_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_core` is `external_body` — tests whether structurally equal cores are interchangeable; since Core has no invariants beyond its fields, this should hold but is unverified
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `valid_core` is `external_body` with `unimplemented!()` body. Since it's opaque, Verus treats it as an uninterpreted predicate. Two `Core` values with identical `node_id` and `core_id` are structurally equal (Core has only those two fields), so Verus can prove `core1 == core2` and substitute. This actually verifies via structural equality, not the external_body — but it confirms `valid_core` is completely unconstrained, accepting any property derivable from its opaqueness.

### φ5: seq_tup_ext_contains_fst_witness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `contains_fst` uses an existential with trigger `self[i] == (fst, self[i].1)` — tests that a concrete witness at index 0 can instantiate the existential; if the trigger is too restrictive, this simple case might not verify
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `contains_fst` is `exists|i| 0 <= i < self.len() && self[i] == (fst, self[i].1)`. With `s[0] == (addr, val)`, witness `i=0` gives `s[0] == (addr, s[0].1)` which is `(addr, val) == (addr, val)`. The trigger `self[i]` at index 0 fires naturally. This is correct existential instantiation.

