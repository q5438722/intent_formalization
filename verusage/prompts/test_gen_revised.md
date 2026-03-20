# Verus Specification Testing Prompt

You are an expert Verus programmer specializing in formal verification of Rust code.

Think step by step. Break down the problem, analyze each function's `requires`/`ensures` clauses, and reason about correctness properties before writing tests.

---

## Context

- **Verus** is a verification tool for Rust. Specifications are written as `requires` (preconditions) and `ensures` (postconditions) on `proof fn` functions.
- **Tests in Verus** are `proof fn` functions. A test **passes** if Verus verifies it (exit code 0). A test **fails** if Verus reports a verification error.
- **Verus binary location**: `./verus/verus`
- **Execution command**: `./verus/verus <path-to-test-file.rs>`

---

## Inputs

- **Target file**: A `.rs` file containing Verus proof functions with specifications. This will be appended to the prompt as: `The target file is <path>`.

## Outputs

All output goes into `workspace/<target-file-stem>/` (e.g., for `commit_mask__lemma_obtain_bit_index_1.rs` → `workspace/commit_mask__lemma_obtain_bit_index_1/`).

The output folder must contain:
1. `correctness_tests.rs` — tests that should all **pass** (verify successfully)
2. `completeness_round1.rs` through `completeness_round5.rs` — tests that should all **fail** (verification errors)
3. `summary.md` — results table with pass/fail status for every test

---

## Step-by-step Procedure

### Phase 1: Analysis

1. Read the target file. Identify:
   - All `proof fn` functions with `requires`/`ensures` clauses (these are the **specs under test**)
   - All `spec fn` functions and macros (these are **helper definitions** used in specs)
   - Any `#[verifier::external_body]` functions (their specs are trusted, not proven — test their *interface*, not their body)
2. Write a brief plan describing what each spec says in plain English.

### Phase 2: Correctness Testing (should PASS)

**Goal**: Verify that the specs are *correct* — valid usages produce valid results.

1. **Create a standalone test file** (`correctness_tests.rs`) that:
   - Copies all definitions from the target file (structs, spec fns, macros, proof fns with their signatures and bodies)
   - Appends new `proof fn test_*()` functions inside the `verus! { }` block

2. **Write two kinds of tests:**

   **a) Parameterized tests** — proof functions with `requires` clauses that take arbitrary valid inputs:
   ```rust
   // Tests that the spec holds for ANY valid input
   proof fn test_param_basic(a: usize)
       requires a != 0  // satisfy the lemma's precondition
   {
       let b = lemma_obtain_bit_index_1(a);
       assert(b < 64);           // check postcondition 1
       assert(is_bit_set(a, b)); // check postcondition 2
   }
   ```

   **b) Concrete tests** — proof functions with no parameters that use specific literal values:
   ```rust
   proof fn test_concrete_a_1() {
       let b = lemma_obtain_bit_index_1(1usize);
       assert(b < 64);
       assert(is_bit_set(1usize, b));
   }
   ```

   For concrete tests, generate **diverse inputs** covering:
   - Boundary values (smallest valid, largest valid)
   - Powers of 2 (single-bit values)
   - All-bits-set values
   - Mixed bit patterns
   - Values that exercise different bit positions (low bits, mid bits, high bits)

3. **Execute**: `./verus/verus workspace/<folder>/correctness_tests.rs`

4. **Reflect on results:**
   - If ALL tests pass → good. Consider if additional edge cases are missing.
   - If a test FAILS → diagnose:
     - Is the test wrong? (e.g., type error, incorrect assertion) → fix the test.
     - Is the spec genuinely incorrect? → note this in the summary.
   - Fix and re-run until all tests pass or issues are documented.

### Phase 3: Completeness Testing (should FAIL)

**Goal**: Verify that the specs are *tight enough* — invalid usages, wrong assertions, and overly strong claims are rejected.

A completeness test is a `proof fn` that either:
- **Violates a precondition** (calls a lemma without satisfying `requires`), OR
- **Asserts something not guaranteed by the spec** (claims a stronger postcondition than `ensures` provides)

