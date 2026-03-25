# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_interp_of_entry_insert_page_implies_interp_insert_page.rs`
**Date:** 2026-03-24T12:56:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: the `external_body` insert propagation axiom is trusted without proof, the commented-out base_vaddr alignment allows misaligned directory bases, and the commented-out frames_aligned allows arbitrary physical frame bases. Two false positives: the overwrite behavior is a standard consequence of Map::insert semantics, and the page entry containing its base key is trivially correct by open spec definition.

## True Positives (Spec Issues)

### insert_page_implies_interp_aux_insert_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that inserting a mapping into one entry's interpretation propagates correctly through the `interp_aux` union chain is trusted without proof.

### inv_no_base_vaddr_alignment
- **Confidence:** high
- **Reasoning:** The commented-out `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed()` means `inv()` accepts any `base_vaddr` including misaligned values like 7. On real x86 page tables, directory base virtual addresses must be aligned to entry boundaries.

### page_frame_base_unconstrained
- **Confidence:** medium
- **Reasoning:** `inv()` via `pages_match_entry_size` constrains `frame.size == entry_size()` but has no constraint on `frame.base`. The commented-out `self.frames_aligned()` suggests frame alignment was intended but disabled — any arbitrary physical address is accepted as a valid page frame base.

## All Candidates

### φ1: insert_page_implies_interp_aux_insert_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` — the inductive proof that single-entry insert propagates through `interp_aux` is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that inserting a mapping into one entry's interpretation propagates correctly through the `interp_aux` union chain is trusted without proof.

### φ2: page_insert_overwrites_existing_mapping
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The commented-out `!self.interp().contains_key(base)` means the axiom allows silently overwriting an existing mapping — a page insert can clobber a prior mapping at the same VA
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the same external_body axiom captured by φ1 combined with standard `Map::insert` semantics. The commented-out precondition was intentionally removed — `Map::insert` overwrites by definition, and the axiom correctly models this.

### φ3: page_insert_base_not_entry_base
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** After inserting a Page at slot j, the new entry's interp maps `entry_base(j)` to the PTE — but nothing constrains that `base` in the insert precondition must equal `entry_base(j)`, allowing mismatched VA-to-slot mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by open spec unfolding: `NodeEntry::Page(pte).interp(entry_base(j))` returns `map![entry_base(j) => pte]`, so the singleton map trivially contains `entry_base(j)`. The `base` parameter in the insert lemma is separate from this — no spec gap here.

### φ4: inv_no_base_vaddr_alignment
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The commented-out alignment constraint means `inv()` accepts any `base_vaddr` including misaligned values like 7
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The commented-out `aligned(self.base_vaddr, self.entry_size() * self.num_entries())` in `well_formed()` means `inv()` accepts any `base_vaddr` including misaligned values like 7. On real x86 page tables, directory base virtual addresses must be aligned to entry boundaries.

### φ5: page_frame_base_unconstrained
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv()` constrains page frame *size* but not frame *base* — any physical address (e.g. 0xDEAD, unaligned) is accepted as a valid page frame base
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `inv()` via `pages_match_entry_size` constrains `frame.size == entry_size()` but has no constraint on `frame.base`. The commented-out `self.frames_aligned()` suggests frame alignment was intended but disabled — any arbitrary physical address is accepted as a valid page frame base.

