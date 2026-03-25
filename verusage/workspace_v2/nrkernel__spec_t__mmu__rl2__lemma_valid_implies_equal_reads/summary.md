# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_valid_implies_equal_reads.rs`
**Date:** 2026-03-24T14:09:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `lemma_write_seq_idle` is an external_body frame lemma trusting that writes to disjoint addresses preserve reads without proof, and `lemma_valid_implies_equal_reads` transitively depends on this unverified assumption for its TSO consistency guarantee. Three false positives: non-writer core_mem identity follows from fold_left base case, contains_fst decomposition is correct by tuple projection, and the contrapositive of the sbuf invariant is a standard logical equivalence.

## True Positives (Spec Issues)

### lemma_write_seq_idle_external_body
- **Confidence:** high
- **Reasoning:** `lemma_write_seq_idle` is `external_body` with `unimplemented!()` — the frame property (writes to disjoint addresses don't affect reads) requires an inductive proof over `fold_left` that is not verified. This is a key lemma used by `lemma_valid_implies_equal_reads`.

### valid_implies_equal_reads_uses_external_body
- **Confidence:** medium
- **Reasoning:** This is a transitive trust assumption — `lemma_valid_implies_equal_reads` is a verified proof, but its body calls `lemma_write_seq_idle` which is `external_body`. The TSO consistency guarantee is only as strong as the unverified frame lemma.

## All Candidates

### φ1: lemma_write_seq_idle_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_idle` is `external_body` — the frame property that writes to other addresses don't affect a read is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq_idle` is `external_body` with `unimplemented!()` — the frame property (writes to disjoint addresses don't affect reads) requires an inductive proof over `fold_left` that is not verified. This is a key lemma used by `lemma_valid_implies_equal_reads`.

### φ2: non_writer_core_mem_equals_pt_mem
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Non-writer cores have empty sbufs, so `core_mem(core) == pt_mem.write_seq(seq![]) == pt_mem` — tests that empty sbuf gives identity, but this relies on fold_left base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `non_writer_sbufs_are_empty` gives `self.sbuf[core]

### φ3: valid_implies_equal_reads_uses_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The proof of equal reads between non-writer and writer core depends on `lemma_write_seq_idle` (external_body) — the entire TSO consistency argument chains through an unverified lemma
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This is a transitive trust assumption — `lemma_valid_implies_equal_reads` is a verified proof, but its body calls `lemma_write_seq_idle` which is `external_body`. The TSO consistency guarantee is only as strong as the unverified frame lemma.

### φ4: contains_fst_implies_exists_entry
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `contains_fst` uses `s[i] == (fst, s[i].1)` which should imply `s[i].0 == fst` — tests that the existential witness decomposition is correct
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `contains_fst` is defined as `exists|i| 0 <= i < self.len() && self[i] == (fst, self[i].1)`. If `self[i] == (fst, self[i].1)`, then `self[i].0 == fst` by tuple projection. This is a correct logical consequence of the definition.

### φ5: inv_mapping_valid_not_in_sbuf_contrapositive
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** By contrapositive of `inv_mapping__valid_is_not_in_sbuf`, if addr is in writer sbuf then non-writer core reads must have P bit unset — tests whether the invariant correctly constrains staleness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the straightforward contrapositive of `inv_mapping__valid_is_not_in_sbuf`: the invariant says `P_bit_set ==> !in_sbuf`, so `in_sbuf ==> !P_bit_set`. This is a correct logical transformation, not a spec gap.

