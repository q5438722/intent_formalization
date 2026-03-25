# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__impl0__lemma_write_seq_idle.rs`
**Date:** 2026-03-24T13:29:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_write_seq` is an `external_body` lemma with unverified postconditions. Three false positives: write-then-read consistency is correct map semantics, the transitive external_body dependency is a duplicate of the direct finding, and PML4 field independence from memory writes is intentional design.

## True Positives (Spec Issues)

### lemma_write_seq_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` body — its postconditions (pml4 preservation and domain monotonicity) are trusted without proof. This is an unverified trust assumption in the proof chain.

## All Candidates

### φ1: lemma_write_seq_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` with `unimplemented!()` — its two postconditions are trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_write_seq` is `external_body` with `unimplemented!()` body — its postconditions (pml4 preservation and domain monotonicity) are trusted without proof. This is an unverified trust assumption in the proof chain.

### φ2: write_then_read_returns_written
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Write-then-read returns the written value — tests basic map insert/index consistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by construction — `write` inserts into the map, and `read` indexes into it. `Map::insert` followed by indexing at the same key returns the inserted value. This is standard map semantics, not a spec gap.

### φ3: read_uncontained_key_arbitrary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Reading an address not in the map domain returns the same `arbitrary()` value for any two PTMem instances — `Map::index` on missing keys returns a deterministic but unspecified value per key

### φ4: write_seq_idle_preserves_read_external_body_dep
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq_idle` depends on `lemma_write_seq` (external_body) in its proof body — the idle preservation guarantee transitively relies on an unverified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** While `lemma_write_seq_idle` does use `lemma_write_seq` (external_body) via broadcast, it only uses the domain subset property to ensure the key is still present. The actual read-preservation property is proved by induction. The transitive dependency is on a likely-correct property, and flagging downstream users of an already-flagged external_body is a duplicate.

### φ5: write_to_pml4_addr_no_effect_on_field
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing a different value to the address equal to `pml4` changes memory but not the `pml4` field — the PML4 base pointer diverges from what's stored at its own address
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pml4` is a fixed field representing the root page table base address, not a memory location that changes when memory is written. The PML4 base pointer is set at initialization and is independent of memory contents. This separation is intentional — the `pml4` field records which address to start the page walk from, not the value stored there.

