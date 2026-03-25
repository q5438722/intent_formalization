# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__assert_maps_equal_contains_pair.rs`
**Date:** 2026-03-24T12:25:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains a single fully verified lemma proving map extensionality via contains_pair, with no external_body or trust gaps. All tested properties are correct mathematical facts or trivial reflexivity.

## All Candidates

### φ1: maps_equal_from_contains_pair
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two maps with identical contains_pair sets are extensionally equal — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the fully verified `assert_maps_equal_contains_pair` lemma. Map extensionality via contains_pair is a correct mathematical property.

### φ2: empty_maps_equal
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two empty maps are trivially equal — tests the degenerate case where both precondition quantifiers are vacuously true
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x

### φ3: singleton_maps_equal
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two identical singleton maps are equal — tests the minimal non-trivial case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same value equals itself — `map![k => v]

### φ4: contains_pair_symmetric_implies_equal
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Extensional equality (`=~=`) implies deep equality (`===`) for maps — tests that the `=~=` to `===` step works directly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `=~=` implying `

### φ5: insert_preserves_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Inserting a key-value pair makes it a contained pair — tests the basic Map axiom that the lemma relies on
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `m.insert(k, v).contains_pair(k, v)` is a basic vstd Map axiom. Correct by definition.

