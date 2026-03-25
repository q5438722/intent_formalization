# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_wf.rs`
**Date:** 2026-03-24T14:33:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `PTMem::view` is `external_body` and completely unconstrained — while structural equality makes the proof trivial, the real issue is that `view` has no verified connection to `pt_walk` or any other spec, meaning it could return arbitrary mappings. Three false positives: alignment requirements from `is_base_pt_walk` follow from `align_to_usize` semantics, and `MAX_BASE` correctly equals 2^48.

## True Positives (Spec Issues)

### ptmem_view_external_body_no_domain_constraint
- **Confidence:** high
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` body. Two structurally equal PTMem values (`mem1.pml4 == mem2.pml4` and `mem1.mem

## All Candidates

### φ1: is_base_pt_walk_requires_aligned_vaddr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `is_base_pt_walk` requires `vbase == vaddr` and `vbase = align_to_usize(vaddr, L3_ENTRY_SIZE)` for 4-level walks — so `vaddr` must already be page-aligned; if this holds for non-aligned vaddrs, the alignment rounding is broken
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_base_pt_walk` requires `vbase == vaddr` where `vbase = align_to_usize(vaddr, L3_ENTRY_SIZE)`. Since `align_to_usize(a, b) = a - (a % b)`, `a == a - (a % b)` implies `a % b == 0`, which is `aligned(a, b)`. This is correct by construction.

### φ2: is_base_pt_walk_l1_requires_1g_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** For 2-level walks (1GB pages), `is_base_pt_walk` forces `vaddr == align_to_usize(vaddr, L1_ENTRY_SIZE)` — vaddr must be 1GB-aligned; tests that huge page base walk semantics correctly constrain alignment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same reasoning as above: for path length 2, `vbase = align_to_usize(vaddr, L1_ENTRY_SIZE)` and `vbase == vaddr` forces `vaddr % L1_ENTRY_SIZE == 0`. Correct by construction.

### φ3: ptmem_view_external_body_no_domain_constraint
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — two PTMem values with identical fields should produce identical views, but this is unverified; if `view` could differ for structurally equal inputs, the abstraction would be unsound
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` body. Two structurally equal PTMem values (`mem1.pml4 == mem2.pml4` and `mem1.mem

### φ4: walk_result_valid_implies_page_last
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `Walk::result` returns Valid only when `path.last().1 is Page` (the if-branch); otherwise it returns `arbitrary()` — tests that the converse holds: if result is Valid with reasonable path length, the last entry must be Page

### φ5: max_base_exceeds_48bit_vaddr_space
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `MAX_BASE = X86_MAX_ENTRY_SIZE * X86_NUM_ENTRIES = (512^3 * 4096) * 512 = 512^4 * 4096 = 2^48` — tests that the virtual address space bound is exactly 256TB (2^48), the canonical x86-64 limit; if wrong, `is_base_pt_walk` would accept or reject valid addresses incorrectly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `MAX_BASE = X86_MAX_ENTRY_SIZE * X86_NUM_ENTRIES` where `X86_MAX_ENTRY_SIZE = 512^3 * 4096` and `X86_NUM_ENTRIES = 512`. This equals `512^4 * 4096 = 2^48`. This is the correct canonical x86-64 virtual address space size.

