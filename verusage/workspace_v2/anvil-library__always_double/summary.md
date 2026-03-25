# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_double.rs`
**Date:** 2026-03-24T03:38:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives were identified. φ1 and φ5 each isolate the core intensional gap in the two external_body axioms (`always_unfold` and `always_propagate_forwards` respectively), while φ2 shows how composing them amplifies the trust surface to enable reasoning over nested suffixes without a suffix-composition lemma. φ3 and φ4 are false positives that merely restate φ2's finding through alternative proof paths or universal quantification without identifying new trust assumptions.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** The external_body `always_unfold` bridges an intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures clause. This is an unverified trust assumption — the two formulations are semantically equivalent but intensionally distinct in Verus's closure encoding.

### always_propagate_suffix_composition
- **Confidence:** high
- **Reasoning:** Chaining the two external_body axioms yields `p.satisfied_by(ex.suffix(i).suffix(j))` without ever proving that `suffix(i).suffix(j)` is equivalent to `suffix(i+j)`. The axioms implicitly allow reasoning over nested suffixes that are intensionally distinct from single-offset suffixes, expanding the unverified trust surface beyond what either axiom alone provides.

### always_propagate_zero_redundant
- **Confidence:** medium
- **Reasoning:** `ex.suffix(0)` creates a new `Execution` with closure `|i| f(i+0)`, which is intensionally distinct from `ex` even though semantically equivalent. The external_body `always_propagate_forwards` silently asserts `always(p)` transfers to this intensionally distinct execution at i=0, a concrete and minimal demonstration of the axiom's unverified intensional assumption.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the intensional gap between the forall inside always's closure and a bare forall in the ensures — this unverified axiom could be unsound if Verus's SMT encoding ever distinguishes these closures
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `always_unfold` bridges an intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures clause. This is an unverified trust assumption — the two formulations are semantically equivalent but intensionally distinct in Verus's closure encoding.

### φ2: always_propagate_suffix_composition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining always_propagate_forwards with always_unfold gives access to p on doubly-nested suffixes ex.suffix(i).suffix(j), which is intensionally distinct from ex.suffix(i+j) — the axioms implicitly collapse this distinction without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Chaining the two external_body axioms yields `p.satisfied_by(ex.suffix(i).suffix(j))` without ever proving that `suffix(i).suffix(j)` is equivalent to `suffix(i+j)`. The axioms implicitly allow reasoning over nested suffixes that are intensionally distinct from single-offset suffixes, expanding the unverified trust surface beyond what either axiom alone provides.

### φ3: always_double_unfold_to_bare
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_double composes with double-unfolding to yield p on ex.suffix(i).suffix(j) — if always(always(p)) can be freely unfolded twice, the external_body axioms together provide arbitrary suffix composition without proving suffix(i).suffix(j) == suffix(i+j)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is logically identical to φ2 — it reaches the same conclusion (`p` on double-suffix) via `always_double` + two unfolds instead of `always_propagate_forwards` + one unfold. It identifies no new trust assumption beyond what φ1 and φ2 already cover; `always_double` is a verified lemma that adds no extra unsoundness.

### φ4: always_propagate_unbounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** the two external_body axioms together yield a universally quantified double-suffix property — this unbounded composition amplifies trust surface since suffix nesting never reduces to a single offset without an explicit composition lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the universally quantified version of φ2 — same axiom chain (`always_propagate_forwards` + `always_unfold`) lifted to `forall |i, j|`. It identifies no new trust surface beyond what φ2 already captures; the universal quantification is just a mechanical lifting of the same pattern.

### φ5: always_propagate_zero_redundant
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards at i=0 asserts always(p) holds on ex.suffix(0), but ex.suffix(0) wraps nat_to_state in a new closure |i| f(i+0) which is intensionally distinct from ex — the axiom silently equates these without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `ex.suffix(0)` creates a new `Execution` with closure `|i| f(i+0)`, which is intensionally distinct from `ex` even though semantically equivalent. The external_body `always_propagate_forwards` silently asserts `always(p)` transfers to this intensionally distinct execution at i=0, a concrete and minimal demonstration of the axiom's unverified intensional assumption.

