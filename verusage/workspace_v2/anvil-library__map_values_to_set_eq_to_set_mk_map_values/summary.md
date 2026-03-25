# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/map_values_to_set_eq_to_set_mk_map_values.rs`
**Date:** 2026-03-24T04:48:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive from the external_body axiom `push_to_set_eq_to_set_insert` (unverified trust assumption bridging sequence push and set insert). Two false positives: empty sequence base case and duplicate push idempotence are correct consequences with no independent trust surface.

## True Positives (Spec Issues)

### push_to_set_eq_to_set_insert_bridge
- **Confidence:** high
- **Reasoning:** `push_to_set_eq_to_set_insert` is external_body asserting `s.push(e).to_set() == s.to_set().insert(e)` without proof. Unverified trust assumption.

## All Candidates

### φ1: push_to_set_eq_to_set_insert_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** push_to_set_eq_to_set_insert is external_body asserting intensional equality between push-then-to_set and to_set-then-insert without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `push_to_set_eq_to_set_insert` is external_body asserting `s.push(e).to_set() == s.to_set().insert(e)` without proof. Unverified trust assumption.

### φ2: empty_seq_map_values_to_set_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty sequence mapped should produce empty set — tests whether mk_map on empty set produces empty map with empty values
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence mapped produces empty sequence, whose `to_set()` is empty. `mk_map` on empty set produces empty map with empty values. Correct and expected.

### φ3: singleton_map_values_to_set
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** singleton sequence mapped should produce singleton set — tests base case interaction between to_set, mk_map, and values

### φ4: map_values_to_set_len_leq_seq_len
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** mapping can collapse values so the image set should never exceed the domain set size — tests that mk_map values count is bounded by domain

### φ5: duplicate_push_to_set_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pushing a duplicate element should not change the set — relies on external_body axiom plus set insert idempotence; if the axiom interacts incorrectly with to_set this could mask an inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Set insert is idempotent when the element already exists. Combined with the axiom `push(e).to_set() == to_set().insert(e)` and `s.contains(e)` implying `to_set().contains(e)`, this is correct. No new trust surface beyond φ1.

