# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_candidate_mapping_inflight_vmem_overlap_os_implies_hl.rs`
**Date:** 2026-03-24T14:43:55Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The rl2 interp pt_mem unfolding, empty map non-overlap, x86 arch invariant, between exclusive upper bound, and adjacent region non-overlap are all correct properties following directly from the open spec definitions, Map axioms, concrete constant construction, and arithmetic.

## All Candidates

### φ1: rl2_interp_pt_mem_is_writer_mem
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The rl2→rl1 interp sets `pt_mem: self.writer_mem()` which is `self.core_mem(self.writes.core)` = `self.pt_mem.write_seq(self.sbuf[self.writes.core])` — the rl1 view's page table is the writer's TSO-applied memory, meaning non-writer cores' pending writes are invisible at rl1 level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `rl2::State::interp()` sets `pt_mem: self.writer_mem()` where `writer_mem` is `#[verifier(inline)]` expanding to `self.core_mem(self.writes.core)` = `self.pt_mem.write_seq(self.sbuf[self.writes.core])`. The ensures is a direct unfolding of the open spec definitions.

### φ2: candidate_mapping_overlaps_empty_map
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** With an empty mapping table, no existing mapping can overlap — tests that the existential in `candidate_mapping_overlaps_existing_vmem` correctly returns false when the map has no keys
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Map::empty()` has no keys, so `mappings.contains_key(b)` is false for all `b`. The existential cannot be satisfied. Correct by Map axioms.

### φ3: arch_inv_holds_for_x86
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `x86_arch_spec` is a concrete constant — if `inv()` doesn't hold, all recommends-guarded functions using `x86_arch_spec` operate outside their intended preconditions, silently producing arbitrary results
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x86_arch_spec` is a concrete 4-layer constant with entry sizes `[L0, L1, L2, L3]` and `num_entries: 512` at each layer. The inv checks `layers.len() <= 4`, positive sizes/entries bounded by constants, and `entry_size_is_next_layer_size` — all hold by construction since `L0 = 512*L1`, `L1 = 512*L2`, `L2 = 512*L3`.

### φ4: between_exclusive_upper
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `between(x, a, b)` uses strict `<` for the upper bound — the endpoint `b` itself should not be "between" a and b; if it were, fence-post errors would corrupt range checks throughout the spec
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `between(b, a, b)` = `a <= b && b < b`. Since `b < b` is false, the result is false. Correct by definition — the upper bound is exclusive.

### φ5: overlap_adjacent_no_overlap
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two adjacent regions (one ends exactly where the next begins) should not overlap — tests that `overlap` uses strict `<` correctly; if adjacent regions overlapped, contiguous memory layouts would be impossible
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `r1 = {base, size}` and `r2 = {base+size, size}`, since `base <= base+size`, we check `base == base+size` (false since `size > 0`) or `base+size < base+size` (false). Non-overlap is correct — adjacent regions don't overlap.

