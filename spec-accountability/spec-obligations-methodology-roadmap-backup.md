# Spec Obligations — Methodology and Roadmap Backup

Archived from `spec-obligations-1.0.md` on 2026-07-24. These sections are not
part of the active semantic-obligation document, but are retained for future
methodology, implementation, and planning work.

---

## Archived Section 7 — Cross-Cutting Evidence Mechanisms

### Self-Composition

Self-composition searches for violations of:

```text
R_spec_f(i,o_1)
∧ R_spec_f(i,o_2)
⇒ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

for Determinism at level `λ`, and:

```text
α_{f,λ}^I(i_1) = α_{f,λ}^I(i_2)
∧ R_spec_f(i_1,o_1)
∧ R_spec_f(i_2,o_2)
⇒ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

for Behavioral Congruence at level `λ`.

`UNSAT` proves the encoded implication, `SAT` yields a counterexample, and
`UNKNOWN` is inconclusive.

### Intent Traceability

Traceability may map specification clauses `γ_j` to intent requirements `r_k`,
but the semantic formulas remain:

```text
Requirements define intent:
R_intent_f(i,o)
  ⇔ ∧_{r_k ∈ R_f*} r_k(i,o)
```

and:

```text
Root Correctness:
[∧_{r_k ∈ R_f*} r_k(i,o)]
⇒ R_spec_f(i,o)
```

For clause-level provenance, Correctness also gives:

```text
R_intent_f(i,o) ⇒ γ_j(i,o)
```

Pairwise mappings explain provenance; requirements do not form a separate
obligation node.

### Drift Analysis

Drift analysis compares:

```text
R_intent_{f,v} → R_intent_{f,v+1}
R_spec_{f,v}   → R_spec_{f,v+1}
```

and:

```text
 (A_bar_{f,v}, ⪯_v)
   →
 (A_bar_{f,v+1}, ⪯_{v+1})
```

including added/removed levels, changed kernels, and changed comparability
relations.

Each mismatch is classified as:

```text
Correctness drift
Completeness drift
Abstraction drift
```

### Assumption Accountability

Every claim records its trusted inputs:

```text
intent relation R_intent_f and its source artifacts
abstraction family A_f and level identifiers
refinement order ⪯ and kernel computations
per-level maps α_{f,λ}^I, α_{f,λ}^O
extraction of I, O, P_f, Q_f
analysis universe Ω_f and measure μ_f
solver and self-composition encoding
decompositions Γ_f and R_f*
human/LLM judgments used for traceability
```

Assumption Accountability is a meta-layer, not a fourth semantic root.

### Representation Well-Formedness

Because `spec_f` is written as `<P_f,Q_f>` but interpreted as a relation, it is
useful to check:

```text
∀i.
  P_f(i) ⇒ ∃o. Q_f(i,o)
```

This prevents the declared precondition from naming an input for which the
postcondition admits no output. It is an intent-free representation sanity
check rather than a fourth root property.

### Composition

Cross-function composition is outside the primitive single-function model. A
future extension should lift relation Correctness, relation Completeness, and
Abstraction across call boundaries.

---

## Archived Section 8 — Status and Roadmap

| Item | Status | Formula |
|---|---|---|
| Correctness root | Defined | `R_intent_f ⊆ R_spec_f` |
| Completeness root | Defined | `¬R_intent_f(i,o) ⇒ ¬R_spec_f(i,o)` |
| Exact intent capture | Defined | `R_spec_f = R_intent_f` |
| Determinism checker | Done | Section 5.3 |
| Per-level Congruence checker | Existing checker; hierarchy generalization planned | Sections 6.3–6.4 |
| Abstraction hierarchy | Defined | Partial order in Section 6.2 |
| Traceability split | Active | Archived Intent Traceability section |
| Clause traceability | Defined | Section 4.4 |
| Clause redundancy | Defined; quantitative evaluation planned | Section 5.4 |
| Drift classification | Planned | Archived Drift Analysis section |
| Assumption accountability | Planned | Archived Assumption Accountability section |
| Cross-function composition | Deferred | Archived Composition section |

---

## Archived Overview Sections 7–8

### Evidence Mechanisms

| Mechanism | Role |
|---|---|
| **Self-Composition** | Searches for counterexamples to Determinism and Behavioral Congruence at each abstraction level. |
| **Intent Traceability** | Requirements define `R_intent_f`; root Correctness checks that every pair satisfying them is accepted. |
| **Clause Traceability** | Maps each false-rejection pair to every clause that rejects it, with a reverse clause-to-pairs index. |
| **Redundancy Measurement** | Reports absolute and proportional overlap between one clause's rejection space and the union rejected by other clauses. |
| **Drift Analysis** | Compares changes in `R_spec_f`, `R_intent_f`, and the abstraction poset. |
| **Assumption Accountability** | Records `R_intent_f`, the abstraction family/order, extraction, solver, `Ω_f`, `μ_f`, and human/LLM judgments. |

Pairwise traceability matrices are explanatory only. Correctness and
Completeness are always relation-inclusion claims.

### Scope Boundary

This taxonomy deliberately concerns one function's input/output relation.
Cross-function composition is deferred and should later lift these same three
roots across call boundaries.
