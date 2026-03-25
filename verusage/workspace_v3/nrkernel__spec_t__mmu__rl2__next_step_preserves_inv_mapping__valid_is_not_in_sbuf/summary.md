# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_mapping__valid_is_not_in_sbuf/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment rounds down, so the result must never exceed the input

### φ2: align_always_zero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A trivially-zero alignment function would satisfy divisibility but discard address information

### φ3: align_identity_when_unaligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** When the input is not aligned, the result must differ from the input; identity here means no rounding occurred

### φ4: aligned_implies_zero → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Many non-zero addresses are aligned; if only zero satisfies alignment the spec is degenerate

### φ5: aligned_false_compose → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to b and c individually does not imply alignment to b*c (e.g. 12 is aligned to 4 and 6 but not 24)

