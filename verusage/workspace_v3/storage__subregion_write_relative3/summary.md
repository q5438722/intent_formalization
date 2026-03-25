# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/storage__subregion_write_relative3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: write_is_noop → `write_relative`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, writes of non-empty bytes silently have no effect, meaning the spec is vacuous or the write operation is a no-op.

### φ2: view_length_changes → `write_relative`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, a write changes the subregion length, violating the expectation that writes preserve the memory region's size.

### φ3: outside_range_modified → `write_relative`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, a write corrupts bytes outside the target range, violating spatial isolation of the write operation.

### φ4: write_not_recorded_as_outstanding → `write_relative`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, written data is not recorded as an outstanding write, meaning the persistent memory model loses written data before flush.

### φ5: flush_state_corrupted → `write_relative`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, a write mutates the last-flushed state of bytes in-range, violating the crash-consistency model where only flush updates persistent state.

