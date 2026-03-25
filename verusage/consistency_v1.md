# 📄 Motivation (Consistency via Entailment-Guided Querying)

## Background

Formal verification systems such as Verus enable developers to encode program intent as formal specifications and mechanically prove correctness. In this setting, a specification defines a semantic boundary: it determines which inputs, behaviors, and logical consequences are admissible.

However, a fundamental challenge remains:

> **User intent is inherently latent and cannot be directly observed or fully specified.**

As a result, a specification may successfully pass all proofs and tests, yet still be *incomplete*: it may admit unintended behaviors or support invalid reasoning that contradicts the developer’s intent.

---

## Limitations of Existing Approaches

Existing techniques evaluate specifications from several perspectives:

* **Executable testing (e.g., Rust tests)** checks concrete input-output behaviors;
* **Formal verification** ensures implementations satisfy the specification;
* **Mutation-based testing** perturbs outputs to check whether incorrect behaviors are rejected.

While effective, these approaches share a key limitation:

> **They focus primarily on behavioral correctness, without capturing the full semantic space of the specification.**

In particular, they do not address whether a specification:

* improperly admits invalid inputs,
* implicitly guarantees properties not stated (e.g., determinism), or
* allows unintended logical inferences.

Critically:

> **Rejecting incorrect outputs does not imply rejecting incorrect reasoning.**

---

## Key Insight: Consistency as Entailment Control

We observe that specification quality can be characterized by a single unifying principle:

> **A correct specification should entail all intended properties, and entail none of the unintended ones.**

This leads to a reformulation of the problem:

> Instead of asking whether a specification is “complete,” we ask whether it **incorrectly entails undesirable properties**.

This perspective shifts the focus from *verification* to *entailment control*.

---

## A Unified View: Queries over the Semantic Space

To operationalize this idea, we view specification evaluation as a process of **querying the specification**.

Given a specification ( S ), we systematically construct a query ( \phi ), and check whether:

[
S \vdash \phi
]

If ( \phi ) is an *undesirable property* but is nevertheless entailed by ( S ), then the specification is inconsistent with respect to the intended semantics.

This view unifies a wide range of existing and new techniques:

* **Behavioral queries**:
  ( \phi ) encodes input-output relations
  (e.g., mutation testing)

* **Boundary queries**:
  ( \phi ) encodes input validity
  (e.g., precondition violation tests)

* **Logical queries**:
  ( \phi ) encodes arbitrary semantic claims
  (e.g., determinism, stronger properties)

---

## From Behavioral Mutation to Semantic Falsification

Under this framework, traditional mutation testing can be seen as a special case:

> It generates queries in the **behavioral space** by mutating outputs.

Our key insight is that:

> **This idea can be generalized from the behavioral space to the entire semantic space.**

In particular, we extend mutation-based testing to **logical properties**, by generating adversarial claims that should not be derivable from the specification.

This enables us to detect a new class of errors:

> **Logical inconsistencies**, where the specification admits unintended reasoning even when all behaviors appear correct.

---

## Our Perspective

Based on the above, we advocate a **falsification-oriented approach** to specification evaluation:

> **Systematically generate undesirable properties (queries) and check whether the specification incorrectly entails them.**

This leads to a unified notion of **consistency**:

> A specification is consistent if it rejects all queries that fall outside the intended semantic space.

---

## Closing Statement

> **Specification completeness is not defined by what a specification can prove, but by what it refuses to entail.**
