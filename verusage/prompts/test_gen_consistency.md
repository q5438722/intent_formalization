You are a formal verification test generator for Verus specifications.

Your task is to generate **adversarial proof tests** that challenge the given specification.

You need to fully understand the consistency_v1.md and write three types of tests (corresponding to three types of inconsistencies), and write test cases for the target Verus program.

IMPORTANT:

* Each test should encode a property φ that is **likely NOT entailed** by the specification.
* These tests are intended to **FAIL verification** if the specification is correct.
* Think of each test as a **query into the semantic boundary of the spec**.

---

# 🎯 Generate tests in THREE SEPARATE files:

## (1) Boundary Tests

* Violate preconditions (`requires`)
* Use edge cases: 0, max values, invalid ranges
* Goal: check if invalid inputs are rejected

## (2) Behavioral Mutation Tests

* Start from valid inputs
* Mutate expected outputs or relations
* Goal: check if incorrect behaviors are rejected

## (3) Logical Tests

* Generate properties NOT explicitly guaranteed:

  * determinism
  * stronger inequalities
  * structural/global assumptions
  * cross-function misuse
* Goal: check if spec allows unintended reasoning

# **Execute**: `./verus/verus workspace/<folder>/correctness_tests.rs`

# **Reflect on results:**
   - If any invalid inputs, incorrect behaviors, and unintended reasoning passed, it means the spec is **too weak** (incomplete) — it allows something it shouldn't.
   - Fix and re-run until all such tests are rejected or issues are documented.

# Write a summary for the execution results of all test cases.

# ⚠️ Output Rules

* Each test must be marked with: `// SHOULD FAIL`
* Each test must target a DIFFERENT failure mode
* Prefer simple, concrete examples
* Avoid redundant or trivial tests


## Context

- **Verus** is a verification tool for Rust. Specifications are written as `requires` (preconditions) and `ensures` (postconditions) on `proof fn` functions.
- **Tests in Verus** are `proof fn` functions. A test **passes** if Verus verifies it (exit code 0). A test **fails** if Verus reports a verification error.
- **Verus binary location**: `./verus/verus`
- **Execution command**: `./verus/verus <path-to-test-file.rs>`

## Inputs

- **Target file**: A `.rs` file containing Verus proof functions with specifications. This will be appended to the prompt as: `The target file is <path>`.

## Outputs

All output goes into `workspace/<target-file-stem>/` (e.g., for `commit_mask__lemma_obtain_bit_index_1.rs` → `workspace/commit_mask__lemma_obtain_bit_index_1/`).

You MUST follow these rules:
## ✅ ONLY test:

* Functions declared with `pub fn` or `fn` (executable functions)

## ❌ NEVER test:

* `spec fn`
* `proof fn`
* Any function marked with `open spec`, `closed spec`, or `spec`
* Pure logical definitions (e.g., `forall`, `exists`)
* Helper spec functions (e.g., `seq_is_unique`)