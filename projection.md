# 📄 Background & Motivation (Draft)

## Background

Formal verification systems such as Verus enable developers to express program intent as formal specifications and mechanically prove correctness. In this setting, a specification is expected to faithfully capture the *user intent*, including both functional behavior and implicit constraints on valid inputs and outputs.

However, a fundamental challenge arises:

> **user intent is inherently latent and cannot be directly observed or quantified.**

As a result, evaluating whether a specification is *complete* with respect to user intent becomes intrinsically difficult. Unlike traditional testing, where coverage metrics (e.g., branch or path coverage) provide a measurable notion of completeness, there is no direct analogue for specification completeness. A specification may be logically consistent and verifiable, yet still underconstrained, allowing unintended behaviors or invalid inputs.

---

## Limitations of Existing Approaches

Existing evaluation approaches typically fall into two categories:

* **Executable testing (e.g., Rust tests)**:
  These validate concrete input-output behaviors but only explore a finite subset of the input space. They provide strong signals for *behavioral correctness* but cannot guarantee the absence of unintended behaviors.

* **Formal verification (proof checking)**:
  This ensures that a program satisfies its specification for all inputs, but critically depends on the *quality of the specification itself*. If the specification is incomplete or too weak, verification may succeed while failing to capture the intended semantics.

Neither approach alone can adequately assess whether a specification fully captures user intent.

---

## Projection-Based View of Intent Approximation

To address this challenge, we adopt a **projection-based perspective**.
Instead of attempting to directly measure specification completeness with respect to user intent, we approximate it through multiple *observable projections*, each capturing a different aspect of intent.

We identify four complementary projections:

---

### 1. Behavior Projection

Behavior projection captures the expected input-output relation of the system:

[
f : \text{Input} \rightarrow \text{Output}
]

This projection is instantiated via:

* executable tests (e.g., Rust tests), and
* successful verification of functional specifications.

It provides evidence that **intended behaviors are correctly realized**, but is inherently limited to sampled inputs.

---

### 2. Boundary Projection

Boundary projection characterizes the **valid input space**, i.e., which inputs are permitted by the specification.

This is evaluated via *negative proof tests* (e.g., “SHOULD FAIL” cases), where deliberately invalid inputs are expected to violate preconditions.

This projection captures the **negative space of intent**—what should *not* be allowed—complementing behavior projection.

---

### 3. Robustness Projection

Robustness projection measures the **necessity and tightness of constraints** in the specification.

This is typically evaluated through mutation:

* weakening or removing preconditions (`requires`),
* relaxing postconditions (`ensures`), or
* mutating assertions.

If verification still succeeds after such perturbations, it indicates that the specification may be underconstrained.

Thus, robustness projection provides a proxy for the **discriminative strength** of the specification.

---

### 4. Consistency Projection

Consistency projection evaluates whether different *views* of the system agree on the same semantics.

We decompose consistency into two complementary components:

* **Positive consistency**:
  Valid input-output pairs should be *accepted* by the specification:
  [
  \text{Spec} \vdash f(x) = y
  ]

* **Negative consistency**:
  Invalid input-output pairs should be *rejected*:
  [
  \text{Spec} \vdash f(x) \neq y'
  ]

Negative consistency can be operationalized via *oracle mutation*, where correct outputs are perturbed and the specification is expected to refute the mutated behaviors.

This projection captures the specification’s ability to both **admit correct behaviors and exclude incorrect ones**, serving as a critical bridge between correctness and completeness.

---

## Toward Intent Coverage

Taken together, these four projections provide a multi-dimensional approximation of user intent:

* Behavior projection captures **what should happen**
* Boundary projection captures **what should not be allowed**
* Robustness projection captures **which constraints are necessary**
* Consistency projection captures **whether all views agree**

We define **intent coverage** as the combined strength of these projections:

[
\text{Intent Coverage} \approx
\text{Behavior} +
\text{Boundary} +
\text{Robustness} +
\text{Consistency}
]

While none of these projections alone is sufficient, their combination provides a practical and systematic way to approximate specification completeness in the absence of observable ground truth.

---

## Key Insight

Rather than attempting to prove that a specification is complete, our approach follows a *falsification-oriented paradigm*:

> We subject the specification to diverse adversarial signals—invalid inputs, incorrect outputs, and mutated constraints—and assess whether it consistently rejects unintended behaviors.

If a specification remains stable across all projections, it provides strong empirical evidence that it closely approximates user intent.

