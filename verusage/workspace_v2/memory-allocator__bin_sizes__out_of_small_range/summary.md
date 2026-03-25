# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__out_of_small_range.rs`
**Date:** 2026-03-24T10:28:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: lemma_bin_sizes_constants_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_bin_sizes_constants` is `external_body` with `unimplemented!()` — all 146 ensures clauses (73 bin sizes + 73 division results) are trusted without proof, and if any single value is wrong the entire allocator reasoning is unsound

### φ2: size_of_bin_outside_valid_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `size_of_bin` has only a `recommends` (not `requires`) for `valid_bin_idx`, so it's callable on out-of-range indices — `size_of_bin(0)` falls through all branches to the group/inner formula with bin_idx=0, producing `(8 * ((-9)%4 + 5) * pow2((-9)/4 + 1))` with negative intermediate values

### φ3: pfd_lower_bin1_is_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Bin 1 has lower bound 0 meaning it accepts zero-sized allocations — the allocator would serve a zero-byte request from bin 1 (size 8), which may mask bugs where callers pass size 0

### φ4: bin_huge_size_not_monotone
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The jump from bin 72 (4194304 = 4MB) to bin 73 (4194312 = 4MB+8) is only 8 bytes — bin 73 is BIN_HUGE meant to represent "infinite" but its artificial upper bound is barely above bin 72, breaking the exponential growth pattern

### φ5: out_of_small_range_gap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When `size_of_bin(bin_idx)` equals exactly `SMALL_SIZE_MAX` (1024), it doesn't satisfy the strict `>` in `out_of_small_range`'s requires — the boundary bin (bin 24, size 1024) is classified as "small" even though `SMALL_SIZE_MAX = 128 * 8 = 1024`, potentially allowing it to bypass the `PAGES_DIRECT` check

