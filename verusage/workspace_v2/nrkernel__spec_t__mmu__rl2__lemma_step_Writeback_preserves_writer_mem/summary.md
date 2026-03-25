# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_Writeback_preserves_writer_mem.rs`
**Date:** 2026-03-24T13:58:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `lemma_write_seq_first` is an external_body lemma trusting the fold_left decomposition without proof, and the writeback writer memory preservation depends transitively on this unverified lemma. Three false positives: writeback non-interference at other addresses follows from Map::insert, non-writer core global visibility is correct TSO semantics, and store buffer address uniqueness is a restatement of the existing invariant.

## True Positives (Spec Issues)

### lemma_write_seq_first_external_body
- **Confidence:** high
- **Reasoning:** `lemma_write_seq_first` is `external_body` with `unimplemented!()` — the decomposition of `fold_left` into processing the first element then the rest is trusted without proof. This is a key lemma used in `lemma_step_Writeback_preserves_writer_mem`.

### writeback_preserves_writer_mem_via_external
- **Confidence:** medium
- **Reasoning:** The writer memory preservation proof depends directly on the unverified `lemma_write_seq_first`. While the property itself is correct and desirable, the proof chain is only as strong as the external_body lemma — making this a transitive trust assumption.

## All Candidates

### φ1: lemma_write_seq_first_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_first` is `external_body` with `unimplemented!()` — the inductive decomposition of `write_seq` (popping the first element) is trusted without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_write_seq_first` is `external_body` with `unimplemented!()` — the decomposition of `fold_left` into processing the first element then the rest is trusted without proof. This is a key lemma used in `lemma_step_Writeback_preserves_writer_mem`.

### φ2: writeback_preserves_writer_mem_via_external
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writer memory preservation across writeback depends on the unverified `lemma_write_seq_first` — if the fold decomposition is wrong, writeback could silently change the writer's view
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The writer memory preservation proof depends directly on the unverified `lemma_write_seq_first`. While the property itself is correct and desirable, the proof chain is only as strong as the external_body lemma — making this a transitive trust assumption.

### φ3: writeback_only_changes_pt_mem_at_addr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writeback should only modify the drained address — tests that `Map::insert` doesn't affect other keys in the global page table memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from `Map::insert` non-interference — inserting at one key doesn't affect other keys. This is a standard vstd map axiom and correct behavior.

### φ4: writeback_non_writer_core_empty_sbuf
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** After writeback, non-writer cores (with empty sbufs) see the newly committed value — tests that writeback makes the write globally visible to other cores
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Non-writer cores have empty store buffers (by `non_writer_sbufs_are_empty`), so `core_mem(other_core) == pt_mem.write_seq(seq![]) == post.pt_mem`. After writeback, `post.pt_mem == pre.pt_mem.write(addr, value)`, so `post.pt_mem.mem[addr] == value`. This is correct TSO global visibility semantics.

### φ5: writer_sbuf_unique_implies_no_duplicate_addrs
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `writer_sbuf_entries_are_unique` should enforce injectivity of addresses — tests that duplicate addresses in the store buffer are impossible
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the contrapositive of `writer_sbuf_entries_are_unique` — if `i != j` implies different addresses, then same address implies `i == j`. This is correct and is simply restating the invariant in a different logical form.

