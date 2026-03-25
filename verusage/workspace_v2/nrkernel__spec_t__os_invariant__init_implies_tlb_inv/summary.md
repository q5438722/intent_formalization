# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__init_implies_tlb_inv.rs`
**Date:** 2026-03-24T14:39:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives targeting the refinement chain: `init_refines` is an `external_body` proof asserting rl3→rl1 refinement without verification, and `rl3::init` combined with `rl3::State::interp` are both `external_body`/`closed spec`, making the entire initial state refinement an unverified trust assumption. Two false positives: PML4 region allocation and empty PML4 entries are direct conjuncts of the `os::init` predicate.

## True Positives (Spec Issues)

### init_refines_external_body_unconstrained
- **Confidence:** high
- **Reasoning:** `init_refines` is `external_body` with `unimplemented!()` body — it asserts the refinement `rl3::init(pre, c) ==> rl1::init(pre@, c)` without proof. This is an unverified trust assumption at the foundation of the rl3→rl2→rl1 abstraction chain.

### rl3_init_external_body_unconstrained
- **Confidence:** high
- **Reasoning:** This chains two `external_body` functions: `rl3::init` (closed spec, opaque precondition) and `init_refines` (external_body proof). The `happy` property of the rl1 interpretation is derived entirely through unverified trust assumptions — both the rl3 init predicate and the refinement proof are opaque.

## All Candidates

### φ1: init_refines_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `init_refines` is `external_body` — it asserts rl3 init refines to rl1 init without proof; if the refinement is wrong, the entire rl3→rl1 abstraction chain is unsound
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `init_refines` is `external_body` with `unimplemented!()` body — it asserts the refinement `rl3::init(pre, c) ==> rl1::init(pre@, c)` without proof. This is an unverified trust assumption at the foundation of the rl3→rl2→rl1 abstraction chain.

### φ2: rl3_init_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `rl3::init` is `closed spec` `external_body` — its precondition is opaque, so `init_refines` can derive any property of the rl1 interpretation without the rl3 init condition being verifiably checked
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This chains two `external_body` functions: `rl3::init` (closed spec, opaque precondition) and `init_refines` (external_body proof). The `happy` property of the rl1 interpretation is derived entirely through unverified trust assumptions — both the rl3 init predicate and the refinement proof are opaque.

### φ3: rl3_interp_external_body_no_constraint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` `closed spec` — structurally equal rl3 states should produce equal rl2 interpretations, but the interp function is completely opaque and unverified

### φ4: os_init_pml4_region_in_allocated
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** OS init requires the PML4 region is in `allocated` — tests that the allocated set is correctly initialized; if wrong, the PML4 page could be reallocated for other uses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `os::init` explicitly requires `s.os_ext.allocated

### φ5: os_init_empty_pml4_entries
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** OS init requires all 512 PML4 entries are zero — tests that the page table starts empty; non-zero entries at init would create phantom mappings before the OS sets up its address space
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `os::init` explicitly requires `forall|i: usize| 0 <= i < 512 ==> s.mmu@.pt_mem.read(add(s.mmu@.pt_mem.pml4, mul(i, 8))) == 0`. The ensures is a direct restatement of this conjunct.

