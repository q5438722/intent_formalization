# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__next_step_preserves_inv_unmapping__inflight_walks.rs`
**Date:** 2026-03-24T14:30:48Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Nonneg/nonpos mutual exclusivity follows from bitvector arithmetic (`x & 1` can't be both 0 and 1). Write-read roundtrip is a standard Map insert/index axiom. Walk flags out-of-bounds accesses are harmless since the length-1 branch returns `flags0` without using the intermediate computations.

## All Candidates

### φ1: nonneg_nonpos_mutually_exclusive
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_nonneg_write` requires `read(addr) & 1 == 0` and `value & 1 == 1`; `is_nonpos_write` requires `read(addr) & 1 == 1` and `value & 1 == 0` — tests that these are truly exclusive; if both could hold, the P-bit transition model would be inconsistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_nonneg_write` requires `read(addr) & 1 == 0` while `is_nonpos_write` requires `read(addr) & 1 == 1`. These are contradictory — `x & 1` is either 0 or 1, never both. Similarly `value & 1 == 1` vs `value & 1 == 0` are contradictory. This mutual exclusivity is correct by bitvector arithmetic.

### φ2: write_read_roundtrip
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing `value` at `addr` and reading back should return `value` — tests that `mem.insert(addr, value)` followed by `mem[addr]` gives the inserted value; failure would mean the Map axioms or PTMem wiring is broken
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write` sets `mem: self.mem.insert(addr, value)` and `read` returns `self.mem[addr]`. By vstd Map axioms, `m.insert(k, v)[k] == v`. This is a correct fundamental property of the Map data structure.

### φ3: walk_flags_accesses_beyond_path_len
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `Walk::flags` unconditionally computes `flags1 = flags0.combine(from_GPDE(path[1].1))` etc. even when `path.len() == 1` — the out-of-bounds accesses `path[1]`, `path[2]`, `path[3]` return arbitrary values; tests whether the final branch still selects `flags0`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Walk::flags` computes `flags1`, `flags2`, `flags3` using out-of-bounds indices, but the final `if path.len() == 1 { flags0 }` branch discards them. Verus evaluates spec functions lazily/symbolically — the unused intermediate values don't affect the result. The function correctly returns `flags0` for single-entry paths.

### φ4: walk_result_path_len_1_is_arbitrary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For `path.len() == 1` with a Page last entry, `Walk::result` falls to the `else { arbitrary() }` branch — two different walks with different vaddrs would get the same arbitrary value, meaning the result is unconstrained and identical for all such walks

### φ5: pde_zero_entry_always_invalid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An all-zero PDE entry has P bit = 0, so PDE::view should return Invalid regardless of layer — tests the base case of the PDE classification logic

