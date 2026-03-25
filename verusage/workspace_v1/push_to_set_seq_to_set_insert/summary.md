# Test Execution Summary: `push_to_set_seq_to_set_insert`

## Specification Under Test

```rust
pub proof fn push_to_set_eq_to_set_insert<A>(s: Seq<A>, e: A)
    ensures s.push(e).to_set() == s.to_set().insert(e)
```

No preconditions. Postcondition: pushing an element onto a sequence and converting to a set equals converting the original sequence to a set and inserting the element.

---

## Results Overview

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary Tests | 5 | ✅ Yes |
| Behavioral Mutation Tests | 5 | ✅ Yes |
| Logical Tests | 5 | ✅ Yes |
| **Total** | **15** | **✅ 15/15** |

All 15 adversarial tests were **correctly rejected** by Verus, indicating the specification is strong enough to distinguish correct from incorrect properties across all three query categories.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_boundary_1_push_empty_gives_empty_set` | `empty().push(0).to_set() =~= Set::empty()` | ❌ Rejected |
| 2 | `test_boundary_2_push_preserves_length` | `s.push(e).len() == s.len()` | ❌ Rejected |
| 3 | `test_boundary_3_push_empty_seq_has_no_elements` | `!s.push(e).contains(e)` | ❌ Rejected |
| 4 | `test_boundary_4_push_zero_set_len_zero` | `s.push(e).to_set().len() == 0` (after push on empty) | ❌ Rejected |
| 5 | `test_boundary_5_push_gives_wrong_singleton` | `empty().push(1).to_set() =~= {2}` | ❌ Rejected |

**Conclusion**: The spec correctly constrains edge-case behavior. Even with empty sequences and zero values, invalid set equalities are rejected.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Mutation Applied | Result |
|---|---|---|---|
| 1 | `test_mutation_1_push_set_eq_original_set` | Omit `insert(e)` from RHS | ❌ Rejected |
| 2 | `test_mutation_2_push_set_eq_remove` | Replace `insert(e)` with `remove(e)` | ❌ Rejected |
| 3 | `test_mutation_3_pushed_element_not_in_set` | Negate containment of pushed element | ❌ Rejected |
| 4 | `test_mutation_4_insert_wrong_element` | Insert wrong element (99 instead of 5) | ❌ Rejected |
| 5 | `test_mutation_5_negate_ensures` | Negate the entire ensures clause | ❌ Rejected |

**Conclusion**: The spec is precise enough to reject all behavioral mutations — omitting the insert, replacing it with remove, inserting the wrong element, or negating the postcondition.

---

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property Queried | Result |
|---|---|---|---|
| 1 | `test_logical_1_set_size_always_increases` | `\|s.push(e).to_set()\| == \|s.to_set()\| + 1` (false when e ∈ s) | ❌ Rejected |
| 2 | `test_logical_2_push_reversible_via_remove` | `s.push(e).to_set().remove(e) == s.to_set()` (false when e ∈ s) | ❌ Rejected |
| 3 | `test_logical_3_seq_len_eq_set_len` | `\|s.push(e).to_set()\| == \|s.push(e)\|` (false with duplicates) | ❌ Rejected |
| 4 | `test_logical_4_different_pushes_different_cardinality` | Different pushes always give different cardinalities | ❌ Rejected |
| 5 | `test_logical_5_to_set_injective` | `to_set` is injective over sequences | ❌ Rejected |

**Conclusion**: The spec does not entail stronger-than-intended properties. It correctly avoids implying set cardinality always increases, that push is reversible, or that `to_set` is injective.

---

## Overall Assessment

**The specification is consistent.** It:
- Rejects all invalid edge-case inputs (boundary)
- Rejects all mutated output relations (behavioral)
- Does not entail stronger logical properties than stated (logical)

No specification weaknesses were detected across the 15 adversarial queries.
