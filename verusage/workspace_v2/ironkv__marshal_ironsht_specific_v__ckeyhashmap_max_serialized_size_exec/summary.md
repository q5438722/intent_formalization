# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__ckeyhashmap_max_serialized_size_exec.rs`
**Date:** 2026-03-24T09:06:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — they are trivial arithmetic consequences of the constant 0x100000 being a small, well-chosen value. The opaque constant intentionally limits serialized hash map size for the SHT protocol, and the resulting properties (fits in u32, power of two, far below usize::MAX, summable without overflow) are all desirable safety margins rather than spec inconsistencies.

## All Candidates

### φ1: max_serialized_size_fits_in_u32
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The max serialized size (0x100000 = 1 MiB) fits in a u32, meaning the spec silently constrains hash maps to at most 1 MiB serialized — this may be too small for large hash maps and the limit is buried in an opaque constant with no documentation of why this bound was chosen
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a trivial consequence of the constant being 0x100000 (1 MiB). The bound is an intentional design choice for the SHT protocol's network message size limit, not a spec gap.

### φ2: max_serialized_size_is_power_of_two
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The constant is exactly 2^20 — a suspiciously round number that may be an arbitrary placeholder rather than a carefully derived bound, and downstream code relying on this specific value could break if the real protocol needs a different limit
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This merely confirms the literal value of the constant. Whether 2^20 is a placeholder or carefully chosen is a design decision, not a spec inconsistency — the spec correctly defines and exposes this value.

### φ3: max_size_smaller_than_usize_max
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The max serialized size is far below `usize::MAX / 2` — any code that adds the max size to another offset will never overflow, but this means the spec artificially limits hash map sizes to a tiny fraction of addressable memory without justification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Being far below `usize::MAX / 2` is a desirable safety margin that prevents overflow in size arithmetic. This is a standard engineering practice for serialization bounds, not a spec weakness.

### φ4: exec_reveals_opaque_constant
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The opaque spec can be revealed by any caller to learn the exact constant — the `opaque` annotation provides no real encapsulation since `ckeyhashmap_max_serialized_size_exec` already leaks the value through its ensures clause and any proof can call `reveal`

### φ5: two_max_sizes_fit_in_usize
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two maximally-sized serialized hash maps can be summed without overflow — this could mask buffer overflow bugs in code that concatenates two serialized hash maps, since the spec allows it but the bound may be too generous or too restrictive depending on the actual protocol requirements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a straightforward arithmetic consequence of the constant being 1 MiB. It's a useful property ensuring safe addition of sizes, not an indication of a spec gap.

