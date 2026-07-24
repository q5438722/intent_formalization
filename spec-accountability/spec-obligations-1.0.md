# Spec Obligations — Relation-Based Single-Function Model

## 1. Scope

We evaluate the specification of one function through its concrete
input/output relation.

Following the Spec Determinism paper:

```text
f : I → O
spec_f = <P_f, Q_f>
P_f : I → B
Q_f : I × O → B
```

`I` contains explicit inputs and relevant pre-state. `O` contains the return
value and relevant post-state.

The three semantic roots are:

1. **Correctness** — every input/output pair satisfying intent is accepted by
   the specification.
2. **Completeness** — every input/output pair that does not satisfy intent is
   rejected by the specification.
3. **Abstraction** — the input and output abstraction maps retain exactly the
   properties that should matter.

The goal is not initially to assume that the specification is a function.
The specification is a relation on `I × O`. Determinism and per-level
Congruence state when that relation descends to a mapping.

---

## 2. Core Objects

### 2.1 Specification Relation

The concrete relation accepted by the specification is:

```text
R_spec_f ⊆ I × O

R_spec_f(i,o)
  ⇔ P_f(i) ∧ Q_f(i,o)
```

For any relation `R ⊆ I × O`, define:

```text
dom(R) = { i ∈ I | ∃o. R(i,o) }
R(i)   = { o ∈ O | R(i,o) }
```

Thus:

```text
R_spec_f(i)
  = { o ∈ O | P_f(i) ∧ Q_f(i,o) }
```

### 2.2 Pre/Post Clause Decomposition

Decompose the precondition and postcondition into clauses:

```text
P_f(i)
  ⇔ ∧_{a=1}^{m_P} p_a(i)

Q_f(i,o)
  ⇔ ∧_{b=1}^{m_Q} q_b(i,o)
```

Lift every clause to a predicate on the common pair space `I × O`:

```text
γ_a(i,o)       = p_a(i)                  -- lifted Pre clause
γ_{m_P+b}(i,o) = q_b(i,o)                -- Post clause

Γ_f = { γ_1, ..., γ_m }
m   = m_P + m_Q
```

Then:

```text
R_spec_f(i,o)
  ⇔ ∧_{γ_j ∈ Γ_f} γ_j(i,o)
```

Choose a measure space:

```text
Ω_f ⊆ I × O
(Ω_f, F_f, μ_f)
```

By default `Ω_f = I × O`. For finite spaces, `μ_f(X)=|X|`. For infinite
spaces, `μ_f` must be an explicit probability measure, bounded-domain counting
measure, or sampling distribution.

Require every `Acc_j`, `Rej_j`, and every intent-overlay set used below to
belong to the measurable family `F_f`.

For each clause `γ_j`, define its acceptance and rejection spaces:

```text
Acc_j = { (i,o) ∈ Ω_f | γ_j(i,o) }
Rej_j = { (i,o) ∈ Ω_f | ¬γ_j(i,o) }
      = Ω_f \ Acc_j
```

Within `Ω_f`, the whole specification accepts the intersection and rejects the
union:

```text
Acc_spec^Ω
  = R_spec_f ∩ Ω_f
  = ∩_{j=1}^m Acc_j

Rej_spec^Ω
  = Ω_f \ R_spec_f
  = ∪_{j=1}^m Rej_j
```

These spaces are shared by Clause Correctness and Clause Redundancy.

### 2.3 Abstraction Levels

Allow a family of abstraction levels:

```text
A_f = { A_λ | λ ∈ L_f }

A_λ
  = (α_{f,λ}^I, α_{f,λ}^O)

α_{f,λ}^I : I → I_hat_λ
α_{f,λ}^O : O → O_hat_λ
```

Each level contains one input abstraction map and one output abstraction map.

They induce equivalence relations:

