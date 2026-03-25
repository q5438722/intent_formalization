# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_loses_existing_key → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, set could silently drop existing keys unrelated to the inserted key, violating map update semantics.

### φ2: set_value_wrong_for_key → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, the value stored at the inserted key would differ from the value passed in, making set fundamentally broken.

### φ3: set_modifies_other_value → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, inserting one key-value pair would corrupt the value of an unrelated existing key, breaking frame preservation.

### φ4: set_adds_spurious_key → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, set would introduce phantom keys into the domain that were never inserted, violating the domain-change guarantee.

### φ5: set_gap_survives_insertion → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, a gap that previously contained k would remain empty even after inserting k, meaning the gap postcondition fails to detect new keys.

