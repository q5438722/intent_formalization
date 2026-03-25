# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_unmapping__notin_nonpos.rs`
**Date:** 2026-03-24T14:31:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Write non-interference and empty write_seq identity follow from vstd Map/Seq axioms. PDE view reflexivity is trivially true. The supervisor flag inversion correctly implements x86 US-bit semantics.

## All Candidates

### φ1: write_preserves_other_reads
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing at `addr1` should not affect reads at a different `addr2` — tests non-interference of the Map insert operation; failure would indicate aliasing in the PTMem memory model
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write` sets `mem: self.mem.insert(addr1, value)` and `read(addr2)` returns `self.mem[addr2]`. By vstd Map axioms, `m.insert(k1, v)[k2] == m[k2]` when `k1 != k2`. This is standard non-interference from Map semantics.

### φ2: write_seq_empty_is_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Applying an empty write sequence via `fold_left` should return the original PTMem — tests that `fold_left` with empty Seq returns the accumulator; failure would break the base case of write_seq induction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write_seq` unfolds to `Seq::empty().fold_left(self, ...)`. By vstd axioms, `fold_left` on an empty sequence returns the accumulator. This is correct by definition.

### φ3: pde_same_entry_layer_same_view
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** PDE::view should be deterministic — same entry and layer should always produce the same GPDE; but `all_mb0_bits_are_zero` is `external_body`, so if it were nondeterministic, two calls could disagree (this is trivially true by referential transparency but tests the assumption)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x == x` is trivially true for any expression by reflexivity. This doesn't test nondeterminism — it's the same expression on both sides, so the SMT solver equates them immediately without even unfolding.

### φ4: from_bits_supervisor_inverted
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `from_bits` sets `is_supervisor: !flag_US` — so US=true means not-supervisor and US=false means supervisor; this inversion matches x86 semantics where the US bit grants user access, but if inverted incorrectly, kernel pages would be accessible from userspace
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `from_bits(true, true, false)` sets `is_supervisor: !true == false` and `from_bits(true, false, false)` sets `is_supervisor: !false == true`. This correctly models x86 semantics where the US (User/Supervisor) bit being set grants user-mode access, i.e., !US means supervisor-only.

### φ5: pt_walk_only_depends_on_reachable_memory
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If two memories agree on pml4 and the L0 entry (which is non-present), the walk should terminate at L0 with the same path — tests that pt_walk short-circuits on Invalid entries and doesn't read further memory

