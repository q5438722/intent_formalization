Read the file {{file_path}}.

This is a Verus (verified Rust) program. Your task is to write **verified test functions** for the function `{{target_function}}`.

## What are verified tests in Verus?

A verified test is a regular `fn` (not `#[test]`) inside a `verus! { }` block that:
1. Has `requires` clauses specifying symbolic preconditions on its parameters.
2. Calls the target function and uses `assert(...)` to check postcondition properties.
3. Is proven correct by the Verus verifier — the assertions must hold for ALL inputs satisfying the preconditions.

## Your task

1. Analyse `{{target_function}}` — its signature, specifications (requires/ensures), and implementation.
2. Write {{num_tests}} diverse verified test functions that:
   - Cover different aspects of the specification (each test checks different ensures clauses).
   - Include edge cases (empty state, boundary values, error paths).
   - Use the function's return value and assert properties from its postconditions.
   - Include loop invariants and decreases clauses where loops are used.
3. Each test function should be named `test_{{target_function}}_gen_N` (N = 1, 2, ...).

## Output format

Output ONLY the test functions, inside a single ```rust code block. Do NOT include `verus! { }` wrapping, module declarations, or imports — just the test function bodies.

Example structure:
```rust
fn test_{{target_function}}_gen_1(/* symbolic params */)
    requires
        /* preconditions */
{
    /* call the function, assert properties */
}

fn test_{{target_function}}_gen_2(/* symbolic params */)
    requires
        /* preconditions */
{
    /* different aspect of the spec */
}
```

{{extra_context}}

IMPORTANT: The tests must be verifiable by Verus. Use symbolic parameters (not concrete values) with requires clauses. Use `assert(...)` for checking properties. Include necessary loop invariants and decreases clauses.
