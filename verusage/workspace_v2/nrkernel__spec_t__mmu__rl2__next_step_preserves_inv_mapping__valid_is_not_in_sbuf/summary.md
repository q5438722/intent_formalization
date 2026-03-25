# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_mapping__valid_is_not_in_sbuf.rs`
**Date:** 2026-03-24T14:19:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `PTMem::view` is `external_body` making the entire PTE interpretation opaque and unverified — structural equality of inputs implying equal outputs is trusted. Four false positives: TSO read definitional unfolding, flag combine absorption properties (correct boolean OR/AND semantics matching x86 hierarchy), and empty write_seq identity (follows from fold_left base case).

## True Positives (Spec Issues)

### view_external_body_unconstrained
- **Confidence:** high
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the entire PTE interpretation function is opaque. Even though structurally equal inputs should produce equal outputs (spec functions are deterministic), the fact that view is completely unverified means the mapping from raw page table memory to PTE entries is a critical trust assumption.

## All Candidates

### φ1: read_from_mem_tso_equals_core_mem_read
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `read_from_mem_tso` should unfold to `core_mem(core).read(addr)` which is `pt_mem.write_seq(sbuf[core]).read(addr)` — tests definitional unfolding consistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `read_from_mem_tso` is defined as `self.core_mem(core).read(addr)` and `core_mem` is defined as `self.pt_mem.write_seq(self.sbuf[core])`. This is a direct definitional unfolding — trivially true by construction.

### φ2: flags_combine_absorbs_supervisor
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Once `is_supervisor` is true, combining with any flags keeps it true (OR absorbs true) — this means a supervisor-only directory entry makes the entire subtree supervisor-only, potentially too restrictive
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` defines `is_supervisor: self.is_supervisor || other.is_supervisor`. If `self.is_supervisor` is true, `true || x == true` for any `x`. This is correct boolean OR absorption — it correctly models x86 semantics where supervisor restriction propagates down the page table hierarchy.

### φ3: flags_combine_restricts_writable
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Once `is_writable` is false, combining with any flags keeps it false (AND absorbs false) — a read-only directory makes the entire subtree read-only regardless of leaf permissions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` defines `is_writable: self.is_writable && other.is_writable`. If `self.is_writable` is false, `false && x == false` for any `x`. This is correct boolean AND absorption — it models x86 semantics where a read-only directory makes the entire subtree read-only.

### φ4: view_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — structural equality of inputs should imply equal outputs, but this is trusted without proof; the PTE interpretation could be nondeterministic
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is `external_body` with `unimplemented!()` — the entire PTE interpretation function is opaque. Even though structurally equal inputs should produce equal outputs (spec functions are deterministic), the fact that view is completely unverified means the mapping from raw page table memory to PTE entries is a critical trust assumption.

### φ5: lemma_write_seq_idle_external_body_direct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `write_seq(seq![])` should be identity by fold_left base case — tests that even the trivial case (empty writes) preserves reads without needing the external_body lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write_seq(seq![])` unfolds to `seq![].fold_left(mem, f)` which returns `mem` by the base case of `fold_left`. Therefore `mem.write_seq(seq![]).read(addr) == mem.read(addr)` follows without any external_body lemma. This is trivially correct.

