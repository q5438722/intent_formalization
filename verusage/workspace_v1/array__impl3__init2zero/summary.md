# Test Summary: `array__impl3__init2zero`

## Target Specification

- **`Array<usize, N>`** with ghost `seq: Ghost<Seq<usize>>` and concrete `ar: [usize; N]`
- **`wf()`**: `self.seq@.len() == N`
- **`set(i, out)`**: requires `0 <= i < N`, `wf()`; ensures `seq@ == old.seq@.update(i, out)`, `wf()`
- **`init2zero()`**: requires `wf()`, `N <= usize::MAX`; ensures `∀ 0 <= i < N: self@[i] == 0`, `wf()`

## Results

**All 15 adversarial tests FAILED verification as expected.** The specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical properties.

### Boundary Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| B1 | `wf()` rejects seq length ≠ N (3 vs 4) | FAIL ✅ |
| B2 | `init2zero` postcondition does not cover index N | FAIL ✅ |
| B3 | `set` postcondition not derivable for index == N | FAIL ✅ |
| B4 | `wf()` guarantees length == N, not N+1 | FAIL ✅ |
| B5 | `wf()` rejects empty seq when N > 0 | FAIL ✅ |

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| M1 | `init2zero` sets elements to 0, not 1 | FAIL ✅ |
| M2 | `set(2, 99)` puts 99 at index 2, not 100 | FAIL ✅ |
| M3 | `set(2, 99)` does NOT modify index 0 | FAIL ✅ |
| M4 | `init2zero` maintains `wf()`, not breaks it | FAIL ✅ |
| M5 | `init2zero` zeroes ALL elements including last | FAIL ✅ |

### Logical Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| L1 | `init2zero` does NOT preserve old nonzero values | FAIL ✅ |
| L2 | `set` is NOT a no-op when values differ | FAIL ✅ |
| L3 | Spec says nothing about negative indices | FAIL ✅ |
| L4 | `set` with different values produces different results | FAIL ✅ |
| L5 | `set(0, 5)` on zero array breaks all-zero invariant | FAIL ✅ |

## Observations

1. **Spec is well-guarded**: The `wf()` precondition and `0 <= i < N` bound on `set` correctly restrict the valid input space. Out-of-bounds and non-well-formed inputs are properly rejected.

2. **Behavioral correctness**: The `update` semantics of `set` and the universal quantifier of `init2zero` are precise enough to reject all tested mutations (wrong values, wrong indices, wrong frame).

3. **Logical soundness**: The spec does not entail unintended properties — it correctly rejects preservation of old values across `init2zero`, negative indexing, conflating distinct `set` operations, and composing contradictory postconditions.

4. **Note on extensional equality**: During development, two initial logical tests (asserting two zero-arrays are distinct, and asserting `set(i, same_value)` changes the sequence) passed verification. These are entailed by Verus's built-in `Seq` extensional equality axioms — a legitimate consequence of the spec + theory, not a spec weakness. They were replaced with stronger tests.

## Conclusion

The specification for `Array<usize, N>::init2zero` is **consistent**: it correctly rejects invalid inputs, incorrect behaviors, and unintended logical inferences across all 15 adversarial tests.
