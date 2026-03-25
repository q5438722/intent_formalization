# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_unique_and_overlap_values_implies_overlap_vmem.rs`
**Date:** 2026-03-24T14:51:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The rl2→rl1 refinement intentionally abstracts non-writer store buffers as part of the TSO model, and valid_core delegation to common constants is correct since core validity is a hardware-level property.

## All Candidates

### φ1: map_done_failure_still_has_pte_size
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A failed MapDone still reports the original pte's frame size — the inflight region persists even after the map operation failed, blocking other operations from using that virtual address range until the core returns to Idle

### φ2: is_map_includes_waiting_but_not_crit
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapWaiting is classified as `is_map` but not `is_in_crit_sect` — if code checks `is_map` expecting critical section protection, it would incorrectly assume mutual exclusion for waiting operations

### φ3: rl2_interp_loses_sbuf_for_non_writer
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The rl2→rl1 refinement flattens pt_mem to the writer's view only — all non-writer cores' store buffers are invisible at rl1, meaning rl1 cannot distinguish states that differ only in non-writer sbufs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The rl2→rl1 refinement explicitly sets `pt_mem: self.writer_mem()` which unfolds to `self.pt_mem.write_seq(self.sbuf[self.writes.core])`. This is the intentional design of the refinement — rl1 abstracts away non-writer store buffers by collapsing to the writer's view. This is correct TSO abstraction.

### φ4: candidate_mapping_in_bounds_ignores_base_alignment
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `candidate_mapping_in_bounds` only checks `base + size < upper_vaddr` without requiring alignment — a mapping at base=1 (unaligned) passes the bounds check, potentially allowing misaligned virtual mappings

### φ5: valid_core_delegates_to_common
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `os::Constants::valid_core` delegates entirely to `common.valid_core` — os-level fields like `ult2core` and `ult_no` don't affect core validity, meaning two os::Constants with different thread configurations but same common constants agree on valid cores
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `os::Constants::valid_core` is defined as `self.common.valid_core(core)`. Two constants sharing the same `common` field will trivially agree on core validity. This is correct by construction — core validity is a hardware/topology property independent of OS-level thread configuration.