```text
i_1 ≡_{f,λ}^I i_2
  ⇔ α_{f,λ}^I(i_1) = α_{f,λ}^I(i_2)

o_1 ≡_{f,λ}^O o_2
  ⇔ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

The paper's per-function output equivalence `eq_f` corresponds to selecting
one level `λ` and using `≡_{f,λ}^O`.

### 2.4 Intent Relation

Intent is represented directly as a concrete relation:

```text
R_intent_f ⊆ I × O
```

An ideal mapping `f*` may be one source from which this relation is constructed,
but the obligation taxonomy depends only on `R_intent_f`. It does not require a
single distinguished or ideal abstraction level.

The intent relation may contain multiple outputs for one input when intent
permits multiple concrete behaviors.

### 2.5 Relation Fibers

For each input:

```text
R_spec_f(i)   = { o | R_spec_f(i,o) }
R_intent_f(i) = { o | R_intent_f(i,o) }
```

The desired endpoint is:

```text
R_spec_f = R_intent_f
```

or equivalently:

```text
∀i. R_spec_f(i) = R_intent_f(i)
```

Correctness proves one inclusion; Completeness proves the other.

### 2.6 Kernels of Abstraction Maps

For any abstraction map `α : X → X_hat`, define:

```text
ker(α)
  = { (x_1,x_2) | α(x_1) = α(x_2) }
```

Two abstraction maps express the same distinctions when their kernels are
equal, even if they use differently named abstract values.

---

## 3. Semantic Obligation Tree

```text
Specification Obligations for f
├── 1. Correctness: R_intent_f ⊆ R_spec_f
│   ├── 1.1 Domain Correctness
│   ├── 1.2 Clause Correctness
│   └── 1.3 Clause Traceability
├── 2. Completeness: ¬R_intent_f(i,o) ⇒ ¬R_spec_f(i,o)
│   ├── 2.1 Domain Completeness
│   ├── 2.2 Determinism
│   └── 2.3 Clause Redundancy
└── 3. Abstraction Hierarchy
    ├── 3.1 Refinement Partial Order
    ├── 3.2 Domain Congruence at Every Level
    └── 3.3 Behavioral Congruence at Every Level
```

Correctness, Completeness, and Abstraction are the only semantic roots.
Traceability, non-vacuity, self-composition, drift analysis, and assumption
accountability are evidence mechanisms.

---

## 4. Root 1 — Correctness

### 4.1 Root Formula

Correctness means that every intent-satisfying pair is accepted by the
specification:

```text
Correct_f
  ⇔ R_intent_f ⊆ R_spec_f
```

Expanded:

```text
∀i,o.
  R_intent_f(i,o)
  ⇒ R_spec_f(i,o)
```

Using `P_f` and `Q_f`:

```text
∀i,o.
  R_intent_f(i,o)
  ⇒ P_f(i) ∧ Q_f(i,o)
```

Correctness prevents false rejection.

If intent is decomposed into requirements:

```text
R_f* = { r_1, ..., r_n }

R_intent_f(i,o)
  ⇔ ∧_{r_k ∈ R_f*} r_k(i,o)
```

then root Correctness already includes those requirements:

```text
∀i,o.
  [∧_{r_k ∈ R_f*} r_k(i,o)]
  ⇒ R_spec_f(i,o)
```

Requirement coverage is therefore not a separate semantic obligation.

### 4.2 Domain Correctness

Every input for which intent admits an output must occur in the specification
relation:

```text
dom(R_intent_f) ⊆ dom(R_spec_f)
```

Expanded:

```text
∀i.
  [∃o. R_intent_f(i,o)]
  ⇒ [∃o. R_spec_f(i,o)]
```

This is the domain projection of root Correctness.

There is no separate Behavioral Correctness node: acceptance of every
intent-satisfying pair is already exactly the root Correctness formula.

### 4.3 Clause Correctness

Using the shared decomposition `Γ_f` from Section 2.2, each clause must accept
every intent-satisfying pair:

```text
∀γ_j ∈ Γ_f, i, o.
  R_intent_f(i,o)
  ⇒ γ_j(i,o)
```

The conjunction of all Clause Correctness obligations is equivalent to root
Correctness.

### 4.4 Clause Traceability

Define the false-rejection relation:

```text
FR_f
  = R_intent_f \ R_spec_f
```

For every falsely rejected pair, define the set of clauses that reject it:

```text
Trace_f : FR_f → 2^{Γ_f} \ {∅}

Trace_f(i,o)
  = { γ_j ∈ Γ_f | ¬γ_j(i,o) }
