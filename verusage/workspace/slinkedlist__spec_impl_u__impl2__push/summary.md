# Test Summary: `StaticLinkedList::push`

## File Under Test

`slinkedlist__spec_impl_u__impl2__push.rs` — Implements `push` on a `StaticLinkedList<T, N>`, a doubly-linked list backed by a fixed-size array. The `push` function appends a new unique value to the list, maintaining well-formedness invariants, uniqueness, and node reference stability.

### Key Specs

**`push(&mut self, new_value: &T) -> SLLIndex`**
- **Requires**: `wf()`, `len() < N`, `unique()`, `!@.contains(*new_value)`, `N > 2`
- **Ensures**: `wf()`, `@ == old(@).push(*new_value)`, `len() == old(len) + 1`, existing `get_node_ref` preserved, `get_node_ref(*new_value) == returned index`, `unique()`

**`seq_push_lemma<A>()`** — Containment properties for `Seq::push`
**`seq_push_index_of_lemma<A>()`** — `index_of` preservation after `Seq::push`

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_push_preserves_wf` | Push preserves well-formedness | PASS | ✅ PASS |
| 2 | `test_push_appends` | Push appends to logical sequence | PASS | ✅ PASS |
| 3 | `test_push_increments_len` | Push increments length by 1 | PASS | ✅ PASS |
| 4 | `test_push_preserves_unique` | Push preserves uniqueness | PASS | ✅ PASS |
| 5 | `test_push_returns_ref` | Push returns correct node ref for new value | PASS | ✅ PASS |
| 6 | `test_push_preserves_existing_refs` | Push preserves node refs of existing values | PASS | ✅ PASS |
| 7 | `test_seq_push_contains` | seq_push_lemma: pushed value is contained | PASS | ✅ PASS |
| 8 | `test_seq_push_preserves_existing` | seq_push_lemma: existing values remain contained | PASS | ✅ PASS |
| 9 | `test_seq_push_non_member` | seq_push_lemma: non-members stay non-members | PASS | ✅ PASS |
| 10 | `test_seq_push_index_preserved` | seq_push_index_of_lemma: index preserved | PASS | ✅ PASS |
| 11 | `test_two_pushes` | Two consecutive pushes maintain well-formedness | PASS | ✅ PASS |
| 12 | `test_push_large_n` | Push works with larger N=16 | PASS | ✅ PASS |
| 13 | `test_seq_push_empty` | seq_push_lemma: push onto empty sequence | PASS | ✅ PASS |
| 14 | `test_seq_push_all_indices_preserved` | seq_push_index_of_lemma: all existing indices preserved | PASS | ✅ PASS |

**Verification result**: `14 verified, 0 errors`

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_wf` | Push without `wf()` precondition | FAIL | ❌ FAIL (precondition `wf()` not satisfied) |
| 2 | `test_full_list` | Push when `len() == N` (list full) | FAIL | ❌ FAIL (precondition `len() < N` not satisfied) |
| 3 | `test_duplicate_value` | Push value already in list | FAIL | ❌ FAIL (precondition `!contains` not satisfied) |
| 4 | `test_n_too_small` | Push with N=2 (violates N > 2) | FAIL | ❌ FAIL (precondition `wf()` not satisfied) |

**Verification result**: `0 verified, 4 errors`

### Round 2: Overly Strong Postconditions

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_len_plus_2` | Assert length increases by 2 | FAIL | ❌ FAIL (assertion failed) |
| 2 | `test_seq_unchanged` | Assert view unchanged after push | FAIL | ❌ FAIL (assertion failed) |
| 3 | `test_index_always_zero` | Assert returned index is always 0 | FAIL | ❌ FAIL (assertion failed) |
| 4 | `test_len_equals_n` | Assert length equals N after push | FAIL | ❌ FAIL (assertion failed) |

**Verification result**: `0 verified, 4 errors`

### Round 3: Negated Postconditions

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_not_wf` | Assert `!wf()` after push | FAIL | ❌ FAIL (assertion failed) |
| 2 | `test_len_unchanged` | Assert length same as before push | FAIL | ❌ FAIL (assertion failed) |
| 3 | `test_not_unique` | Assert `!unique()` after push | FAIL | ❌ FAIL (assertion failed) |
| 4 | `test_wrong_ref` | Assert `get_node_ref` returns wrong value | FAIL | ❌ FAIL (assertion failed) |

**Verification result**: `0 verified, 4 errors`

### Round 4: Wrong Specific Values

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_len_from_empty` | Assert length is 2 after push (should be 1) | FAIL | ❌ FAIL (assertion failed) |
| 2 | `test_ref_negative_one` | Assert returned ref is always -1 | FAIL | ❌ FAIL (assertion failed) |
| 3 | `test_push_not_contains` | Assert pushed value NOT contained (contradicts lemma) | FAIL | ❌ FAIL (assertion failed) |
| 4 | `test_len_zero_after_push` | Assert length is 0 after push | FAIL | ❌ FAIL (assertion failed) |

**Verification result**: `0 verified, 4 errors`

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_double_push_same` | Push same value twice (second violates `!contains`) | FAIL | ❌ FAIL (precondition not satisfied) |
| 2 | `test_push_removes_element` | Assert existing element removed after push | FAIL | ❌ FAIL (assertion failed) |
| 3 | `test_push_changes_ref` | Assert existing node ref changed after push | FAIL | ❌ FAIL (assertion failed) |
| 4 | `test_push_beyond_capacity` | Push twice when second exceeds capacity | FAIL | ❌ FAIL (precondition not satisfied) |

**Verification result**: `0 verified, 4 errors`

---

## Overall Assessment

### Correctness: ✅ PASS
All 14 correctness tests verify successfully. The `push` function's postconditions are correct — it maintains well-formedness, appends to the logical sequence, increments length, preserves uniqueness, and correctly tracks node references.

### Completeness: ✅ PASS
All 20 completeness tests fail as expected. The specs reject:
- Missing or violated preconditions (Round 1)
- Overly strong claims about postconditions (Round 2)
- Negations of guaranteed postconditions (Round 3)
- Incorrect specific values (Round 4)
- Cross-function misuse and capacity violations (Round 5)

### Observations
- The `unique()` and `N > 2` preconditions in `push` are technically redundant since `wf()` already implies both (via `value_list_wf()` including `unique()`, and `wf()` including `N > 2`). This is not a bug — it makes the API contract more explicit.
- The specs are tight: no unintended behaviors were allowed through.
