# Adversarial Proof Test Summary: `push` on `StaticLinkedList`

**Target**: `slinkedlist__spec_impl_u__impl2__push.rs`
**Function under test**: `pub fn push(&mut self, new_value: &T) -> SLLIndex`

## Results: All 15 tests FAILED verification ✅

All tests correctly failed, confirming the specification properly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (5/5 failed) — `boundary_tests.rs`

| Test | Failure Mode | Violated Precondition | Result |
|---|---|---|---|
| `test_push_missing_wf` | Missing `wf()` | `old(self).wf()` | ✅ FAIL |
| `test_push_when_full` | List at capacity `len == N` | `old(self).len() < N` | ✅ FAIL |
| `test_push_duplicate_value` | Value already in list | `!old(self)@.contains(*new_value)` | ✅ FAIL |
| `test_push_second_push_full` | Sequential push overflow | `old(self).len() < N` (2nd call) | ✅ FAIL |
| `test_get_next_negative_index` | Negative index `-1` | `0 <= index < N` | ✅ FAIL |

**Finding**: `old(self).unique()` is redundant in the precondition — it is already implied by `wf()` (which includes `value_list_wf()` → `unique()`). The verifier derives this even though `wf()` is a closed spec.

---

### Behavioral Mutation Tests (5/5 failed) — `behavioral_tests.rs`

| Test | Mutated Property | Correct Postcondition | Result |
|---|---|---|---|
| `test_push_length_unchanged` | `len == old_len` | `len == old_len + 1` | ✅ FAIL |
| `test_push_length_increases_by_two` | `len == old_len + 2` | `len == old_len + 1` | ✅ FAIL |
| `test_push_seq_unchanged` | `self@ =~= old@` | `self@ == old@.push(*v)` | ✅ FAIL |
| `test_push_value_at_front` | `self@[0] == *v` | Value appended at end | ✅ FAIL |
| `test_push_loses_unique` | `!unique()` | `unique()` preserved | ✅ FAIL |

---

### Logical Tests (5/5 failed) — `logical_tests.rs`

| Test | Unintended Property Tested | Why Not Entailed | Result |
|---|---|---|---|
| `test_push_index_always_zero` | `ret == 0` | Index depends on free list state | ✅ FAIL |
| `test_push_result_bounded_by_len` | `ret < len` | Index is array slot `[0,N)`, not bounded by len | ✅ FAIL |
| `test_push_free_list_len_unchanged` | `free_list_len` preserved | Internal state; `wf()` is opaque | ✅ FAIL |
| `test_push_value_list_head_unchanged` | `value_list_head` preserved | Internal state; not in postconditions | ✅ FAIL |
| `test_push_result_always_positive` | `ret > 0` | Index 0 is a valid free list slot | ✅ FAIL |

---

## Conclusion

The `push` specification is **well-bounded**: it correctly rejects all 15 adversarial queries across boundary, behavioral, and logical categories. The spec properly:

1. **Guards inputs**: Invalid states, full lists, and duplicates are rejected by preconditions.
2. **Constrains outputs**: Mutated length, sequence, and uniqueness properties are all rejected.
3. **Limits reasoning**: Internal state (`free_list_len`, `value_list_head`) and arbitrary return value assumptions are not derivable from the closed `wf()` and opaque `get_node_ref()`.

**Minor observation**: The `unique()` precondition on `push` is redundant given `wf()`, since the verifier can derive it through the closed spec chain `wf() → value_list_wf() → unique()`.
