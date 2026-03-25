# Adversarial Test Summary: `array__impl2__init2zero`

## Target Specification
- **`Array<A, N>`**: struct with ghost `Seq<A>` and concrete `[A;N]`
- **`wf()`**: `self.seq@.len() == N`
- **`set(i, out)`**: requires `0 <= i < N`, `wf()`; ensures extensional update + `wf()`
- **`init2zero()`**: requires `wf()`, `N <= usize::MAX`; ensures all elements in `[0,N)` are 0 + `wf()`

## Results: **15/15 tests FAILED verification** ✅

All adversarial properties were correctly rejected by the specification.

### Boundary Tests (5/5 failed) ✅
| Test | Property Queried | Result |
|------|-----------------|--------|
| B1 | `wf()` holds with wrong seq length (3 ≠ 4) | FAILED ✅ |
| B2 | `init2zero` covers index N (out of bounds) | FAILED ✅ |
| B3 | `set` postcondition at index N | FAILED ✅ |
| B4 | `wf()` allows length ≠ N after init | FAILED ✅ |
| B5 | `wf()` holds with empty seq (0 ≠ 4) | FAILED ✅ |

### Behavioral Mutation Tests (5/5 failed) ✅
| Test | Property Queried | Result |
|------|-----------------|--------|
| M1 | Element is 1 after init2zero (mutated value) | FAILED ✅ |
| M2 | Old value persists after set (wrong update) | FAILED ✅ |
| M3 | Unrelated index corrupted by set | FAILED ✅ |
| M4 | `!wf()` after init2zero (inverted postcondition) | FAILED ✅ |
| M5 | Length changes after set | FAILED ✅ |

### Logical Tests (5/5 failed) ✅
| Test | Property Queried | Result |
|------|-----------------|--------|
| L1 | Two zero-initialized arrays differ (determinism) | FAILED ✅ |
| L2 | Old value preserved through init2zero | FAILED ✅ |
| L3 | Set implies specific original value | FAILED ✅ |
| L4 | Non-trivial set is identity | FAILED ✅ |
| L5 | Set is non-invertible | FAILED ✅ |

## Conclusion

The specification for `Array::init2zero` and `Array::set` is **consistent** with respect to all 15 adversarial queries tested:
- **Boundary**: Invalid inputs (wrong lengths, out-of-bounds indices) are properly rejected.
- **Behavioral**: Incorrect output mutations are detected and rejected.
- **Logical**: Unintended properties (determinism violations, value preservation through zeroing, unjustified value inference) are correctly not entailed.

The spec is tight enough to reject all tested undesirable properties while being precise about what it guarantees (element-wise zeroing in `[0,N)`, extensional sequence update for `set`, and well-formedness preservation).
