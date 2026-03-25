# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_writenonneg_new_walk_has_pending_map.rs`
**Date:** 2026-03-24T14:06:57Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `Walk::result` returns `arbitrary()` for path length 1 with a Page entry, leaving behavior unconstrained for this edge case. Four false positives: view domain/value properties follow directly from `Map::new` after reveal, the MAX_BASE bound is an explicit guard in `is_base_pt_walk`, and `P || !P` is a tautology that doesn't expose the `all_mb0_bits_are_zero` external_body gap.

## True Positives (Spec Issues)

### walk_result_path_len1_arbitrary
- **Confidence:** high
- **Reasoning:** When `path.len() == 1` and the last entry is Page, none of the `if path.len() == 2/3/4` branches match, so `Walk::result` falls through to the `else { arbitrary() }` case. The result is unconstrained — it could be `Valid` or anything else. This is a real spec gap since a length-1 Page path has no defined semantics.

## All Candidates

### φ1: view_domain_only_base_walks
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Tests that the view map domain is exactly the set of base-aligned valid walk addresses — if opaque view leaks too much through triggers this may not hold
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is defined as `Map::new(|va| self.is_base_pt_walk(va), ...)`. After `reveal`, `contains_key(va)` directly implies the domain predicate `is_base_pt_walk(va)`. This is correct by construction — the view map's domain is exactly the base walk set.

### φ2: view_value_equals_walk_pte
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Tests that the view map values match pt_walk results — correct by construction but depends on `Map::new` axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem::view` is `Map::new(|va| self.is_base_pt_walk(va), |va| self.pt_walk(va).result()->pte)`. After `reveal`, indexing into the map at a key satisfying the domain predicate returns the value function applied to that key. This is correct by `Map::new` axioms.

### φ3: walk_result_path_len1_arbitrary
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When path.len()==1 and last entry is Page, `Walk::result` falls into `arbitrary()` since none of the len==2/3/4 branches match — the result is unconstrained
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** When `path.len() == 1` and the last entry is Page, none of the `if path.len() == 2/3/4` branches match, so `Walk::result` falls through to the `else { arbitrary() }` case. The result is unconstrained — it could be `Valid` or anything else. This is a real spec gap since a length-1 Page path has no defined semantics.

### φ4: is_base_pt_walk_requires_max_base
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `is_base_pt_walk` requires `vaddr < MAX_BASE` — tests the upper bound cutoff; if MAX_BASE is underspecified, addresses above it are silently excluded from the view
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_base_pt_walk` explicitly requires `vaddr < MAX_BASE` as its first conjunct. When `va >= MAX_BASE`, this conjunct is false, so the whole predicate is false. This is correct and intentional — the view is bounded by the virtual address space limit.

### φ5: all_mb0_bits_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `all_mb0_bits_are_zero` is `external_body` — this tautology verifies trivially but the predicate's actual behavior is completely undefined, gating all PDE validity decisions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `P || !P` is a tautology that holds for any boolean expression regardless of whether it's `external_body` or not. This proves nothing about the external_body gap — it would verify even with a fully specified implementation.

