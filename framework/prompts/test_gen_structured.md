Read the file {{file_path}}.

This is a Verus (verified Rust) program. Your task is to write **structured verification tests** for the function `{{target_function}}`.

## What are verified tests in Verus?

A verified test is a `proof fn` inside a `verus! { }` block that:
1. Has `requires` clauses specifying symbolic preconditions on its parameters.
2. Calls the target function and uses `assert(...)` to check postcondition properties.
3. Is proven correct (or incorrect) by the Verus verifier.

## Your task

Analyse `{{target_function}}` — its signature, specifications (requires/ensures), and implementation. Then generate tests in **6 sections** using the exact markers below.

### Section 1: Correctness Tests (should PASS)

Tests that verify the spec is correct — valid usages produce valid results.

Include both:
- **Parameterized tests**: `proof fn` with `requires` clauses for arbitrary valid inputs.
- **Concrete tests**: `proof fn` with specific literal values covering boundary cases, powers of 2, edge cases.

### Section 2-6: Completeness Rounds (should FAIL)

Each round targets a different category of incorrectness:

| Round | Category | What to test |
|-------|----------|-------------|
| 1 | **Precondition violations** | Call functions without meeting `requires` clauses |
| 2 | **Overly strong postconditions** | Assert tighter bounds than the spec guarantees |
| 3 | **Negated/contradicted postconditions** | Assert the opposite of what the spec says |
| 4 | **Wrong specific values** | Assert incorrect concrete values |
| 5 | **Cross-function misuse & edge cases** | Chain function results incorrectly, test out-of-range positions |

## Output format

Output ALL test functions inside a single ```rust code block, using these **exact section markers**:

```rust
### CORRECTNESS_TESTS
// Correctness tests that should all PASS
proof fn test_correct_1(...) { ... }
proof fn test_correct_2(...) { ... }

### COMPLETENESS_ROUND_1
// Precondition violations — should all FAIL
proof fn test_r1_1(...) { ... }

### COMPLETENESS_ROUND_2
// Overly strong postconditions — should all FAIL
proof fn test_r2_1(...) { ... }

### COMPLETENESS_ROUND_3
// Negated postconditions — should all FAIL
proof fn test_r3_1(...) { ... }

### COMPLETENESS_ROUND_4
// Wrong specific values — should all FAIL
proof fn test_r4_1(...) { ... }

### COMPLETENESS_ROUND_5
// Cross-function misuse — should all FAIL
proof fn test_r5_1(...) { ... }
```

Do NOT include `verus! { }` wrapping, module declarations, or imports — just the section markers and test function bodies.

{{extra_context}}

## Important Verus-specific notes

1. **Type system**: Verus arithmetic on `usize`/`u64` may produce `int` type. Use `as usize` / `as u64` casts when needed.
2. **`reveal()` calls**: Some `spec fn` are opaque. Use `reveal(fn_name);` if needed.
3. **Bit-vector assertions**: Use `assert(...) by (bit_vector)` for bit-level reasoning.
4. **`#[verifier::external_body]`**: Test the spec interface, not the body.
5. **Proof functions are ghost code**: `proof fn` return values by specification. Call them to obtain witnesses.
6. Write **at least 3 tests per section** for good coverage.
