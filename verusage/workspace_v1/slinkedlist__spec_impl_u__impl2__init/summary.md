# Test Execution Summary: `slinkedlist__spec_impl_u__impl2__init`

## Target
`StaticLinkedList<T, N>::init()` — initializes a static (array-backed) doubly-linked list, setting up all N slots as free list nodes and leaving the value list empty.

## Specification Under Test
- **Requires**: `N > 2`, `N < SLLIndex::MAX`, `old(self).array_wf()`
- **Ensures**: `self.wf()`, `self@ =~= Seq::empty()`, `self.len() == 0`

All `closed spec fn` predicates were changed to `open spec fn` in test files to enable meaningful semantic testing.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 errors) |

**Total: 15/15 tests correctly rejected by the verifier.**

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Failure Mode | Verus Error |
|---|------|-------------|-------------|
| 1 | `test_boundary_init_n_eq_2` | N=2 violates `N > 2` | precondition not satisfied |
| 2 | `test_boundary_init_n_eq_1` | N=1 violates `N > 2` | precondition not satisfied |
| 3 | `test_boundary_init_no_array_wf` | Missing `array_wf()` | precondition not satisfied |
| 4 | `test_boundary_wf_n_eq_2` | `wf()` unsatisfiable for N=2 | assertion failed |
| 5 | `test_boundary_array_wf_wrong_size` | `size != N` breaks `array_wf()` | assertion failed |

**Conclusion**: The spec correctly rejects all invalid inputs and boundary violations.

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation | Verus Error |
|---|------|----------|-------------|
| 1 | `test_mutation_init_nonempty_view` | Assert `sll@.len() > 0` (should be 0) | assertion failed |
| 2 | `test_mutation_init_nonzero_value_list_len` | Assert `value_list_len == 1` (should be 0) | assertion failed |
| 3 | `test_mutation_init_value_head_wrong` | Assert `value_list_head != -1` (should be -1) | assertion failed |
| 4 | `test_mutation_init_free_list_len_wrong` | Assert `free_list_len != 5` (should be N=5) | assertion failed |
| 5 | `test_mutation_init_free_head_minus_one` | Assert `free_list_head == -1` (should be valid index) | assertion failed |

**Conclusion**: The spec correctly rejects all mutated/incorrect post-init behaviors.

## Logical Tests (5/5 FAILED ✅)

| # | Test | Property Tested | Verus Error |
|---|------|----------------|-------------|
| 1 | `test_logical_determinism` | Free list order not uniquely determined | assertion failed |
| 2 | `test_logical_all_nodes_have_values` | Not all nodes have `Some` values | assertion failed |
| 3 | `test_logical_wf_does_not_imply_nonempty` | `wf()` doesn't force non-empty | assertion failed |
| 4 | `test_logical_wf_does_not_imply_full` | `wf()` doesn't force `value_list_len == N` | assertion failed |
| 5 | `test_logical_free_head_not_fixed` | Free list head not pinned to index 0 | assertion failed |

**Conclusion**: The spec correctly avoids over-constraining — it does not entail unintended properties like deterministic free list ordering, mandatory non-emptiness, or universal `Some` values.

---

## Overall Assessment

The specification for `StaticLinkedList::init()` is **well-formed and consistent** with respect to all 15 adversarial queries:

1. **Boundary integrity**: Invalid precondition violations (wrong N, missing array_wf) are properly rejected.
2. **Behavioral correctness**: Mutated postconditions (wrong lengths, heads, non-empty views) are properly rejected.
3. **Logical soundness**: The spec does not over-constrain the state — it correctly allows flexibility in free list ordering while enforcing structural invariants.

No spec weaknesses were detected. The `wf()` predicate and `init()` ensures clauses form a tight, consistent specification.
