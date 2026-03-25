# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/vest__utils__set_range/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: noop_nonempty_input → `set_range`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a non-empty write is a no-op, set_range never actually modifies data

### φ2: splice_changes_length → `set_range`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If splice changes the sequence length, the ensures clause (length preserved) would be contradictory

### φ3: written_byte_mismatch → `set_range`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the byte at position i doesn't match input[0], the splice wrote incorrect data

### φ4: prefix_corrupted → `set_range`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If bytes before position i are modified, set_range corrupts data outside the target range

### φ5: distinct_inputs_same_result → `set_range`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two inputs differing in the first byte produce identical splices, the spec ignores input content

