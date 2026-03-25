# Adversarial Test Summary: `un_acked_messages_extend`

## Target
`single_delivery_state_v__impl3__un_acked_messages_extend.rs` — proves that `un_acked_messages_for_dest_up_to(src, dst, i+1)` equals `un_acked_messages_for_dest_up_to(src, dst, i).insert(packet)` where `packet` wraps the `i`-th un-acked message.

## Results

All **9 adversarial tests** correctly **FAILED** verification, confirming the specification is consistent and sufficiently strong.

### Boundary Tests (3/3 FAILED ✅)

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| `test_boundary_dst_not_in_send_state` | `self@.send_state.contains_key(dst)` | FAILED — precondition not satisfied |
| `test_boundary_index_out_of_bounds` | `i < self@.send_state[dst].un_acked.len()` | FAILED — precondition not satisfied |
| `test_boundary_send_state_not_valid` | `self.send_state.valid()` | FAILED — precondition not satisfied |

### Behavioral Mutation Tests (3/3 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_negate_equality` | Assert `set(i+1) !== set(i).insert(packet)` | FAILED — assertion failed |
| `test_mutation_no_insert` | Assert `set(i+1) =~= set(i)` (no insert) | FAILED — assertion failed |
| `test_mutation_wrong_src` | Assert `set(i+1) =~= set(i).insert(wrong_src_packet)` | FAILED — assertion failed |

### Logical Tests (3/3 FAILED ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_count_zero_nonempty` | Set with count=0 is non-empty | FAILED — assertion failed |
| `test_logical_different_src_same_set` | Different `src` yields same set | FAILED — assertion failed |
| `test_logical_extend_shrinks` | Inserted packet not in extended set | FAILED — assertion failed |

## Conclusion

The specification for `un_acked_messages_extend` is **consistent**:
- **Preconditions** correctly reject invalid inputs (missing dst, out-of-bounds index, invalid send state).
- **Postcondition** correctly rejects mutated behaviors (negation, omission, wrong source).
- **Logical properties** are properly bounded (empty set at count 0, src-sensitivity, monotonic growth).

No spec weaknesses were found.
