# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_map_insert_values_equality.rs`
**Date:** 2026-03-24T14:49:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

One true positive: `lemma_map_insert_value` is an `external_body` proof asserting that inserted values appear in the map's values set. While this is a correct property, it's trusted without verification — a gap that could be closed by proving it from vstd Map axioms directly.

## True Positives (Spec Issues)

### lemma_map_insert_value_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_map_insert_value` is `external_body` with `unimplemented!()` body — its ensures (`map.insert(key, value).values().contains(value)`) is a correct Map property but is trusted without proof. While this is a standard consequence of Map axioms, the fact that it's asserted via external_body rather than derived from vstd axioms makes it an unverified trust assumption.

## All Candidates

### φ1: lemma_map_insert_value_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_map_insert_value` is `external_body` with `unimplemented!()` — its ensures is a standard Map property but is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_map_insert_value` is `external_body` with `unimplemented!()` body — its ensures (`map.insert(key, value).values().contains(value)`) is a correct Map property but is trusted without proof. While this is a standard consequence of Map axioms, the fact that it's asserted via external_body rather than derived from vstd axioms makes it an unverified trust assumption.

### φ2: values_equality_swaps_old_new
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The lemma establishes a set equality involving `.insert(map[key])` on both sides — if this stronger form (removing old, adding new) also holds, the values set correctly tracks replacements; if it doesn't, the lemma may be weaker than expected

### φ3: insert_existing_key_same_value_noop
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Re-inserting the same key-value pair should leave values unchanged — tests that the lemma's set equality degenerates correctly when `value == map[key]`

### φ4: values_equality_key_not_in_dom
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the key is fresh (not in domain), insert should simply add the value to values — the verified lemma only handles existing keys; if this also verifies via `lemma_map_insert_value`, the external_body is doing more work than expected

### φ5: values_equality_singleton_map
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Overwriting the only entry in a singleton map should produce values = {new_val} — tests that the equality lemma correctly accounts for singleton maps where the old value is fully replaced

