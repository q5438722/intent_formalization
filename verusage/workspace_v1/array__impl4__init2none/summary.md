# Adversarial Test Summary: `array__impl4__init2none`

## Target Specification

- **`init2none`**: Sets all elements of `Array<Option<T>, N>` to `None`
  - Requires: `old(self).wf()` (seq length == N), `N <= usize::MAX`
  - Ensures: `forall|index:int| 0 <= index < N ==> self@[index].is_None()`, `self.wf()`
- **`set(i, out)`**: Updates element at index `i`
  - Requires: `0 <= i < N`, `old(self).wf()`
  - Ensures: `self.seq@ =~= old(self).seq@.update(i, out)`, `self.wf()`

## Results

### Boundary Tests — 4/4 FAILED ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_init2none_missing_wf` | Missing `wf()` precondition for `init2none` | FAILED (precondition not satisfied) ✅ |
| `test_set_index_equals_n` | Out-of-bounds index `i = N` for `set` | FAILED (precondition not satisfied) ✅ |
| `test_set_missing_wf` | Missing `wf()` precondition for `set` | FAILED (precondition not satisfied) ✅ |
| `test_set_on_empty_array` | Index 0 on empty array (0 < 0 is false) | FAILED (precondition not satisfied) ✅ |

### Behavioral Mutation Tests — 4/4 FAILED ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_init2none_element_is_some` | Mutated output: element is `Some(42)` after `init2none` | FAILED (postcondition not satisfied) ✅ |
| `test_init2none_wrong_length` | Mutated length: `len != N` after `init2none` | FAILED (postcondition not satisfied) ✅ |
| `test_set_value_mutated` | Mutated relation: element is `None` after `set(0, Some(42))` | FAILED (postcondition not satisfied) ✅ |
| `test_set_changes_other_index` | Mutated scope: unrelated index changed by `set(2, ...)` | FAILED (postcondition not satisfied) ✅ |

### Logical Tests — 4/4 FAILED ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_init2none_preserves_seq` | Structural preservation: seq unchanged after `init2none` | FAILED (postcondition not satisfied) ✅ |
| `test_init2none_stronger_length` | Stronger inequality: `len > N` instead of `len == N` | FAILED (postcondition not satisfied) ✅ |
| `test_set_then_init2none_preserves` | Cross-function: `set` value survives `init2none` | FAILED (postcondition not satisfied) ✅ |
| `test_init2none_then_set_still_none` | Cross-function: `init2none` value survives `set` | FAILED (postcondition not satisfied) ✅ |

## Conclusion

**All 12 adversarial tests failed verification as expected.** The specification correctly:

1. **Rejects invalid inputs** — preconditions for `wf()` and index bounds are enforced
2. **Rejects incorrect behaviors** — mutated outputs and relations are not entailed
3. **Rejects unintended reasoning** — preservation, stronger properties, and cross-function misuse are all properly rejected

The specification of `init2none` and `set` appears **consistent and appropriately tight** — it entails no undesirable properties tested here.