```

Within the analysis universe, equivalently:

```text
∀(i,o) ∈ FR_f ∩ Ω_f.
Trace_f(i,o)
  = { γ_j ∈ Γ_f | (i,o) ∈ Rej_j }
```

Clause Traceability requires:

```text
∀(i,o) ∈ FR_f.
  Trace_f(i,o) ≠ ∅
```

Because `R_spec_f` is the conjunction of all clauses, this trace is exact:

```text
(i,o) ∈ FR_f
  ⇒
  [γ_j ∈ Trace_f(i,o) ⇔ ¬γ_j(i,o)]
```

Interpretation:

```text
|Trace_f(i,o)| = 1
  -- one clause directly rejects the intent pair

|Trace_f(i,o)| > 1
  -- multiple clauses simultaneously reject the intent pair
```

The reverse index for clause `γ_j` is:

```text
FR_j
  = { (i,o) ∈ R_intent_f | ¬γ_j(i,o) }
  = R_intent_f ∩ { (i,o) | ¬γ_j(i,o) }
```

Thus Clause Traceability supports both queries:

```text
pair → rejecting clause set
clause → falsely rejected intent-pair set
```

It is a derived localization construction rather than additional semantic
strength: root Correctness identifies that a false rejection exists; Clause
Traceability identifies the responsible clause or clauses. Its accountable
failure modes are incorrect clause extraction or lost clause provenance.

---

## 5. Root 2 — Completeness

### 5.1 Root Formula

Completeness means that every input/output pair that does **not** satisfy
intent is rejected by the specification:

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

Using `P_f` and `Q_f`:

```text
∀i,o.
  ¬R_intent_f(i,o)
  ⇒
  ¬[P_f(i) ∧ Q_f(i,o)]
```

Completeness prevents false acceptance.

### 5.2 Domain Completeness

If an input belongs to the specification relation's domain, it must also
belong to the intent relation's domain:

```text
∀i.
  i ∈ dom(R_spec_f)
  ⇒ i ∈ dom(R_intent_f)
```

Equivalently:

```text
dom(R_spec_f) ⊆ dom(R_intent_f)
```

There is no separate Behavioral Completeness node: rejection of non-intent
outputs is already exactly the root Completeness formula.

### 5.3 Determinism

At a selected abstraction level `A_λ`, all outputs admitted for one concrete
input must have the same output abstraction:

```text
Deterministic_f(A_λ)
  ⇔
∀i,o_1,o_2.
  R_spec_f(i,o_1)
  ∧ R_spec_f(i,o_2)
  ⇒ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

Expanded in the paper's notation:

```text
∀i,o_1,o_2.
  P_f(i)
  ∧ Q_f(i,o_1)
  ∧ Q_f(i,o_2)
  ⇒ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

This is the existing intent-free Determinism check instantiated at level `λ`.

If:

```text
R_intent_f ⊆ R_spec_f                  -- Correctness
dom(R_spec_f) = dom(R_intent_f)        -- domain equality

∀i ∈ dom(R_intent_f).
  ∃c ∈ O_hat_λ.
    R_intent_f(i)
      = { o ∈ O | α_{f,λ}^O(o) = c }
                                          -- one complete λ-class per input
```

then Determinism at level `λ` is equivalent to root Completeness. Correctness
supplies the complete intended output class; Domain Completeness rules out
extra input-domain points; Determinism prevents a second class.

### 5.4 Clause Redundancy

For clause `γ_j`, let the rejection space covered by all other clauses be:

```text
Rej_{-j}
  = ∪_{k ≠ j} Rej_k
```

The part of `γ_j`'s rejection space already covered by another clause is:

```text
Rej_j^overlap
  = Rej_j ∩ Rej_{-j}
```

The part rejected uniquely by `γ_j` is:

```text
Rej_j^unique
  = Rej_j \ Rej_{-j}
```

#### Absolute Redundancy

The absolute redundant rejection mass is:

```text
RedMass_j
  = μ_f(Rej_j ∩ Rej_{-j})
```

This reports the size of the region rejected by `γ_j` that is also rejected by
at least one other clause.

#### Redundancy Ratio

When `0 < μ_f(Rej_j) < ∞`, define:

```text
RedRatio_j
  = μ_f(Rej_j ∩ Rej_{-j})
    / μ_f(Rej_j)
