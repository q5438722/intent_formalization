# Spec Obligations — Relation-Based Overview

## 1. Core Model

For one function:

```text
f : I → O
spec_f = <P_f, Q_f>
```

The specification denotes a concrete input/output relation:

```text
R_spec_f(i,o)
  ⇔ P_f(i) ∧ Q_f(i,o)
```

Decompose Pre/Post into clauses and lift them to the pair space:

```text
P_f(i)   ⇔ ∧_a p_a(i)
Q_f(i,o) ⇔ ∧_b q_b(i,o)

Γ_f = {γ_1,...,γ_m}

γ_a(i,o)       = p_a(i)
γ_{m_P+b}(i,o) = q_b(i,o)

R_spec_f(i,o) ⇔ ∧_{γ_j∈Γ_f} γ_j(i,o)
```

Each lifted clause has an acceptance and rejection space over an explicit
analysis universe `Ω_f ⊆ I×O`:

```text
(Ω_f, F_f, μ_f)  -- measure space

Acc_j = { (i,o)∈Ω_f | γ_j(i,o) }
Rej_j = Ω_f \ Acc_j
```

Space size is measured by `μ_f`; finite cardinality is the special case
`μ_f(X)=|X|`. All measured clause and intent-overlay sets must belong to
`F_f`.

Allow multiple abstraction levels:

```text
A_f = { A_λ | λ∈L_f }

A_λ = (α_{f,λ}^I, α_{f,λ}^O)

α_{f,λ}^I : I → I_hat_λ
α_{f,λ}^O : O → O_hat_λ
```

Intent is represented directly as `R_intent_f ⊆ I×O`. An ideal mapping may be
one source for this relation, but no single abstraction level is designated as
ideal.

Thus both the specification and intent are relations on the same concrete
space:

```text
R_spec_f, R_intent_f ⊆ I × O
```

---

## 2. Three Root Properties

### Correctness

Every intent-satisfying pair is accepted by the specification:

```text
Correct_f
  ⇔ R_intent_f ⊆ R_spec_f
```

or:

```text
∀i,o.
  R_intent_f(i,o)
  ⇒ R_spec_f(i,o)
```

### Completeness

Every input/output pair that does not satisfy intent is rejected:

```text
Complete_f
  ⇔ ∀i,o.
       ¬R_intent_f(i,o)
       ⇒ ¬R_spec_f(i,o)
```

Equivalently:

```text
R_spec_f ⊆ R_intent_f
```

### Abstraction Hierarchy

For a map `α`, define:

```text
ker(α) = { (x_1,x_2) | α(x_1)=α(x_2) }
```

Identify levels that have equal input/output kernels. On the resulting
equivalence classes, define refinement:

```text
[A_λ] ⪯ [A_κ]
  ⇔ ker(α_{f,λ}^I) ⊆ ker(α_{f,κ}^I)
     ∧ ker(α_{f,λ}^O) ⊆ ker(α_{f,κ}^O)
```

The abstraction root requires:

```text
(A_f/≡, ⪯) is a partial order

and every A_λ satisfies
Domain Congruence ∧ Behavioral Congruence.
```

Correctness and Completeness together give:

```text
R_spec_f = R_intent_f
```

---

## 3. Obligation Tree

```text
Specification Obligations for f
├── 1. Correctness: R_intent_f ⊆ R_spec_f
│   ├── Domain Correctness
│   ├── Clause Correctness
│   └── Clause Traceability
├── 2. Completeness: ¬R_intent_f(i,o) ⇒ ¬R_spec_f(i,o)
│   ├── Domain Completeness
│   ├── Determinism
│   └── Clause Redundancy
└── 3. Abstraction Hierarchy
    ├── Refinement Partial Order
    ├── Domain Congruence at Every Level
    └── Behavioral Congruence at Every Level
```

---

## 4. Correctness Children

For relation `R`, define:

```text
dom(R) = { i | ∃o. R(i,o) }
R(i)   = { o | R(i,o) }
```

| Property | Formula |
|---|---|
| **Domain Correctness** | `dom(R_intent_f) ⊆ dom(R_spec_f)` |
| **Clause Correctness** | For every lifted spec clause `γ_j`, `R_intent_f(i,o) ⇒ γ_j(i,o)` |
| **Clause Traceability** | For `(i,o)∈R_intent_f\R_spec_f`, `Trace_f(i,o)={γ_j∈Γ_f | ¬γ_j(i,o)}`. |

There is no separate Behavioral Correctness property: acceptance of every
intent-satisfying pair is already the root Correctness definition.

---

## 5. Completeness Children

| Property | Formula |
|---|---|
| **Domain Completeness** | `i∈dom(R_spec_f) ⇒ i∈dom(R_intent_f)` |
| **Determinism at level λ** | `R_spec_f(i,o_1) ∧ R_spec_f(i,o_2) ⇒ α_{f,λ}^O(o_1)=α_{f,λ}^O(o_2)` |
| **Clause Redundancy** | `RedRatio_j = μ_f(Rej_j ∩ ⋃_{k≠j}Rej_k) / μ_f(Rej_j)` when `0<μ_f(Rej_j)<∞`. |

There is no separate Behavioral Completeness property: rejecting non-intent
outputs is already the root Completeness definition.

`γ_j` is fully redundant when:

```text
Rej_j ⊆ ⋃_{k≠j} Rej_k
```

The absolute redundant mass is:

```text
RedMass_j = μ_f(Rej_j ∩ ⋃_{k≠j}Rej_k)
```

If `μ_f(Rej_j)=0` or `∞`, `RedRatio_j` is undefined without an additional
normalization; zero-rejection clauses are reported separately.

The full-redundancy condition means that removing `γ_j` does not change
`R_spec_f` **within `Ω_f`**. It is a global logical redundancy result only when
`Ω_f=I×O`. The unique-rejection ratio is `1-RedRatio_j`.

Given Correctness, Domain Completeness, and intent fibers that are complete
output-abstraction classes at level `λ`, Determinism at `λ` is equivalent to
root Completeness.

---

## 6. Abstraction Hierarchy

| Property | Formula |
|---|---|
| **Refinement Partial Order** | `[A_λ]⪯[A_κ]` iff both input and output kernels at `λ` are subsets of those at `κ`. |
| **Domain Congruence at level λ** | `α_{f,λ}^I(i_1)=α_{f,λ}^I(i_2) ⇒ [i_1∈dom(R_spec_f) ⇔ i_2∈dom(R_spec_f)]` |
| **Behavioral Congruence at level λ** | `α_{f,λ}^I(i_1)=α_{f,λ}^I(i_2) ∧ R_spec_f(i_1,o_1) ∧ R_spec_f(i_2,o_2) ⇒ α_{f,λ}^O(o_1)=α_{f,λ}^O(o_2)` |

Every level must satisfy both Congruence formulas, inducing:

```text
R_hat_spec_f^λ : I_hat_λ ⇀ O_hat_λ
```

Behavioral Congruence at level `λ` contains Determinism at `λ` as the special
case `i_1=i_2`.

If `[A_λ]⪯[A_κ]`, the coarser maps factor through the finer maps and the
induced partial mappings commute:

```text
h_{λκ}^O ∘ R_hat_spec_f^λ
  =
R_hat_spec_f^κ ∘ h_{λκ}^I
```
