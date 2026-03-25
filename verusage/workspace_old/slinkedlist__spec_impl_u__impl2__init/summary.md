# Specification Test Summary

## File Under Test

`slinkedlist__spec_impl_u__impl2__init.rs` — Defines a `StaticLinkedList<T, N>` backed by a fixed-size array with separate value and free linked lists. The main function under test is `init(&mut self)`, which initializes the list to an empty state with all nodes in the free list. Also tested: `set_value`, `set_next`, `set_prev` (external_body helper functions) and `len()`.

### Key Specs
- **`init`**: requires `N > 2`, `N < i32::MAX`, `array_wf()`; ensures `wf()`, view is empty, length is 0
- **`set_value/set_next/set_prev`**: require `array_wf()`; ensure frame conditions (only the targeted field changes)
- **`len()`**: returns `value_list_len`; when `wf()` holds, equals `view.len()`

---

## Correctness Results (`correctness_tests.rs`)

All 13 tests **PASS** (verification successful).

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_init_n5_postconditions` | Init N=5, assert wf() and empty view | PASS | PASS ✅ |
| 2 | `test_init_n3_postconditions` | Init N=3 (minimum valid), assert postconditions | PASS | PASS ✅ |
| 3 | `test_init_n100_postconditions` | Init N=100, assert postconditions | PASS | PASS ✅ |
| 4 | `test_init_len_returns_zero` | After init, `len()` returns 0 | PASS | PASS ✅ |
| 5 | `test_init_view_len_zero` | After init, `view.len() == 0` | PASS | PASS ✅ |
| 6 | `test_init_spec_len_zero` | After init, `spec_len() == 0` | PASS | PASS ✅ |
| 7 | `test_post_empty_view_properties` | Proof: postconditions imply empty view properties | PASS | PASS ✅ |
| 8 | `test_post_n10` | Proof: postconditions hold for N=10 | PASS | PASS ✅ |
| 9 | `test_set_value_basic` | `set_value` sets value and preserves `array_wf()` | PASS | PASS ✅ |
| 10 | `test_set_next_basic` | `set_next` sets next and preserves `array_wf()` | PASS | PASS ✅ |
| 11 | `test_set_prev_basic` | `set_prev` sets prev and preserves `array_wf()` | PASS | PASS ✅ |
| 12 | `test_set_value_preserves_links` | `set_value` preserves next/prev at same index | PASS | PASS ✅ |
| 13 | `test_set_value_preserves_metadata` | `set_value` preserves ghost metadata after init | PASS | PASS ✅ |

---

## Completeness Results

### Round 1: Precondition Violations — 5/5 FAIL ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_init_n_zero` | N=0 violates `N > 2` | FAIL | FAIL ✅ |
| 2 | `test_init_n_one` | N=1 violates `N > 2` | FAIL | FAIL ✅ |
| 3 | `test_init_n_two` | N=2 (boundary) violates `N > 2` | FAIL | FAIL ✅ |
| 4 | `test_init_no_array_wf` | Missing `array_wf()` precondition | FAIL | FAIL ✅ |
| 5 | `test_set_value_no_array_wf` | `set_value` without `array_wf()` | FAIL | FAIL ✅ |

### Round 2: Overly Strong Postconditions — 5/5 FAIL ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_init_len_positive` | Assert `len() > 0` after init | FAIL | FAIL ✅ |
| 2 | `test_init_view_len_one` | Assert `view.len() == 1` after init | FAIL | FAIL ✅ |
| 3 | `test_init_view_len_n` | Assert `view.len() == N` after init | FAIL | FAIL ✅ |
| 4 | `test_init_free_list_len_exceeds_n` | Assert `free_list_len > N` (impossible) | FAIL | FAIL ✅ |
| 5 | `test_init_value_list_len_positive` | Assert `value_list_len > 0` after init | FAIL | FAIL ✅ |

### Round 3: Negated/Contradicted Postconditions — 5/5 FAIL ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_init_not_wf` | Assert `!wf()` after init | FAIL | FAIL ✅ |
| 2 | `test_init_view_not_empty` | Assert view ≠ empty after init | FAIL | FAIL ✅ |
| 3 | `test_init_len_not_zero` | Assert `len() != 0` after init | FAIL | FAIL ✅ |
| 4 | `test_init_spec_len_not_zero` | Assert `spec_len() != 0` after init | FAIL | FAIL ✅ |
| 5 | `test_init_view_len_positive` | Assert `view.len() > 0` after init | FAIL | FAIL ✅ |

### Round 4: Wrong Specific Values — 5/5 FAIL ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_init_len_equals_n` | Assert `len() == 5` after init (wrong) | FAIL | FAIL ✅ |
| 2 | `test_init_len_equals_one` | Assert `len() == 1` after init (wrong) | FAIL | FAIL ✅ |
| 3 | `test_init_spec_len_wrong` | Assert `spec_len() == 3` after init (wrong) | FAIL | FAIL ✅ |
| 4 | `test_set_value_wrong_value` | Assert value is Some(99) after setting Some(42) | FAIL | FAIL ✅ |
| 5 | `test_set_next_wrong_value` | Assert next is 5 after setting 3 | FAIL | FAIL ✅ |

### Round 5: Cross-function Misuse & Edge Cases — 5/5 FAIL ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_set_value_changes_next` | Assert `set_value` changes next (frame violation) | FAIL | FAIL ✅ |
| 2 | `test_set_next_changes_value` | Assert `set_next` changes value (frame violation) | FAIL | FAIL ✅ |
| 3 | `test_set_prev_changes_next` | Assert `set_prev` changes next (frame violation) | FAIL | FAIL ✅ |
| 4 | `test_set_value_changes_other_index` | Assert `set_value(0)` changes node 1 (frame violation) | FAIL | FAIL ✅ |
| 5 | `test_len_without_wf_not_view_len` | Assert `value_list_len == view.len()` without wf | FAIL | FAIL ✅ |

---

## Overall Assessment

- **Correctness**: ✅ All 13 correctness tests pass. The specs for `init`, `set_value`, `set_next`, `set_prev`, and `len()` are correct — valid usages produce expected results.
- **Completeness**: ✅ All 25 completeness tests fail as expected. The specs properly reject:
  - Precondition violations (invalid N values, missing `array_wf()`)
  - Overly strong claims (non-empty after init, bounds exceeding N)
  - Contradicted postconditions (negated wf, negated emptiness)
  - Wrong concrete values (incorrect lengths, wrong field values)
  - Frame condition violations (cross-field/cross-index mutations)
- **Spec Gaps**: None found. The specifications are both correct and complete for the tested properties.
