# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagemap__impl0__set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_corrupts_other_index → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, set violates frame preservation — modifying one index silently corrupts another entry

### φ2: empty_set_becomes_present → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, setting an empty entry paradoxically produces a present entry, contradicting is_empty semantics

### φ3: set_present_loses_address → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, set silently zeroes the physical address of a present mapping, destroying the page table entry

### φ4: set_drops_write_permission → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, set silently strips the write permission, creating a read-only mapping when read-write was intended

### φ5: set_changes_seq_length → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, set changes the sequence length, breaking the 512-entry invariant and corrupting the page table structure

