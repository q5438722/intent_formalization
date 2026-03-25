# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__syscall_io_mmap__impl0__syscall_io_mmap/original.rs`
**Date:** 2026-03-25 04:04:53
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidate properties target the same boundary condition: zero-length `VaRange4K` inputs to `syscall_io_mmap`. While Verus confirms the spec entails these properties, they represent benign vacuous-truth scenarios rather than genuine spec weaknesses. A zero-length IO mmap is a no-op that consumes no resources and preserves all invariants. The quota check similarly behaves correctly — zero quota with zero demand is not a bypass. No true positives were found in this batch.

## All Candidates

### φ1: zero_len_io_mmap_allowed
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero-length VA ranges satisfy all preconditions, allowing syscall_io_mmap to perform vacuous no-op allocations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A zero-length VaRange4K passing all preconditions of `syscall_io_mmap` is not a spec gap — it results in a vacuous no-op that allocates zero pages and maps nothing. The postcondition of `range_alloc_and_map_io` still ensures `self.wf()`, so no invariant is violated. Allowing zero-length operations is standard and harmless; the spec need not reject them.

### φ2: null_paddr_mem_valid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If null physical address 0 passes MEM_valid, IO mmap could create page mappings to the null address without detection

### φ3: zero_raw_entry_is_empty
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If zero-valued raw entries are always empty, uninitialized page table memory silently passes as valid empty entries during IO mapping

### φ4: zero_quota_bypasses_check
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A container with zero remaining quota can pass the quota check via a zero-length range, bypassing resource accounting in syscall_io_mmap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The quota check `quota.mem_4k < va_range.len * 4` correctly passes when both quota and len are 0, since 0 < 0 is false. This isn't a bypass — zero quota with zero-length request means no resources are needed and none are consumed. When len > 0, zero quota correctly triggers rejection. The arithmetic is sound for all meaningful cases.

### φ5: usize2pa_always_valid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If usize2pa produces a MEM_valid address for ANY input, garbage or attacker-controlled values fed through the IO mmap path are silently masked to valid addresses