```

Equivalently, the unique-rejection ratio is:

```text
UniqueRatio_j
  = μ_f(Rej_j \ Rej_{-j})
    / μ_f(Rej_j)
  = 1 - RedRatio_j
```

Interpretation:

```text
RedRatio_j = 0
  -- none of γ_j's rejected space is covered by other clauses

RedRatio_j = 1
  -- all of γ_j's rejected space is already covered by other clauses
```

If `μ_f(Rej_j)=0`, report `γ_j` separately as a zero-rejection
(almost-everywhere accepting) clause. If `μ_f(Rej_j)=∞`, the ratio is also
undefined unless an additional normalized measure is supplied.

#### Full Redundancy Within the Analysis Universe

Clause `γ_j` is fully redundant within `Ω_f` when:

```text
Rej_j ⊆ Rej_{-j}
```

Equivalently:

```text
Rej_j^unique = ∅
```

and:

```text
R_spec_f ∩ Ω_f
  =
R_spec_f^{-j} ∩ Ω_f
```

where:

```text
R_spec_f^{-j}(i,o)
  ⇔ ∧_{k ≠ j} γ_k(i,o)
```

Thus removing `γ_j` does not change the specification relation **inside**
`Ω_f`. This is global logical redundancy only when `Ω_f = I × O` (or when a
separate symbolic proof establishes the equality over all of `I × O`).

#### Relation to Correctness and Completeness

Redundancy itself is intent-free: it only compares clause rejection spaces.
Overlaying intent separates two additional regions:

```text
FalseRej_j
  = Rej_j ∩ (R_intent_f ∩ Ω_f)

NonIntentRej_j
  = Rej_j ∩ (Ω_f \ R_intent_f)
```

Clause Correctness restricted to `Ω_f` requires:

```text
FalseRej_j = ∅
```

This discharges global Clause Correctness only when `Ω_f = I × O`.

Under Clause Correctness, `Rej_j^unique` measures the behavior for which
`γ_j` is the sole rejecting clause, and every such pair is outside intent.

#### Measure Dependence

`RedMass_j` and `RedRatio_j` depend on `Ω_f` and `μ_f`. Exact full redundancy
is a set-inclusion property and can be checked symbolically. Quantitative
redundancy requires counting, integration, or sampling; the chosen universe
and measure are part of the trusted base.

---

## 6. Root 3 — Abstraction Hierarchy

### 6.1 Root Formula

Two abstraction levels are equivalent when they induce the same input and
output partitions:

```text
A_λ ≡ A_κ
  ⇔ ker(α_{f,λ}^I) = ker(α_{f,κ}^I)
     ∧ ker(α_{f,λ}^O) = ker(α_{f,κ}^O)
```

Let:

```text
A_bar_f = A_f / ≡
```

be the abstraction levels modulo abstract-value renaming.

The abstraction root requires:

```text
Abstract_f
  ⇔ (A_bar_f, ⪯) is a partial order
     ∧ ∀[A_λ] ∈ A_bar_f.
         DomainCongruent_f(A_λ)
         ∧ BehavioralCongruent_f(A_λ)
```

Thus there is no single ideal abstraction. Multiple valid levels may coexist,
but they must form a refinement hierarchy and the specification relation must
be congruent at every level.

### 6.2 Refinement Partial Order

Define “finer than or equal to” by kernel inclusion:

```text
[A_λ] ⪯ [A_κ]
  ⇔ ker(α_{f,λ}^I) ⊆ ker(α_{f,κ}^I)
     ∧ ker(α_{f,λ}^O) ⊆ ker(α_{f,κ}^O)
```

Interpretation:

```text
[A_λ] ⪯ [A_κ]
  -- λ distinguishes at least everything κ distinguishes
  -- κ is coarser than or equal to λ
