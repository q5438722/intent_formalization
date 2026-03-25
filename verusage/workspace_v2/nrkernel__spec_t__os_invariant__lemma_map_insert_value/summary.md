# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_map_insert_value.rs`
**Date:** 2026-03-24T14:47:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. They test correct, expected properties of vstd Map axioms: inserted values appear in values(), overwriting removes old values from singletons, empty maps have no values, and double-insert at the same key follows last-write-wins. None expose spec gaps.

## All Candidates

### φ1: insert_value_in_values_trivial
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The lemma has no preconditions — it universally proves that any inserted value appears in the map's values; this is a correct Map axiom consequence, not a spec gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct invocation of the lemma under test, which itself is a correct consequence of vstd Map axioms. The inserted value is at the inserted key, so it's in values(). No spec gap.

### φ2: insert_preserves_other_values
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Inserting at a different key should preserve existing values in the values set — tests that Map insert doesn't drop other entries; if existing values disappeared, the map would be lossy

### φ3: insert_overwrite_removes_old_value
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Overwriting the only key's value should remove the old value from values — tests that values() correctly reflects the updated map; if old values lingered, the values set would be stale
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A singleton map `{key: old_val}` overwritten with `{key: new_val}` has only one key mapping to `new_val`. Since `old_val != new_val` and no other key holds `old_val`, the old value is correctly absent from values(). Standard Map semantics.

### φ4: empty_map_no_values
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty map should have no values — tests the base case; if empty maps had phantom values, the lemma's conclusion would be trivially true for wrong reasons
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty map has an empty domain, so no key maps to any value, and `values()` is empty. This is a correct foundational Map property.

### φ5: insert_same_key_twice_last_wins
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Double insert at the same key should result in last-write-wins — tests that the second insert fully overwrites the first; if the first persisted, the map would have nondeterministic reads
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `map.insert(k, v1).insert(k, v2)[k]` returns `v2` by the Map insert axiom — the second insert overwrites the first at the same key. Standard last-write-wins semantics.

