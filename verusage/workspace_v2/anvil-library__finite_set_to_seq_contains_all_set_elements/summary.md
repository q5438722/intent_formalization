# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/set_lib/finite_set_to_seq_contains_all_set_elements.rs`
**Date:** 2026-03-24T05:00:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from the two independent external_body axioms (`element_in_finite_set_exists_in_set_to_seq` and `element_in_seq_exists_in_original_finite_set`), both unverified trust assumptions that together establish the biconditional. Three false positives: a contrapositive, a definitional base case, and a singleton instantiation with no independent trust surface.

## True Positives (Spec Issues)

### external_body_set_to_seq_forward
- **Confidence:** high
- **Reasoning:** `element_in_finite_set_exists_in_set_to_seq` is external_body — unverified trust assumption. Semantically correct but not proved.

### external_body_seq_to_set_reverse
- **Confidence:** high
- **Reasoning:** `element_in_seq_exists_in_original_finite_set` is external_body — second independent unverified trust assumption. Also semantically correct but not proved.

## All Candidates

### φ1: external_body_set_to_seq_forward
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** element_in_finite_set_exists_in_set_to_seq is external_body — unverified axiom asserting set containment implies to_seq containment
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `element_in_finite_set_exists_in_set_to_seq` is external_body — unverified trust assumption. Semantically correct but not proved.

### φ2: external_body_seq_to_set_reverse
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** element_in_seq_exists_in_original_finite_set is external_body — unverified axiom asserting to_seq containment implies set containment
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `element_in_seq_exists_in_original_finite_set` is external_body — second independent unverified trust assumption. Also semantically correct but not proved.

### φ3: not_in_set_not_in_to_seq
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** contrapositive of the reverse direction — depends on external_body axiom soundness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of φ2. No independent trust surface beyond the already-flagged external_body axiom.

### φ4: empty_set_to_seq_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty set should convert to empty sequence — tests base case consistency with the external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Definitional property of `to_seq` on empty set from vstd. Correct.

### φ5: singleton_to_seq_contains_element
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton set's to_seq should contain the element — tests minimal nontrivial case of the external_body forward axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiation of φ1 on a singleton. No independent trust surface.

