# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l2_impl/impl_u__l2_impl__lemma_aligned_addr_mask_facts.rs`
**Date:** 2026-03-24T13:00:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `lemma_page_aligned_implies_mask_dir_addr_is_identity` and `aligned_transitive` are both `external_body` axioms trusting correct but unproven bitwise/arithmetic properties, and `MAX_PHYADDR_WIDTH` as `external_body` with a weaker axiom than its implementation value creates a specification gap. Two false positives: the L3 mask equality is tautological by definition, and the L1 mask identity is a verified downstream consequence.

## True Positives (Spec Issues)

### page_aligned_implies_mask_identity_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_page_aligned_implies_mask_dir_addr_is_identity` is `external_body` with `unimplemented!()` body — the bitwise property that page-aligned addresses within MAX_PHYADDR are unchanged by masking with MASK_DIR_ADDR is trusted without proof. This is the root axiom for all the mask identity proofs in the file.

### aligned_transitive_external_body
- **Confidence:** medium
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — alignment transitivity (`a % b == 0 ∧ b % c == 0 → a % c == 0`) is a correct arithmetic fact but trusted without proof. Used by `lemma_aligned_addr_mask_facts` to chain L1/L2/L3 alignment down to PAGE_SIZE alignment.

### max_phyaddr_width_external_body_const
- **Confidence:** medium
- **Reasoning:** `MAX_PHYADDR_WIDTH` is declared `external_body` with value 52, but the axiom only constrains it to `32..=52`. The verifier treats it as an opaque value satisfying the axiom rather than the concrete 52, creating a gap where proofs depending on the exact value of 52 would fail while the implementation assumes 52.

## All Candidates

### φ1: page_aligned_implies_mask_identity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_page_aligned_implies_mask_dir_addr_is_identity` is `external_body` — the bitwise identity for page-aligned addresses masked with MASK_DIR_ADDR is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_page_aligned_implies_mask_dir_addr_is_identity` is `external_body` with `unimplemented!()` body — the bitwise property that page-aligned addresses within MAX_PHYADDR are unchanged by masking with MASK_DIR_ADDR is trusted without proof. This is the root axiom for all the mask identity proofs in the file.

### φ2: aligned_transitive_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `aligned_transitive` is `external_body` — alignment transitivity is a correct arithmetic fact but trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `aligned_transitive` is `external_body` with `unimplemented!()` body — alignment transitivity (`a % b == 0 ∧ b % c == 0 → a % c == 0`) is a correct arithmetic fact but trusted without proof. Used by `lemma_aligned_addr_mask_facts` to chain L1/L2/L3 alignment down to PAGE_SIZE alignment.

### φ3: mask_l3_pg_addr_equals_mask_addr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** MASK_L3_PG_ADDR and MASK_ADDR are both `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)` — they are identical by definition, making this tautological rather than a meaningful spec property
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `MASK_L3_PG_ADDR` and `MASK_ADDR` are both defined as `bitmask_inc!(12, MAX_PHYADDR_WIDTH - 1)`, so they are definitionally equal. The equality `addr & MASK_L3_PG_ADDR == addr & MASK_ADDR` is a tautology — correct and expected behavior by design.

### φ4: max_phyaddr_width_external_body_const
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` const with value 52 but the axiom only guarantees 32..=52 — the spec is weaker than the implementation, leaving room for inconsistency if the actual value differs
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is declared `external_body` with value 52, but the axiom only constrains it to `32..=52`. The verifier treats it as an opaque value satisfying the axiom rather than the concrete 52, creating a gap where proofs depending on the exact value of 52 would fail while the implementation assumes 52.

### φ5: l1_aligned_mask_addr_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** L1-aligned addresses within MAX_PHYADDR are identity under MASK_ADDR — depends on both `aligned_transitive` and `lemma_page_aligned_implies_mask_dir_addr_is_identity` external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified consequence of `lemma_aligned_addr_mask_facts`, which has a complete proof body. The trust gaps are already captured by φ1 and φ2; this adds no new unverified assumption.

