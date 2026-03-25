# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagemap__impl0__init/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: init_has_present_entry → `init`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After init all entries must be empty (present==false); if a present entry exists, init failed to clear the page map

### φ2: init_nonzero_addr → `init`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Entry 0 should have addr==0 after init since is_empty requires addr==0; a non-zero addr would mean init left stale physical addresses

### φ3: init_vacuous_precondition → `init`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If init's preconditions are unsatisfiable, every postcondition holds vacuously and init can never be safely called

### φ4: set_modifies_other_index → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** set should only modify index i; if another index j changes, set has unintended side-effects corrupting unrelated page entries

### φ5: set_changes_length → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** set's wf() postcondition guarantees preserved length; if length changes, the array invariant is broken and indexing becomes unsafe

