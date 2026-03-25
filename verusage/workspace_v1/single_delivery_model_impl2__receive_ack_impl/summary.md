# Test Results Summary: `receive_ack_impl`

**Target**: `single_delivery_model_impl2__receive_ack_impl.rs`
**Function under test**: `CSingleDelivery::receive_ack_impl` (spec: `SingleDelivery::receive_ack`)

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes (3/3 failed) |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes (3/3 failed) |
| `logical_tests.rs` | 3 | ✅ Yes (3/3 failed) |

**Total: 9/9 tests correctly rejected by verifier.**

---

## Boundary Tests (Precondition Violations)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_nonempty_acks` | Non-empty acks set (spec requires `acks.is_empty()`) | ✅ FAILED |
| 2 | `test_boundary_invalid_message_type` | `pkt.msg is InvalidMessage` (violates `recommends pkt.msg is Ack`) | ✅ FAILED |
| 3 | `test_boundary_post_differs_at_equality_edge` | `ack_seqno == num_packets_acked` boundary: post differs from pre | ✅ FAILED |

**Assessment**: The spec correctly rejects all invalid inputs. The boundary between the update branch (`>`) and no-op branch (`<=`) is properly enforced.

---

## Behavioral Mutation Tests (Output Mutations)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_wrong_num_packets_acked` | Assert `num_packets_acked == ack_seqno + 1` (correct: `== ack_seqno`) | ✅ FAILED |
| 2 | `test_mutation_state_changed_when_noop` | Assert `pkt.src` removed from send_state in no-op case | ✅ FAILED |
| 3 | `test_mutation_acks_nonempty` | Assert acks is non-empty (spec guarantees empty) | ✅ FAILED |

**Assessment**: The spec correctly rejects all mutated outputs. Incorrect `num_packets_acked` values, unauthorized state modifications, and non-empty ack sets are all properly rejected.

---

## Logical Tests (Unintended Reasoning)

| # | Test | Property Tested | Result | Implication |
|---|------|----------------|--------|-------------|
| 1 | `test_logical_derive_false` | Soundness: can `false` be derived? | ✅ FAILED | Spec is sound (not vacuously true) |
| 2 | `test_logical_receive_state_preserved` | Is `receive_state` preserved in the update branch? | ✅ FAILED | **⚠️ Spec weakness detected** |
| 3 | `test_logical_stronger_inequality` | Is `num_packets_acked > ack_seqno`? | ✅ FAILED | Spec correctly uses equality, not strict inequality |

### ⚠️ Spec Weakness: `receive_state` Not Constrained (Test 2)

The spec for the update branch (`ack_seqno > num_packets_acked`) uses:
```
post =~= Self { send_state: pre.send_state.insert(pkt.src, new_ack_state), ..post }
```

The `..post` struct spread means `receive_state: post.receive_state`, making the constraint on `receive_state` tautological (`post.receive_state =~= post.receive_state`). The spec does **not** guarantee that `post.receive_state == pre.receive_state` in this branch.

The implementation preserves `receive_state` (it only modifies `send_state`), but this is not captured by the abstract specification. A stronger spec would use `..pre` instead of `..post` to constrain `receive_state` preservation:
```
post =~= Self { send_state: pre.send_state.insert(pkt.src, new_ack_state), ..pre }
```

Note: In the no-op branch (`ack_seqno <= num_packets_acked`), `post == pre` does correctly constrain all fields including `receive_state`.

---

## Conclusion

The specification is **mostly correct** — it properly rejects invalid inputs (boundary), incorrect outputs (behavioral mutation), vacuous reasoning, and overly strong claims (logical). However, one **spec incompleteness** was identified: `receive_state` is not constrained to be preserved in the update branch, which could in theory allow an implementation to silently corrupt the receive state while still satisfying the spec.
