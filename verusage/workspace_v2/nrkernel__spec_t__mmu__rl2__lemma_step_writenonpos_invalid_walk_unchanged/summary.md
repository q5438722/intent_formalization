# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_writenonpos_invalid_walk_unchanged.rs`
**Date:** 2026-03-24T14:08:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Flag combine idempotency holds because AND and OR are both idempotent on booleans. The out-of-bounds path access is dead code since only `flags0` is returned for length-1 paths. Equal substitution is trivially true for any spec function. Write-read consistency follows from standard vstd Map axioms.

## All Candidates

### φ1: flags_combine_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Flag combination should NOT be idempotent — `is_supervisor` uses OR, so `false || false == false` but the combine is not a projection; if it is idempotent, the lattice structure may be too restrictive
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `combine` computes `is_writable: a && a == a`, `is_supervisor: a || a == a`, `disable_execute: a || a == a`. All three are idempotent operations on booleans, so `f.combine(f) == f` is mathematically correct. This is a desirable lattice property, not a spec gap.

### φ2: flags_out_of_bounds_path_access
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Walk::flags` computes `flags1 = flags0.combine(Flags::from_GPDE(path[1].1))` unconditionally — for path.len()==1, `path[1]` is out of bounds, making `flags1..flags3` depend on undefined values
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** While `path[1]` is indeed out of bounds when `path.len() == 1`, the `flags()` function returns `flags0` in the `path.len() == 1` branch, so the out-of-bounds computations of `flags1..flags3` are dead code. Verus spec functions are lazily evaluated — the intermediate values are computed but never used in the return path, and SMT can still prove the ensures since only `flags0` matters.

### φ3: walk_result_not_invalid_implies_page_last
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If result is Valid but last entry is not Page, `Walk::result` returns `arbitrary()` which could be anything — tests whether Valid results always come from Page entries

### φ4: valid_core_external_body_vacuous
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_core` is `external_body` — even substitution of equal cores is trusted; the predicate is fully opaque with no constraints
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Equal substitution (`c1 == c2` implies `f(c1) == f(c2)`) holds for any spec function, including `external_body` ones. Spec functions are deterministic by construction in Verus. This tautology proves nothing about the external_body gap.

### φ5: write_read_same_addr
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After writing a value to an address, reading it back should return that value — tests basic Map::insert/index consistency which is foundational but depends on vstd axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write` uses `Map::insert` and `read` uses `Map::index`. `mem.insert(addr, value)[addr] == value` follows from vstd's `Map::insert` axiom. This is correct and expected — basic read-after-write consistency.

