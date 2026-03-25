# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_obtain_bit_index_1_aux.rs`
**Date:** 2026-03-24T11:12:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Unlike the previous `external_body` version of this lemma, this file contains a fully verified proof using bit_vector assertions and structural recursion. The properties tested — existence of a set bit in nonzero values, boundary behavior at hi=1, and the i<64 bound — are all correct mathematical consequences of the verified spec with no trust gaps.

## All Candidates

### φ1: nonzero_has_set_bit
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any nonzero u64 has a set bit below 64 — this is mathematically correct and expected, serving as a sanity check that the lemma is usable at the top level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Any nonzero u64 has at least one set bit — this is a fundamental mathematical fact. The lemma is fully verified (not external_body), so this is a correct and expected consequence.

### φ2: hi_1_forces_bit_0
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With hi=1 the only possible index is 0 and a must be 1 — tests that the recursion base case is tight
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `a != 0` and `a >> 1 == 0`, the only possibility is `a == 1` with bit 0 set. The lemma returns `i < 1`, forcing `i == 0`. This is correct boundary behavior, not a spec gap.

### φ3: result_always_less_than_64
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Since `i < hi` and `hi <= 64`, the result is always below 64 — tests that the bound propagates correctly through the ensures
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures gives `i < hi` and the requires gives `hi <= 64`, so `i < 64` follows trivially by transitivity. This is a correct and expected logical consequence.

### φ4: bit_set_implies_nonzero_mask
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `is_bit_set(a, i)` should imply `a & (1 << i) != 0` — tests that the ensures is strong enough to derive nonzero mask

### φ5: power_of_two_returns_exact_bit
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For a=8 (2^3) the only set bit is bit 3 — if the lemma returns a different index, the is_bit_set ensures would be violated; tests uniqueness for power-of-two inputs

