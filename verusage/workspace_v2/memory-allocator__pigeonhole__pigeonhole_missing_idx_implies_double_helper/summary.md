# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__pigeonhole_missing_idx_implies_double_helper.rs`
**Date:** 2026-03-24T11:41:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives identifying external_body trust gaps. `set_mismatch` derives `false` (contradiction) without proof — especially risky since it's the core reasoning step in the pigeonhole base case. `nat_set_size` trusts finiteness and cardinality without proof, providing the domain size fact that enables the `set_mismatch` invocation. Together these two unverified axioms form the entire trusted foundation of the pigeonhole argument.

## True Positives (Spec Issues)

### set_mismatch_external_body_false
- **Confidence:** medium
- **Reasoning:** `set_mismatch` is `external_body` deriving `false` — a contradiction lemma whose correctness is critical for soundness. The mathematical claim is valid (a strict subset cannot have equal cardinality for finite sets), but it is entirely unverified. Any bug in the preconditions or the reasoning would introduce unsoundness.

### nat_set_size_external_body
- **Confidence:** medium
- **Reasoning:** `nat_set_size` is `external_body` trusting both finiteness and exact cardinality of a set characterized by a membership predicate. This is a foundational axiom used in the pigeonhole proof's base case — unverified trust gap.

## All Candidates

### φ1: set_mismatch_external_body_false
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `set_mismatch` is `external_body` deriving `false` — a contradiction lemma trusted without proof; any error in its preconditions would make the entire system unsound
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `set_mismatch` is `external_body` deriving `false` — a contradiction lemma whose correctness is critical for soundness. The mathematical claim is valid (a strict subset cannot have equal cardinality for finite sets), but it is entirely unverified. Any bug in the preconditions or the reasoning would introduce unsoundness.

### φ2: nat_set_size_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `nat_set_size` is `external_body` trusting finiteness and cardinality without proof — this is the foundation for the pigeonhole proof's termination argument
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `nat_set_size` is `external_body` trusting both finiteness and exact cardinality of a set characterized by a membership predicate. This is a foundational axiom used in the pigeonhole proof's base case — unverified trust gap.

### φ3: helper_dup1_in_range
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The duplicate witness `dup1` is guaranteed to be `< len` by the ensures — but this relies on the `choose` selecting from the existential guaranteed by the external_body-backed proof chain

### φ4: set_mismatch_singleton
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Testing whether `set_mismatch` can be invoked with mismatched sizes to derive `false` — the `s1.len() == s2.len()` guard should prevent this

### φ5: helper_duplicate_value_not_missing
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The duplicated value avoids the missing index — follows from the precondition but confirms the pigeonhole maps `len` keys into `len - 1` non-missing values