```

The order is:

```text
reflexive
antisymmetric on A_bar_f
transitive
```

It is generally not total: two abstraction levels may be incomparable when
one is finer on one distinction but coarser on another.

Kernel inclusion is equivalent to factorization on the images. If
`[A_λ] ⪯ [A_κ]`, there exist maps:

```text
h_{λκ}^I : image(α_{f,λ}^I) → image(α_{f,κ}^I)
h_{λκ}^O : image(α_{f,λ}^O) → image(α_{f,κ}^O)
```

such that:

```text
α_{f,κ}^I = h_{λκ}^I ∘ α_{f,λ}^I
α_{f,κ}^O = h_{λκ}^O ∘ α_{f,λ}^O
```

### 6.3 Domain Congruence at Every Level

For every abstraction level `A_λ`:

```text
∀i_1,i_2.
  α_{f,λ}^I(i_1) = α_{f,λ}^I(i_2)
  ⇒
  [i_1 ∈ dom(R_spec_f) ⇔ i_2 ∈ dom(R_spec_f)]
```

This ensures that whether the specification relation is defined depends only
on the abstract input at level `λ`.

### 6.4 Behavioral Congruence at Every Level

For every abstraction level `A_λ`:

```text
∀i_1,i_2,o_1,o_2.
  α_{f,λ}^I(i_1) = α_{f,λ}^I(i_2)
  ∧ R_spec_f(i_1,o_1)
  ∧ R_spec_f(i_2,o_2)
  ⇒ α_{f,λ}^O(o_1) = α_{f,λ}^O(o_2)
```

Behavioral Congruence contains Determinism at level `λ` as the special case
`i_1 = i_2`.

Together, Domain and Behavioral Congruence ensure that each level induces a
well-defined partial mapping:

```text
R_hat_spec_f^λ : I_hat_λ ⇀ O_hat_λ
```

defined by:

```text
R_hat_spec_f^λ(α_{f,λ}^I(i))
  = α_{f,λ}^O(o)
whenever R_spec_f(i,o)
```

### 6.5 Cross-Level Coherence

If `[A_λ] ⪯ [A_κ]`, the induced mappings commute with the factor maps:

```text
h_{λκ}^O ∘ R_hat_spec_f^λ
  =
R_hat_spec_f^κ ∘ h_{λκ}^I
```

on the domain where the induced mappings are defined.

This coherence follows from the common concrete relation plus Congruence at
both levels; it is a theorem of the hierarchy rather than another obligation.

---

## Appendix A — Key Open Questions

1. **Core approximation problem.** We assume that an authoritative but latent
   binary intent relation exists:

   ```text
   R_intent_f* ⊆ I × O
   ```

   Given only finite or indirect evidence `E_t` (natural-language
   requirements, examples, counterexamples, reference implementations, tests,
   and human judgments), how can we construct and iteratively refine
   approximating relations:

   ```text
   E_t ↦ R_hat_intent_f^t ⊆ I × O
   ```

   toward `R_intent_f*`?

   In the active taxonomy, `R_intent_f` denotes the currently selected
   approximation `R_hat_intent_f^t`; the starred relation denotes the assumed
   but unobservable target.

   The two approximation errors are already characterized by the existing
   properties:

   ```text
   missing_t
     = R_intent_f* \ R_hat_intent_f^t    -- Correctness-side error

   spurious_t
     = R_hat_intent_f^t \ R_intent_f*    -- Completeness-side error
   ```

   The abstraction hierarchy and per-level Congruence properties constrain how
   each approximating relation may induce mappings at different abstraction
   levels. The remaining research problem is therefore not which semantic
   properties an approximation should satisfy, but how to obtain evidence,
   update the relation, and justify that successive approximations are moving
   closer to an unobservable target relation.

2. How should the authoritative intent relation `R_intent_f` be elicited from
   natural-language intent, examples, or an ideal mapping?
3. Can some Correctness/Completeness obligations be checked without explicitly
   materializing `R_intent_f`?
4. Which abstraction levels should be included, and how should stable level
   identities be assigned across versions?
5. What analysis universe `Ω_f` and measure `μ_f` make quantitative redundancy
   meaningful for unbounded or symbolic input/output spaces?
6. When exact model counting is unavailable, which sampling estimator and
   confidence interval should be used for `RedRatio_j`?
7. Should requirement decompositions use minimal supporting clause sets or
   proof-derived explanations instead of pairwise matrices?
8. How should abstraction levels and their partial order be transported when
   concrete representations change across versions?
