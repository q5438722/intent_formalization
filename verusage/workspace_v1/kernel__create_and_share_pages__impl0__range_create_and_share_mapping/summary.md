# Adversarial Proof Test Summary

**Target**: `kernel__create_and_share_pages__impl0__range_create_and_share_mapping.rs`

## Results: All 15 tests FAIL verification ✅

The specification correctly rejects all adversarial queries — no weakness detected.

| # | Category | Test | Result |
|---|----------|------|--------|
| B1 | Boundary | VA=0 not 4k-valid | FAIL ✅ |
| B2 | Boundary | Zero quota insufficient | FAIL ✅ |
| B3 | Boundary | Quota subtract underflow | FAIL ✅ |
| B4 | Boundary | Unaligned page ptr invalid | FAIL ✅ |
| B5 | Boundary | Page index at NUM_PAGES OOB | FAIL ✅ |
| M1 | Mutation | Quota unchanged after subtract | FAIL ✅ |
| M2 | Mutation | Wrong subtraction amount | FAIL ✅ |
| M3 | Mutation | mem_2m changed during 4k subtract | FAIL ✅ |
| M4 | Mutation | Non-zero addr PageEntry not empty | FAIL ✅ |
| M5 | Mutation | pcid changed during 4k subtract | FAIL ✅ |
| L1 | Logical | ret not always 3*len | FAIL ✅ |
| L2 | Logical | free ≠ shareable | FAIL ✅ |
| L3 | Logical | Quota subtract not reversible | FAIL ✅ |
| L4 | Logical | Subtraction result unique | FAIL ✅ |
| L5 | Logical | 4k-valid ≠ 2m-valid | FAIL ✅ |

## Interpretation

- **Boundary tests**: The spec correctly rejects invalid VA addresses, insufficient quota, quota underflow, unaligned page pointers, and out-of-bound page indices.
- **Behavioral mutation tests**: The spec correctly rejects mutated postconditions — unchanged quota after subtraction, wrong subtraction amounts, side-channel modifications to unrelated quota fields (mem_2m, pcid), and falsely empty page entries.
- **Logical tests**: The spec correctly rejects unintended logical inferences — return value determinism assumptions, cross-function misuse (free↔shareable confusion), non-reversibility of subtraction, uniqueness of subtraction results, and false 4k→2m validity implications.

**Conclusion**: The specification of `range_create_and_share_mapping` is consistent with respect to all 15 adversarial queries tested. No unintended entailments were found.
