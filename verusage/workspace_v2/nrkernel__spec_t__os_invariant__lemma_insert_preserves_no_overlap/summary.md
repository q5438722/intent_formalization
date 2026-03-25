# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_insert_preserves_no_overlap.rs`
**Date:** 2026-03-24T14:46:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. Critical section implying non-Idle is correct by the match arm definitions, and multiple cores having identical Idle states is intentionally allowed by the uniqueness predicate's idle guard.

## All Candidates

### φ1: lemma_insert_preserves_shrinking_region_overlap
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The precondition requires only one core is in critical section (`forall|cr| ... is_in_crit_sect() ==> cr == core`), but also requires `other_core` is non-Idle and different from `core` — if non-Idle non-crit-sect states (MapWaiting, UnmapWaiting) can exist, this is satisfiable; if not, it's vacuous

### φ2: crit_sect_implies_not_idle
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `is_in_crit_sect` excludes Idle, MapWaiting, UnmapWaiting — tests that the critical section predicate correctly implies non-Idle; if a critical-section state were also Idle, the lemma's preconditions would be contradictory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `is_in_crit_sect` returns false for `Idle`, `MapWaiting`, and `UnmapWaiting`, and true for everything else. Since Idle maps to false, `is_in_crit_sect()` being true necessarily means `!is_idle()`. Correct by exhaustive case analysis of the match arms.

### φ3: map_waiting_not_in_crit_but_not_idle
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** MapWaiting is neither in critical section nor Idle — this creates a gap where cores with pending map requests exist but aren't protected by the single-crit-sect precondition, potentially allowing the lemma to be applied while another core has a waiting operation

### φ4: pte_size_shrink_allows_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the vaddr isn't in pt, UnmapExecuting with no result has pte_size 0 — the `pte_size >= new_cs.pte_size` precondition is trivially satisfied for any new_cs with size 0, allowing the lemma to be applied even when the original operation had no real region to protect

### φ5: unique_corestates_allows_equal_idle
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `unique_CoreStates` allows multiple cores to have identical Idle states — the `!map[core].is_idle()` guard means Idle is exempt from uniqueness, so duplicate Idle values don't violate the invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `unique_CoreStates` guards with `!map[core].is_idle()`, so Idle entries are exempt from the uniqueness check. Multiple cores sharing the Idle state is correct and expected — uniqueness is only needed for active operations.

