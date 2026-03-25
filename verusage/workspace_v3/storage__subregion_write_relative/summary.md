# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/storage__subregion_write_relative/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: write_immediately_persists → `write_relative`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** write should only set outstanding_write, not immediately update state_at_last_flush (that requires a flush)

### φ2: empty_write_alters_view → `write_relative`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Writing zero bytes should be a no-op; if it changes the view, the write spec is unsound

### φ3: write_changes_view_len → `write_relative`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Write should preserve region length; a length change would corrupt the memory region abstraction

### φ4: write_clobbers_adjacent_byte → `write_relative`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Write must not modify bytes outside the target range; clobbering an adjacent byte violates memory isolation

### φ5: write_drops_outstanding → `write_relative`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** After writing byte b at addr, outstanding_write must be Some(b); if it is None the write was silently lost

