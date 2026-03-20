
# 📄 Background & Motivation (Consistency-Centric Version)

## Background

Formal verification systems such as Verus enable developers to express program intent as formal specifications and mechanically prove correctness. In this setting, a specification serves as a semantic contract that defines which behaviors are valid and which are not.

However, a fundamental challenge remains:

> **User intent is inherently latent and cannot be directly observed or quantified.**

As a result, assessing whether a specification is *complete* with respect to user intent is intrinsically difficult. A specification may be logically consistent and verifiable, yet still fail to capture important aspects of the intended semantics, allowing unintended behaviors or interpretations.

---

## Limitations of Existing Approaches

Existing approaches provide only partial views of specification quality:

* **Executable testing (e.g., Rust tests)** validates concrete input-output behaviors, but only over a finite set of samples.
* **Formal verification** ensures that implementations satisfy the specification, but assumes the specification itself is correct.
* **Mutation-based testing** perturbs outputs to check whether incorrect behaviors are rejected, improving behavioral discrimination.

While effective, these approaches share a common limitation:

> **They primarily operate at the level of observable behaviors, and do not fully capture the semantic boundaries of a specification.**

In particular, they fail to address cases where:

* invalid inputs are implicitly admitted,
* unintended logical consequences can be derived, or
* constraints are present but not actually necessary.

---

## Consistency as a Unifying Principle

We observe that the quality of a specification can be fundamentally understood through a single principle:

> **A correct specification should accept all intended semantics and reject all unintended ones.**

We refer to this property as **consistency**.

Crucially, unintended semantics can arise from multiple sources. A specification may be consistent in one dimension while inconsistent in another. Therefore, consistency must be evaluated across several complementary dimensions.

---

## Dimensions of Inconsistency

We identify four primary sources of inconsistency, each corresponding to a distinct way in which a specification may fail to align with user intent:

---

### 1. Behavioral Inconsistency

A specification should distinguish correct from incorrect input-output behaviors.

This is evaluated by:

* executable tests (correct behaviors), and
* output mutation (incorrect behaviors).

These techniques check whether the specification can **accept correct outputs and reject incorrect ones**, capturing consistency at the behavioral level.

---

### 2. Boundary Inconsistency

A specification should precisely characterize the valid input space.

This is evaluated via precondition violation tests, where invalid inputs are constructed and expected to be rejected.

These tests ensure that the specification does not admit inputs that fall outside the intended domain.

---

### 3. Logical Inconsistency

A specification should not admit unintended logical consequences.

Even when behaviors are correct, the specification may allow invalid inferences, such as:

* over-generalized properties,
* unintended guarantees (e.g., determinism), or
* incorrect structural relationships.

This is evaluated via **spec-level falsification tests**, which attempt to derive stronger or incorrect claims and expect such derivations to fail.

---

### 4. Structural Inconsistency

A specification should contain only necessary constraints.

This is evaluated via specification mutation, where constraints are weakened or removed. If verification still succeeds, it indicates that the specification contains redundant or ineffective constraints.

This captures whether the structure of the specification is **minimally sufficient**.

---

## Toward Intent Coverage

Under this unified view, specification completeness can be approximated by the absence of inconsistencies across all dimensions.

Each dimension captures a different failure mode:

* Behavioral inconsistency: incorrect behaviors are admitted
* Boundary inconsistency: invalid inputs are admitted
* Logical inconsistency: invalid conclusions are derivable
* Structural inconsistency: constraints are unnecessary

We define **intent coverage** as the extent to which a specification eliminates these inconsistencies:

[
\text{Intent Coverage} \approx
\text{Behavioral} +
\text{Boundary} +
\text{Logical} +
\text{Structural}
]

---

## Key Insight

Rather than attempting to directly prove that a specification is complete, we adopt a **falsification-oriented approach**:

> We systematically construct adversarial inputs, behaviors, and logical claims, and evaluate whether the specification rejects all unintended semantics.

A key insight of our work is that:

> **Rejecting incorrect behaviors is necessary but not sufficient—specifications must also reject incorrect reasoning.**

By unifying multiple evaluation techniques under the lens of consistency, our approach provides a principled and extensible framework for approximating specification completeness in the absence of observable ground truth.
