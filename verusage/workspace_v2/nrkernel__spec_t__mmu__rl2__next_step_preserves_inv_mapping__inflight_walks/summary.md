# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_mapping__inflight_walks.rs`
**Date:** 2026-03-24T14:28:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: both `lemma_view_addr_aligned` candidates (Directory and Page variants) expose the same `external_body` trust assumption — address alignment and upper bounds are asserted without proof. Two false positives: `write_seq` value semantics follow from open spec unfolding, and chained pml4 preservation is a straightforward transitive application of the external_body ensures. Note: the φ_5 candidate (`bit_indices_less_512_implies_walk_addr_bounded`) was not included in the verified set for this critic round.

## True Positives (Spec Issues)

### lemma_view_addr_aligned_directory_below_max
- **Confidence:** high
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` with `unimplemented!()` body. It asserts Directory addresses extracted via `entry & MASK_ADDR` are page-aligned and bounded. While `MASK_ADDR = bitmask_inc!(12, 51)` does zero bits 0-11 (ensuring 4096-alignment), the upper bound `< usize::MAX - 4096` and the alignment claims are trusted without proof.

### lemma_view_addr_aligned_page_addr_8_aligned
- **Confidence:** high
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` asserting Page addresses are 8-byte aligned and bounded. Page_addr is extracted via layer-specific masks: `MASK_L1_PG_ADDR` (bits 30+), `MASK_L2_PG_ADDR` (bits 21+), `MASK_L3_PG_ADDR` (bits 12+). All zero bits 0-11 at minimum, so 4096-alignment holds, but the bound `< usize::MAX - 4096` is an unverified trust assumption.

## All Candidates

### φ1: write_seq_dom_grows_but_no_value_constraint
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` only constrains pml4 and domain monotonicity — it says nothing about the *values* written; tests whether last-write-wins is provable without the lemma (it should be, from open spec unfolding)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write_seq` and `write` are open specs. `write_seq(seq![(addr, val2)])` unfolds to `mem.write(addr, val2)` which sets `mem[addr] = val2`. The double write similarly results in `val2` at `addr`. This is provable purely by unfolding the open spec definitions — no external_body lemma needed.

### φ2: lemma_view_addr_aligned_directory_below_max
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view_addr_aligned` is `external_body` — it asserts Directory addresses are page-aligned and bounded, but since Directory_addr is extracted via `entry & MASK_ADDR`, this alignment depends on MASK_ADDR zeroing bits 0-11, which is unverified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` with `unimplemented!()` body. It asserts Directory addresses extracted via `entry & MASK_ADDR` are page-aligned and bounded. While `MASK_ADDR = bitmask_inc!(12, 51)` does zero bits 0-11 (ensuring 4096-alignment), the upper bound `< usize::MAX - 4096` and the alignment claims are trusted without proof.

### φ3: write_seq_preserves_pml4_chained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Chaining two `lemma_write_seq` calls to prove double write_seq preserves pml4 — relies on the external_body lemma being compositional; if the lemma's ensures are too weak, chaining could fail
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The first `lemma_write_seq` call gives `mem.write_seq(w1).pml4 == mem.pml4`, the second gives `mem.write_seq(w1).write_seq(w2).pml4 == mem.write_seq(w1).pml4`. Transitivity yields the result. This is a straightforward composition of the external_body ensures — it doesn't expose any gap beyond what `lemma_write_seq` already claims.

### φ4: lemma_view_addr_aligned_page_addr_8_aligned
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_view_addr_aligned` asserts Page addresses are 8-byte and 4096-byte aligned and bounded — but Page_addr is extracted via layer-specific masks (MASK_L1/L2/L3_PG_ADDR) which zero different low bits; the 8-byte alignment claim is unverified for all layers
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_view_addr_aligned` is `external_body` asserting Page addresses are 8-byte aligned and bounded. Page_addr is extracted via layer-specific masks: `MASK_L1_PG_ADDR` (bits 30+), `MASK_L2_PG_ADDR` (bits 21+), `MASK_L3_PG_ADDR` (bits 12+). All zero bits 0-11 at minimum, so 4096-alignment holds, but the bound `< usize::MAX - 4096` is an unverified trust assumption.

### φ5: bit_indices_less_512_implies_walk_addr_bounded
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `lemma_bit_indices_less_512` is `external_body` — it asserts all bit-index extractions produce values < 512; combined with WORD_SIZE=8, this bounds L0 walk addresses within a 4KB page, but the bit extraction correctness is unverified

