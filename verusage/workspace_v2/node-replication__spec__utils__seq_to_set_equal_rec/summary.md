# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_utils/spec__utils__seq_to_set_equal_rec.rs`
**Date:** 2026-03-24T12:10:50Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: seq_to_set_rec_contains_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `seq_to_set_rec_contains` is `external_body` — the biconditional between `seq.contains(a)` and `seq_to_set_rec(seq).contains(a)` is trusted without proof

### φ2: seq_to_set_equal_rec_via_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The equality `seq_to_set == seq_to_set_rec` depends on the unverified `seq_to_set_rec_contains` axiom — if that axiom is wrong, the two definitions could disagree

### φ3: seq_to_set_rec_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty sequence produces empty set — tests the base case of the recursive definition

### φ4: seq_to_set_rec_singleton
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A singleton sequence produces a singleton set — tests the one-element recursive step

### φ5: seq_to_set_rec_superset_of_elements
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Every element at a valid index is in the resulting set — depends on the unverified `seq_to_set_rec_contains` external_body axiom

