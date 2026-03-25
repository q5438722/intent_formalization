# Adversarial Proof Test Results: `array_set__impl0__init`

## Target Specification

`ArraySet::init(&mut self)` — Resets an `ArraySet<N>` to the empty set.

- **Requires**: `old(self).wf()`
- **Ensures**: `self.wf()`, `self@ == Set::<usize>::empty()`

## Results Summary

| File | Tests | All Failed (as expected) |
|------|-------|--------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 failed) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 failed) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 failed) |

**Total: 15/15 tests correctly rejected by the specification.**

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Property Queried | Result |
|---|------|------------------|--------|
| 1 | `test_boundary_init_set_contains_zero` | Empty set contains element 0 | FAILED ✅ |
| 2 | `test_boundary_init_set_len_positive` | Empty set has length > 0 | FAILED ✅ |
| 3 | `test_boundary_init_set_contains_max_index` | Empty set contains index N-1 | FAILED ✅ |
| 4 | `test_boundary_init_set_contains_out_of_range` | Empty set contains index N | FAILED ✅ |
| 5 | `test_boundary_wf_alone_implies_empty` | `wf()` alone ⇒ set is empty | FAILED ✅ |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation Applied | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_init_not_empty` | Negate emptiness postcondition | FAILED ✅ |
| 2 | `test_mutation_init_preserves_element` | Pre-state element survives init | FAILED ✅ |
| 3 | `test_mutation_init_produces_singleton` | Init produces {0} instead of ∅ | FAILED ✅ |
| 4 | `test_mutation_init_set_len_one` | Post-set length is 1 not 0 | FAILED ✅ |
| 5 | `test_mutation_init_is_identity` | Init acts as identity function | FAILED ✅ |

## Logical Tests (5/5 FAILED ✅)

| # | Test | Non-entailed Property | Result |
|---|------|----------------------|--------|
| 1 | `test_logical_wf_implies_empty` | `wf()` ⇒ empty set | FAILED ✅ |
| 2 | `test_logical_wf_implies_nonempty` | `wf()` ⇒ non-empty set | FAILED ✅ |
| 3 | `test_logical_two_wf_sets_equal` | Two wf sets are equal | FAILED ✅ |
| 4 | `test_logical_init_preserves_data` | Init preserves data array | FAILED ✅ |
| 5 | `test_logical_init_preserves_len` | Init preserves len field | FAILED ✅ |

---

## Analysis

The specification for `ArraySet::init` correctly rejects all 15 adversarial queries:

1. **Boundary robustness**: The empty-set postcondition correctly prevents any element membership claims and rejects the assumption that `wf()` alone implies emptiness.

2. **Behavioral precision**: Mutations to the postcondition (non-empty, preserved elements, wrong cardinality, identity behavior) are all rejected.

3. **Logical soundness**: The spec avoids over-constraining (`wf()` alone does not force emptiness or non-emptiness) and correctly rejects structural assumptions (init does not preserve data or len from the pre-state).

**Note**: During development, logical tests for `post.data@[0] == false` and `post.len == 0` initially *passed* verification because `closed spec` bodies are visible within the same module in Verus. This is correct Verus behavior — these properties ARE entailed when the spec is used in-module. The tests were replaced with genuinely non-entailed properties (preservation across init).

**Conclusion**: The `init` specification is consistent — it correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning within its semantic boundary.
