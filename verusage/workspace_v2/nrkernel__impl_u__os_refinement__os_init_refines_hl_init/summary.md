# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__os_init_refines_hl_init.rs`
**Date:** 2026-03-24T13:18:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

One candidate evaluated: the removal of inflight vaddrs from effective_mappings is a false positive. This is a deliberate concurrency safety mechanism — mappings under active map/unmap operations are intentionally excluded from the effective view to prevent observing transitional states.

## All Candidates

### φ1: ptmem_view_external_body_write_invisible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — writes to page table memory have no observable effect on the PTE interpretation

### φ2: rl3_interp_external_body_collapses
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec — completely opaque, so any two rl3 states could map to the same rl2 state

### φ3: rl2_init_external_body_vacuous
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl2::init` is `external_body` closed spec with no postconditions — the initial state predicate could be trivially satisfiable for any state

### φ4: max_phyaddr_width_weaker_than_constant
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` constant set to 52 but the axiom only constrains it to `[32, 52]` — the axiom is weaker than the actual value, allowing the SMT solver to assume any width in that range

### φ5: effective_mappings_remove_inflight_loses_valid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `effective_mappings` removes all inflight vaddrs from interp_pt_mem — even if the mapping is still valid and the inflight operation hasn't modified it yet, it becomes invisible to the effective mapping view
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional by design. `effective_mappings` deliberately removes inflight vaddrs because those addresses are currently being modified by map/unmap operations — the mapping's validity is in flux. Hiding them from the effective view is a conservative, correct approach that prevents reading stale or transitional mappings during concurrent operations. The `applied_mappings` function (which includes `extra_mappings`) provides the actual view used for virtual memory interpretation.

