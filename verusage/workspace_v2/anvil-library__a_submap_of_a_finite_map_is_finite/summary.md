# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/map_lib/a_submap_of_a_finite_map_is_finite.rs`
**Date:** 2026-03-24T04:44:08Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives — standard properties of `submap_of` from vstd's Map axiomatization. No spec issues found.

## All Candidates

### φ1: submap_dom_subset
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests whether submap_of implies domain subset — if this holds trivially via SMT it's expected, but confirms the semantic foundation the lemma relies on
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `submap_of` is defined as domain subset plus value agreement. Domain subset is a direct consequence — correct and expected.

### φ2: finite_submap_cardinality_bound
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** if the spec is too weak, cardinality ordering might not follow from finiteness alone — tests whether submap + finite propagates to cardinality bound

### φ3: empty_map_submap_of_any
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tests whether empty map is universally a submap — if submap_of is incorrectly axiomatized this could fail or be vacuously exploitable
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty map has empty domain, so `submap_of` holds vacuously for any map. Correct and expected property.

### φ4: submap_value_agreement
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** submap_of should enforce value agreement on shared keys — if it only constrains domains without value equality, the spec would be too weak

### φ5: submap_antisymmetry_implies_equality
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** mutual submap should imply extensional equality — if submap_of is too weak (e.g., only domain containment without value agreement), this would fail

