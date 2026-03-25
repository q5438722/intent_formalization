# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_writer_read_from_sbuf/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: only_zero_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, aligned would be trivially unsatisfiable for any nonzero address, making all page-table alignment constraints vacuous.

### φ2: 4k_implies_8k_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 4K-alignment must not entail 8K-alignment; if provable, the pml4 alignment invariant would silently impose a stronger-than-intended constraint.

### φ3: unique_alignment_size → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An address can be aligned to multiple sizes (e.g., 0 is aligned to every size); if provable, aligned would be inconsistent.

### φ4: self_alignment_false → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Every nonzero n satisfies n % n == 0, so aligned(n,n) must be true; if refuted, the modulus definition is broken.

### φ5: aligned_bounded_by_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligned addresses are unbounded multiples of the alignment; if provable, any address beyond 4096 would be wrongly excluded from valid page-table bases.