If such a test *passes*, it means the spec is **too weak** (incomplete) — it allows something it shouldn't.

1. **Generate 5 rounds** of incorrect test files, each in a separate file (`completeness_round1.rs` ... `completeness_round5.rs`). Each file is standalone (copies all definitions + adds incorrect tests).

2. **Each round targets a different category of incorrectness:**

   | Round | Category | What to test |
   |-------|----------|-------------|
   | 1 | **Precondition violations** | Call functions without meeting `requires` clauses (e.g., pass 0 when `a != 0` is required) |
   | 2 | **Overly strong postconditions** | Assert tighter bounds than the spec guarantees (e.g., `b < 32` when spec says `b < 64`) |
   | 3 | **Negated/contradicted postconditions** | Assert the opposite of what the spec says (e.g., `!is_bit_set(a, b)`) |
   | 4 | **Wrong specific values** | Assert incorrect concrete values (e.g., for `a=2`, assert `b == 0` when it should be `1`) |
   | 5 | **Cross-function misuse & edge cases** | Chain function results incorrectly, test out-of-range positions, assert unguaranteed relationships |

3. **For each round, execute**: `./verus/verus workspace/<folder>/completeness_roundN.rs`

4. **Reflect on results:**
   - If ALL tests in the round FAIL (verification errors) → good. The specs reject invalid claims.
   - If a test PASSES unexpectedly → diagnose:
     - Is the test actually correct? (you wrote a valid assertion by accident) → rewrite it to be genuinely incorrect.
     - Is the spec genuinely incomplete? (it allows something it shouldn't) → note this as a **spec gap** in the summary.
   - Fix and re-run until all tests fail or spec gaps are documented.

### Phase 4: Summary

Create `summary.md` in the result folder with:
1. **File under test**: Brief description of what the target file defines
2. **Correctness results table**: Test name, description, expected result, actual result
3. **Completeness results table**: One section per round, with test name, what it tests, expected result, actual result
4. **Overall assessment**: Are the specs correct? Are the specs complete? Any gaps found?

---

## Important Verus-specific Notes

1. **Type system**: Verus arithmetic on `usize`/`u64` may produce `int` type. Use `as usize` / `as u64` casts when needed. Watch for type mismatch errors and fix them — they are test bugs, not spec issues.

2. **`reveal()` calls**: Some `spec fn` functions are opaque by default. If you get "cannot determine truth of assertion" errors on `is_bit_set` calls, you may need `reveal(is_bit_set);` in your test function.

3. **Bit-vector assertions**: Use `assert(...) by (bit_vector)` for properties that require bit-level reasoning (e.g., `assert(1u64 >> 64u64 == 0u64) by (bit_vector);`).

4. **`#[verifier::external_body]`**: These functions have specs but no verified body. Test their *spec interface* (do the requires/ensures make sense?) but don't try to verify their implementation.

5. **Proof functions are ghost code**: They exist only for verification. `proof fn` functions return values by specification, not by execution. You call them to obtain witnesses that satisfy their `ensures` clauses.

6. **File structure**: Each test file must be **self-contained** — include `use vstd::prelude::*;`, `fn main() {}`, and a `verus! { }` block containing all definitions and tests. Do NOT use Rust `mod` or `include!` to reference the target file.

---

## Example

Given a target file with:
```rust
proof fn my_lemma(x: u64) -> (y: u64)
    requires x > 0
    ensures y <= x, y > 0
{ ... }
```

**Correctness test** (should pass):
```rust
proof fn test_correct(x: u64) requires x > 0 {
    let y = my_lemma(x);
    assert(y <= x);
    assert(y > 0);
}
```

**Completeness test** (should fail):
```rust
proof fn test_wrong_precondition() {
    let y = my_lemma(0u64); // violates x > 0
}
proof fn test_too_strong() requires x > 0 {
    let y = my_lemma(x);
    assert(y == x); // spec only says y <= x, not y == x
}
```
