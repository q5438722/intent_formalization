# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_interp_of_entry_insert_implies_interp_insert.rs`
**Date:** 2026-03-24T12:55:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `lemma_interp_of_entry_insert_implies_interp_aux_insert` is an `external_body` axiom trusting the insert propagation through `interp_aux` without proof, and the commented-out alignment constraint allows directories with misaligned `base_vaddr` values to satisfy `inv()`. The other three are false positives — a redundant consequence of the same external_body, a trivial open spec unfolding for `update`, and a verified wrapper.

## True Positives (Spec Issues)

### insert_implies_interp_aux_insert_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that inserting a mapping into one entry's interpretation propagates correctly through `interp_aux`'s `union_prefer_right` chain is trusted without proof.

### inv_no_base_vaddr_alignment
- **Confidence:** high
- **Reasoning:** The commented-out `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed()` means `inv()` accepts any `base_vaddr`, including misaligned values like 7. On real x86 page tables, directory base addresses must be aligned to entry boundaries.

## All Candidates

### φ1: insert_implies_interp_aux_insert_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` — the inductive proof that single-entry map insert propagates through `interp_aux` is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that inserting a mapping into one entry's interpretation propagates correctly through `interp_aux`'s `union_prefer_right` chain is trusted without proof.

### φ2: insert_overwrites_existing_key
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The commented-out `!self.interp_aux(i).contains_key(base)` precondition means the external_body axiom allows silently overwriting an existing mapping at `base` — insert propagation holds even when `base` is already mapped
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of the same external_body axiom already captured by φ1, combined with standard `Map::insert` semantics. The commented-out precondition `!self.interp_aux(i).contains_key(base)` was removed intentionally — `Map::insert` overwrites by definition.

### φ3: update_preserves_other_entries
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `update` only modifies index j but `interp_of_entry(k)` uses `entry_base(k)` which depends on `base_vaddr` — tests that non-target entries are truly unaffected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `update` replaces only index `j` in the entries sequence via `Seq::update`, so `entries[k as int]` is unchanged for `k != j`. Since `entry_base(k)` depends on `base_vaddr` and `layer` (both preserved by `update`), `interp_of_entry(k)` is trivially equal. Correct by open spec unfolding.

### φ4: insert_interp_implies_insert_interp_public
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The public `lemma_interp_of_entry_insert_implies_interp_insert` directly wraps the external_body axiom — tests that the full interp-level consequence is derivable
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct downstream consequence of the external_body axiom already captured by φ1. The public wrapper `lemma_interp_of_entry_insert_implies_interp_insert` just calls the external_body with `i=0` — no new trust assumption.

### φ5: inv_no_base_vaddr_alignment
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The commented-out alignment constraint `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` means `inv()` accepts arbitrary `base_vaddr` like 7 — misaligned base addresses are allowed
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The commented-out `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed()` means `inv()` accepts any `base_vaddr`, including misaligned values like 7. On real x86 page tables, directory base addresses must be aligned to entry boundaries.

