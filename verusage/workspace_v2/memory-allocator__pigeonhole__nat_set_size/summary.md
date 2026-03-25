# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__nat_set_size.rs`
**Date:** 2026-03-24T11:35:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives identify the `external_body` trust gap in `lemma_nat_range`: the general finiteness/cardinality axiom and its singleton instantiation both rely on unverified assumptions. Three false positives test correct consequences: empty set properties, set extensional equality from identical membership, and biconditional-implied exclusion — all are trivial logical/set-theoretic facts independent of the external_body concern.

## True Positives (Spec Issues)

### lemma_nat_range_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_nat_range` is `external_body` trusting both finiteness and cardinality of nat range sets without proof. This is a foundational axiom that `nat_set_size` depends on — any error would propagate to all downstream set cardinality reasoning.

### lemma_nat_range_singleton
- **Confidence:** medium
- **Reasoning:** The singleton range having length 1 depends entirely on the `external_body` `lemma_nat_range`. This is mathematically correct but unverified — the trust gap in the axiom applies to all instantiations including this basic case.

## All Candidates

### φ1: lemma_nat_range_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_nat_range` is `external_body` — finiteness and length of the nat range set are trusted without proof; if the axiom were wrong, `nat_set_size` would inherit the unsoundness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_nat_range` is `external_body` trusting both finiteness and cardinality of nat range sets without proof. This is a foundational axiom that `nat_set_size` depends on — any error would propagate to all downstream set cardinality reasoning.

### φ2: nat_set_size_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `nat_set_size` with `bound == 0` proves the empty set has length 0 — relies on the external_body lemma for the vacuous range `[0, 0)`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The empty set being finite with length 0 is a basic set-theoretic fact. While it chains through the external_body lemma, the property itself is trivially correct and expected.

### φ3: nat_set_size_uniqueness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Any two sets satisfying the same `[0, bound)` membership are extensionally equal — follows from the `=~=` assertion in `nat_set_size`, but the proof is powered by the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two sets with identical membership are extensionally equal — this is the definition of set equality (`=~=`). The biconditional precondition fully determines membership, so `s1 =~= s2` follows directly from extensionality, not from the external_body axiom.

### φ4: nat_set_size_superset
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `nat_set_size` constrains membership to exactly `[0, bound)` via biconditional — an element at `bound` is excluded, but the precondition's biconditional is very strong and may over-constrain the set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The precondition is a biconditional `0 <= i < bound <==> s.contains(i)`, which directly implies `!s.contains(bound)` since `bound < bound` is false. This is a trivial logical consequence of the precondition, not a spec gap.

### φ5: lemma_nat_range_singleton
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A singleton range `[n, n+1)` has length 1 — relies on the external_body axiom; if `lemma_nat_range` were unsound, singleton set finiteness/length would be wrong
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The singleton range having length 1 depends entirely on the `external_body` `lemma_nat_range`. This is mathematically correct but unverified — the trust gap in the axiom applies to all instantiations including this basic case.

