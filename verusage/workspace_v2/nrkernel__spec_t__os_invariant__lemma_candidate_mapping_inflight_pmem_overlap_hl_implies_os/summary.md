# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_candidate_mapping_inflight_pmem_overlap_hl_implies_os.rs`
**Date:** 2026-03-24T14:40:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the rl3→rl2 interp is `external_body` `closed spec`, making the entire `s@.pt_mem === s.interp().writer_mem()` chain an unverified trust assumption. Four false positives: overlap symmetry and zero-size non-overlap follow from case analysis of the definition, candidate_mapping individual bounds follow from nat arithmetic, and arch entry size layer 0 is correct by construction of `x86_arch_spec`.

## True Positives (Spec Issues)

### rl3_interp_external_body_view_chain
- **Confidence:** high
- **Reasoning:** `rl3::State::interp` is `external_body` `closed spec` — the equality `s@.pt_mem

## All Candidates

### φ1: rl3_interp_external_body_view_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `rl3::State::interp` is `external_body` `closed spec` — the rl1 view's `pt_mem` is set to `rl2.writer_mem()` by `rl2::interp`, but the rl3→rl2 interp is opaque; if it maps fields incorrectly, the entire refinement chain's pt_mem is wrong
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `rl3::State::interp` is `external_body` `closed spec` — the equality `s@.pt_mem

### φ2: overlap_not_symmetric
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `overlap` has an asymmetric definition (splits on `region1.base <= region2.base`) — if the two branches don't correctly mirror each other, overlap could be non-symmetric, breaking mapping conflict detection
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `overlap` splits on `region1.base <= region2.base`. When swapped, the other branch applies. Case analysis: if `r1.base <= r2.base` and overlap holds (either `r1.base == r2.base` or `r2.base < r1.base + r1.size`), then for `overlap(r2, r1)` we're in the else branch (`r2.base >= r1.base` doesn't hold if `r2.base < r1.base`, but in this case we had `r1.base <= r2.base`). The symmetry follows from correct case splitting in both directions.

### φ3: overlap_zero_size_region
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero-size regions at different bases should never overlap — if the overlap function returns true for empty regions, phantom conflicts would block valid mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `base1 != base2` and `size: 0`, WLOG `base1 < base2`: the first branch checks `base1 == base2` (false) or `base2 < base1 + 0 = base1` (false since `base1 < base2`). Similarly for `base1 > base2`. Non-overlap is correct for distinct-base zero-size regions.

### φ4: candidate_mapping_in_bounds_allows_overflow
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `candidate_mapping_in_bounds` checks `base + pte.frame.size < upper_vaddr` using strict `<` — tests whether base and size are individually bounded; if frame.size could be enormous with a small base, the check passes but the mapping spans unreasonable ranges
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `base + pte.frame.size < upper_vaddr` with nat arithmetic (no overflow) directly implies both `base < upper_vaddr` (since `pte.frame.size >= 0`) and `pte.frame.size < upper_vaddr` (since `base >= 0`). This is correct — the sum bound implies individual bounds over naturals.

### φ5: arch_entry_size_layer0_equals_l0
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Layer 0 entry size should match `L0_ENTRY_SIZE` (512 * L1_ENTRY_SIZE) — if the arch spec layers disagree with the constants, walk result sizes and alignment would be incorrect
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x86_arch_spec` is defined with layer 0 having `entry_size: L0_ENTRY_SIZE as nat`. `entry_size(0)` returns `self.layers[0].entry_size` which is exactly `L0_ENTRY_SIZE as nat`. Correct by construction.

