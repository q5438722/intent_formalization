# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/storage__subregion_write_absolute/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: write_changes_subregion_length → `write_absolute`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Write should never change the length of the subregion view; if it did, invariant `self.view(wrpm).len() == self.len()` would break on subsequent calls.

### φ2: empty_write_changes_view → `write_absolute`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A zero-length write must be a no-op; if the spec allowed it to modify the view, callers could observe spurious state changes from vacuous writes.

### φ3: write_modifies_outside_range → `write_absolute`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Bytes outside the written range must be unaffected; if write could modify them, the `views_differ_only_where_subregion_allows` invariant would be violated.

### φ4: write_alters_state_at_last_flush → `write_absolute`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Write must only set outstanding_write, never alter state_at_last_flush; changing it would corrupt the crash-consistency model since flush recovery depends on state_at_last_flush being stable.

### φ5: nonempty_write_is_noop → `write_absolute`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-empty write to a region with no outstanding writes must visibly change the view (outstanding_write goes from None to Some); if it were a no-op the write would be silently lost.

