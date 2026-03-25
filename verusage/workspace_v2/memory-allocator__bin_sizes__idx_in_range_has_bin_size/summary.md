# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__idx_in_range_has_bin_size.rs`
**Date:** 2026-03-24T10:25:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 5

## Summary

All five candidates are false positives — they verify essential allocator correctness invariants: bin sizes are monotone and word-aligned, bins have non-empty ranges with contiguous coverage, and the smallest-bin-fitting-size function returns bins large enough to hold the requested size. These are desirable properties, not spec gaps.

## True Positives (Spec Issues)

### pfd_lower_upper_coverage
- **Confidence:** medium
- **Reasoning:** `pfd_lower(bin_idx) > pfd_upper(bin_idx - 1)` means there's a gap: word sizes equal to `pfd_upper(bin_idx - 1) + 1` through `pfd_lower(bin_idx) - 1` fall in neither bin. Specifically, `pfd_lower(n) = size_of_bin(n-1)/8 + 1` while `pfd_upper(n-1) = size_of_bin(n-1)/8`, so the gap is exactly zero (off by the `+1`). Wait — `pfd_lower(n) = pfd_upper(n-1) + 1`, which means they're contiguous with no overlap, which is actually correct for exclusive lower / inclusive upper bounds. This is the expected coverage property.

## All Candidates

### φ1: smallest_bin_fitting_size_roundtrip
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If the smallest bin fitting a size couldn't actually hold that size, the allocator would return undersized blocks
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This confirms the allocator's fundamental correctness property — the bin returned for a given size can actually hold that size. This is a desirable soundness check, not a spec gap.

### φ2: pfd_lower_upper_coverage
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If lower bound of bin N exceeds upper bound of bin N-1, there's a gap — word sizes between bins would have no valid bin, causing allocation failures
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pfd_lower(bin_idx) > pfd_upper(bin_idx - 1)` means there's a gap: word sizes equal to `pfd_upper(bin_idx - 1) + 1` through `pfd_lower(bin_idx) - 1` fall in neither bin. Specifically, `pfd_lower(n) = size_of_bin(n-1)/8 + 1` while `pfd_upper(n-1) = size_of_bin(n-1)/8`, so the gap is exactly zero (off by the `+1`). Wait — `pfd_lower(n) = pfd_upper(n-1) + 1`, which means they're contiguous with no overlap, which is actually correct for exclusive lower / inclusive upper bounds. This is the expected coverage property.

### φ3: size_of_bin_monotone
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If bin sizes weren't monotonically increasing, the binary-search-like logic in smallest_bin_fitting_size would return incorrect bins
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Monotonicity of bin sizes is a fundamental design invariant for the allocator's bin selection logic. This is a desirable property confirmed by the concrete constant values.

### φ4: size_of_bin_divisible_by_8
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** All bin sizes should be word-aligned (divisible by 8) — if not, allocations would return misaligned blocks causing undefined behavior on architectures requiring alignment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Word-alignment (8-byte divisibility) of all bin sizes is an essential allocator invariant ensuring returned blocks satisfy alignment requirements. Confirmed by all 73 concrete bin size values.

### φ5: pfd_lower_leq_upper
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If the lower bound of a bin exceeded its upper bound, the bin would represent an empty range of word sizes — no allocation could ever use that bin
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Each bin must represent a non-empty range of word sizes. `pfd_lower <= pfd_upper` confirms every bin can serve at least one word-size class — a fundamental allocator correctness property.

