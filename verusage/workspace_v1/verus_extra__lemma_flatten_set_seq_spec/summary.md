# Adversarial Proof Test Summary: `lemma_flatten_set_seq_spec`

## Target Specification

The lemma `lemma_flatten_set_seq_spec` proves a complete characterization of `flatten_set_seq`, which folds a `Seq<Set<A>>` into a single `Set<A>` via iterated union:

- **Forward**: `x ∈ flatten(sets) ⟹ ∃i. 0 ≤ i < |sets| ∧ x ∈ sets[i]`
- **Backward**: `0 ≤ i < |sets| ∧ x ∈ sets[i] ⟹ x ∈ flatten(sets)`

The lemma has **no preconditions** (`requires` clause is absent).

---

## Results Overview

| Category               | Tests | All Failed (as expected) |
|------------------------|-------|--------------------------|
| Boundary Tests         | 3     | ✅ Yes                   |
| Behavioral Mutation    | 3     | ✅ Yes                   |
| Logical Tests          | 3     | ✅ Yes                   |
| **Total**              | **9** | **✅ 9/9 rejected**      |

---

## Boundary Tests (`boundary_tests.rs`)

All 3 tests **FAILED verification** ✅ (correctly rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_empty_seq_contains_element` | Empty sequence's flatten contains an element | ✅ Rejected |
| 2 | `test_boundary_all_empty_sets_contains_element` | Sequence of empty sets' flatten contains an element | ✅ Rejected |
| 3 | `test_boundary_negative_index_access` | Negative index access on sequence (`sets[-1]`) | ✅ Rejected (Seq::index precondition + assertion) |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All 3 tests **FAILED verification** ✅ (correctly rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_negate_backward` | Element in `sets[0]` is NOT in flatten (negate backward direction) | ✅ Rejected |
| 2 | `test_mutation_wrong_index` | Element 1 in `sets[0]` is claimed to be in `sets[1]` (wrong witness) | ✅ Rejected |
| 3 | `test_mutation_phantom_element` | Element 99 (not in any set) is in flatten (phantom membership) | ✅ Rejected |

## Logical Tests (`logical_tests.rs`)

All 3 tests **FAILED verification** ✅ (correctly rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logical_uniqueness_of_membership` | Element cannot appear in two different sets (false uniqueness) | ✅ Rejected |
| 2 | `test_logical_order_dependence` | Reversing sequence order loses elements (false order dependence) | ✅ Rejected |
| 3 | `test_logical_cross_sequence_misuse` | Lemma called on `[s1,s2]` used to conclude about `flatten([s1])` containing 2 | ✅ Rejected |

---

## Conclusion

The specification for `lemma_flatten_set_seq_spec` is **robust** across all three adversarial dimensions:

1. **Boundary**: Edge cases (empty sequences, empty sets, invalid indices) are correctly handled.
2. **Behavioral**: Incorrect membership claims (negated direction, wrong index, phantom elements) are all rejected.
3. **Logical**: Unintended reasoning (false uniqueness, false order sensitivity, cross-sequence misuse) is not entailed.

The bidirectional ensures clause (forward + backward) provides a **complete** characterization of `flatten_set_seq` membership, leaving no gap for adversarial queries to exploit.
