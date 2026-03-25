# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/log_append/append_L_tentatively_append.rs`
**Date:** 2026-03-24T15:02:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_addresses_external_body_roundtrip` exposes an `external_body` trust assumption for the offset↔relative-position roundtrip property. The remaining four are false positives — they confirm correct definitional properties of the wrapping arithmetic, struct update preservation, and range-guarded writes.

## True Positives (Spec Issues)

### lemma_addresses_external_body_roundtrip
- **Confidence:** medium
- **Reasoning:** `lemma_addresses_in_log_area_subregion_correspond_to_relative_log_positions` is `external_body` with an empty body. The roundtrip property (offset→relative pos→offset) is trusted without proof. While the underlying modular arithmetic is correct, this is an unverified trust assumption that could be proved from the definitions directly.

## All Candidates

### φ1: lemma_addresses_external_body_roundtrip
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_addresses_in_log_area_subregion_correspond_to_relative_log_positions` is `external_body` — the roundtrip between offset and relative position is trusted without proof; an incorrect modular arithmetic implementation would break the entire log layout
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_addresses_in_log_area_subregion_correspond_to_relative_log_positions` is `external_body` with an empty body. The roundtrip property (offset→relative pos→offset) is trusted without proof. While the underlying modular arithmetic is correct, this is an unverified trust assumption that could be proved from the definitions directly.

### φ2: relative_pos_wraps_incorrectly
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Tests that relative position 0 maps to head_offset and the wrap-around point maps to offset 0 — if the wrapping logic were off-by-one, log reads would be shifted by one byte
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `relative_log_pos_to_log_area_offset(0, head_offset, log_area_len)` = `head_offset + 0` = `head_offset` (< log_area_len, so no subtraction). For `log_area_len - head_offset`: `head_offset + (log_area_len - head_offset)` = `log_area_len` ≥ `log_area_len`, so it returns `log_area_len - log_area_len` = 0. Correct by definition.

### φ3: inverse_not_bijective_at_boundary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The inverse mapping at the boundary (offset just below head) should give the maximum relative position — if the boundary case is wrong, the last byte of the log area would be misaddressed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `log_area_offset_to_relative_log_pos(head_offset, head_offset, _)`: `head_offset >= head_offset` so returns 0. For `head_offset - 1 < head_offset`: returns `(head_offset - 1) - head_offset + log_area_len` = `log_area_len - 1`. Both correct by the definition's case split.

### φ4: tentative_append_preserves_committed_log
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `tentatively_append` only extends `pending` — if it also modified `log`, `head`, or `capacity`, committed data would be corrupted by tentative operations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `tentatively_append` is defined as `Self { pending: self.pending + bytes, ..self }`, which only modifies `pending` and preserves `log`, `head`, and `capacity` via struct update syntax. Correct by construction.

### φ5: write_outside_range_unchanged
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing to a range should leave bytes outside that range untouched — if the map-based write implementation leaked into other positions, it would corrupt unrelated log data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write` uses `state.map(|pos, pre_byte| if addr <= pos < addr + bytes.len() { ... } else { pre_byte })`. For `k` outside the range, the else branch returns the original byte. Correct by the map definition.

