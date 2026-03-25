# Test Execution Summary: `remove_helper5`

## Target
`slinkedlist__spec_impl_u__impl2__remove_helper5.rs` — removes the head element from a static linked list, appending the freed node to the free list.

## Results: All 15 adversarial tests correctly FAILED ✅

The specification properly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (5/5 failed) — `boundary_tests.rs`

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| B1: `value_list_len == 1` | `value_list_len != 1` | ❌ FAILED (correct) |
| B2: `free_list_len == 0` | `free_list_len != 0` | ❌ FAILED (correct) |
| B3: `value_list_head != remove_index` | `value_list_head == remove_index` | ❌ FAILED (correct) |
| B4: value not in list | `@.contains(v@)` | ❌ FAILED (correct) |
| B5: not well-formed | `wf()` | ❌ FAILED (correct) |

Note: `remove_helper5` itself verified successfully (1 verified, 5 errors).

---

### Behavioral Mutation Tests (5/5 failed) — `behavioral_mutation_tests.rs`

| Test | Mutated Property | Result |
|------|-----------------|--------|
| M1: length unchanged | `new.len() == old.len()` (should be `old.len() - 1`) | ❌ FAILED (correct) |
| M2: wrong return value | `ret != v` (should be `ret == v`) | ❌ FAILED (correct) |
| M3: removed value present | `new@.contains(ret)` (should not contain) | ❌ FAILED (correct) |
| M4: sequence unchanged | `new@ =~= old@` (should be `old@.remove_value(ret)`) | ❌ FAILED (correct) |
| M5: length increases | `new.len() > old.len()` (length decreases) | ❌ FAILED (correct) |

---

### Logical Tests (5/5 failed) — `logical_tests.rs`

| Test | Unintended Property | Result |
|------|-------------------|--------|
| L1: all values preserved | `∀x. old@.contains(x) ⟹ new@.contains(x)` | ❌ FAILED (correct) |
| L2: result always empty | `new@.len() == 0` | ❌ FAILED (correct) |
| L3: node refs change | `old.get_node_ref(w) != new.get_node_ref(w)` for remaining `w` | ❌ FAILED (correct) |
| L4: first element constant | `new@[0] == 0u64` | ❌ FAILED (correct) |
| L5: internal state determinism | `new1.value_list_head == new2.value_list_head` from spec alone | ❌ FAILED (correct) |

---

## Conclusion

The specification of `remove_helper5` is **consistent** across all three test dimensions:

1. **Input validation**: All preconditions are necessary — violating any one causes verification failure.
2. **Behavioral correctness**: The postconditions precisely constrain the output — no mutated behavior is admitted.
3. **Logical soundness**: The spec does not entail unintended properties — it neither over-constrains results to constants nor admits universal element preservation or arbitrary structural assumptions.
