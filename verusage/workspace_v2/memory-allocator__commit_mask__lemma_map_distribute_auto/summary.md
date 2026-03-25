# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_map_distribute_auto.rs`
**Date:** 2026-03-24T11:10:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_map_distribute` is an `external_body` axiom trusting set map distributivity over union without proof. While the property is mathematically true, the lack of verification means any subtle mismatch between the axiom and Verus's internal set representation would go undetected. The other three candidates are false positives — they are correct mathematical consequences of distributivity combined with standard set identities (empty set, union idempotence, associativity).

## True Positives (Spec Issues)

### map_distribute_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_map_distribute` is `external_body` — the distributivity of `map` over `union` is a mathematically true property, but it is trusted without proof. Since `is_bit_set`-style bugs have shown external_body axioms can be subtly wrong, this unverified assumption represents a real trust gap.

## All Candidates

### φ1: map_distribute_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_map_distribute` is `external_body` — the set map distributivity over union is trusted without proof, and if the axiom is wrong for infinite sets or non-injective `f`, it could enable unsound reasoning
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_map_distribute` is `external_body` — the distributivity of `map` over `union` is a mathematically true property, but it is trusted without proof. Since `is_bit_set`-style bugs have shown external_body axioms can be subtly wrong, this unverified assumption represents a real trust gap.

### φ2: map_union_empty_left
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Combining distributivity with empty set union could silently mask issues if `Set::empty().map(f)` doesn't equal `Set::empty()` in Verus's set axioms — the external_body lemma would paper over this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of map distributivity plus the standard set axiom that `Set::empty().union(s) == s` and `Set::empty().map(f) == Set::empty()`. The property `∅ ∪ s).map(f) == s.map(f)` is mathematically correct and expected.

### φ3: map_self_union_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The distributivity lemma combined with union idempotence gives `s.union(s).map(f) == s.map(f)` — if `union` on `Set` is not truly idempotent for all sets (e.g., infinite), the external_body axiom would hide the discrepancy
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s.union(s) == s` is a fundamental set identity (union idempotence), and `s.union(s).map(f) == s.map(f)` follows trivially. This is a correct mathematical property, not a spec gap.

### φ4: auto_trigger_unbounded_instantiation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The auto lemma's universal trigger `s1.map(f).union(s2.map(f))` could cause cascading instantiation — each application produces new `map(f).union(...)` terms that re-trigger the quantifier, potentially leading to solver resource exhaustion or unsound matching loops
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property `s1.union(s2).union(s3).map(f) == s1.map(f).union(s2.map(f)).union(s3.map(f))` is a correct associative application of distributivity. Trigger-based instantiation concerns are solver performance issues, not soundness gaps — the entailed property itself is mathematically correct.

### φ5: map_distribute_non_injective_collapse
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** With a constant (non-injective) function, both sides collapse to `{0}` — the distributivity holds trivially here but the external_body axiom asserts it for all `f` including pathological cases without verification

