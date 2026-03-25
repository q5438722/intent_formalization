# Test Summary: `delegation_map_v__impl3__find_key`

## Target Specification

```rust
fn find_key(&self, k: &K) -> (o: Option<usize>)
    requires self.valid(),
    ensures
        match o {
            None => !self@.contains_key(*k),
            Some(i) => 0 <= i < self.keys@.len() && self.keys@[i as int] == k,
        },
```

## Results

All 9 adversarial tests **FAILED verification** as expected, indicating the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (3/3 FAIL ✓)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_domain_mismatch` | Key in keys but domain ≠ keys.to_set() → can't prove containment | FAIL ✓ |
| `test_boundary_empty_keys` | Empty valid map → can't prove any key exists | FAIL ✓ |
| `test_boundary_oob_index` | Index == keys.len() (off-by-one) → postcondition rejected | FAIL ✓ |

### Behavioral Mutation Tests (3/3 FAIL ✓)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_behavioral_some_but_not_in_map` | Some(i) returned → assert key NOT in map (negate membership) | FAIL ✓ |
| `test_behavioral_none_but_in_map` | None returned → assert key IS in map (negate absence) | FAIL ✓ |
| `test_behavioral_wrong_key` | Some(i) returned → assert keys[i] ≠ k (negate key equality) | FAIL ✓ |

### Logical Tests (3/3 FAIL ✓)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_none_implies_empty` | None → map is empty (over-generalization) | FAIL ✓ |
| `test_logical_always_first` | Some(i) → i == 0 (over-constrained position) | FAIL ✓ |
| `test_logical_adjacent_indices` | Two keys → adjacent indices (over-constrained relation) | FAIL ✓ |

## Conclusion

The specification for `find_key` is **consistent** with respect to all tested properties:

- **Boundary correctness**: The `valid()` precondition properly constrains the state space. Without `map_valid()`, the connection between keys and the ghost map breaks down. Out-of-bounds indices are correctly rejected.
- **Behavioral correctness**: The postcondition correctly distinguishes `None` (key absent) from `Some(i)` (key found at index i). Mutating either direction is rejected by the solver.
- **Logical correctness**: The spec does not over-promise — it does not imply the map is empty when a key is absent, does not fix the index position, and does not constrain relative positions of different keys.

No spec weaknesses were detected.
