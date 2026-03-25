# Adversarial Proof Test Results

## Target
`single_delivery_model_v__impl2__receive_impl.rs` — `CSingleDelivery::receive_impl` and its spec-level predicates (`SingleDelivery::receive`, `new_single_message`, `receive_real_packet`, `should_ack_single_message`, `send_ack`, `maybe_ack_packet`).

---

## Summary

| Test Category         | File                        | Tests | All Fail? |
|-----------------------|-----------------------------|-------|-----------|
| Boundary              | `boundary_tests.rs`         | 5     | ✅ Yes     |
| Behavioral Mutation   | `behavioral_mutation_tests.rs` | 5  | ✅ Yes     |
| Logical               | `logical_tests.rs`          | 5     | ✅ Yes     |

**All 15 adversarial tests correctly FAIL verification**, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (5/5 FAIL ✅)

| ID | Test | What it checks | Result |
|----|------|----------------|--------|
| B1 | `test_boundary_invalid_msg_nonempty_acks` | InvalidMessage must produce empty acks | FAIL ✅ |
| B2 | `test_boundary_ack_produces_nonempty_acks` | Ack packets must produce empty ack set | FAIL ✅ |
| B3 | `test_boundary_wrong_seqno_is_new` | Wrong seqno (5 instead of 1) should not be new | FAIL ✅ |
| B4 | `test_boundary_future_seqno_should_ack` | Future seqno (> last) should not trigger ack | FAIL ✅ |
| B5 | `test_boundary_tombstone_absent_nonzero` | Absent key tombstone is 0, not > 0 | FAIL ✅ |

## Behavioral Mutation Tests (5/5 FAIL ✅)

| ID | Test | What it checks | Result |
|----|------|----------------|--------|
| M1 | `test_mutation_fresh_packet_no_state_change` | Fresh packet must update receive_state | FAIL ✅ |
| M2 | `test_mutation_ack_wrong_direction` | Ack dst must equal pkt.src (not pkt.dst) | FAIL ✅ |
| M3 | `test_mutation_ack_wrong_seqno` | Ack seqno must match packet seqno | FAIL ✅ |
| M4 | `test_mutation_wrong_tombstone_value` | Tombstone must be last_seqno+1, not +2 | FAIL ✅ |
| M5 | `test_mutation_should_ack_but_empty_acks` | should_ack must produce non-empty acks | FAIL ✅ |

## Logical Tests (5/5 FAIL ✅)

| ID | Test | What it checks | Result |
|----|------|----------------|--------|
| L1 | `test_logical_insert_affects_other_keys` | Map insert only affects the specified key | FAIL ✅ |
| L2 | `test_logical_always_new_message` | new_single_message is not universally true | FAIL ✅ |
| L3 | `test_logical_ack_is_message_type` | Ack msg is Ack variant, not Message variant | FAIL ✅ |
| L4 | `test_logical_tombstone_jumps_by_two` | Tombstone increments by 1, not ≥2 | FAIL ✅ |
| L5 | `test_logical_new_and_should_ack_both_true` | new_single_message and should_ack are mutually exclusive | FAIL ✅ |

---

## Conclusion

The specification for `receive_impl` is **consistent** with respect to the 15 adversarial properties tested:

- **Boundary completeness**: The spec correctly constrains edge cases (wrong seqno, absent tombstone, invalid message types).
- **Behavioral correctness**: Mutated outputs (wrong direction, wrong seqno, wrong tombstone value) are all rejected.
- **Logical soundness**: The spec does not entail unintended properties (universal newness, wrong ack type, tombstone over-increment, cross-key interference, mutual exclusivity violation).

### Implementation Note
Tests use field-level reasoning (direct `let`-bindings and concrete endpoint values) rather than `assume(x == generic_struct{...})` patterns to avoid Z3 SMT solver issues with generic struct equality and map axiom instantiation under `assume`.
