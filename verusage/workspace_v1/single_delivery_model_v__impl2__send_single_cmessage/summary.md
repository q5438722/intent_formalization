# Test Execution Summary: `send_single_cmessage`

**Target**: `single_delivery_model_v__impl2__send_single_cmessage.rs`
**Function under test**: `CSingleDelivery::send_single_cmessage` / `SingleDelivery::send_single_message`

---

## Results Overview

| Test File | Tests | All Failed (as expected) |
|---|---|---|
| `boundary_tests.rs` | 5 | ✅ 5/5 |
| `behavioral_mutation_tests.rs` | 5 | ✅ 5/5 |
| `logical_tests.rs` | 5 | ✅ 5/5 |

**Total: 15/15 tests correctly rejected by the specification.**

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Violation | Result |
|---|---|---|---|
| 1 | `test_boundary_invalid_dst_too_long` | dst.id.len() == 0x100000 (not < 0x100000) | ✅ FAILED |
| 2 | `test_boundary_seqno_overflow_returns_some` | Asserts `sm is Some` when seqno overflows | ✅ FAILED |
| 3 | `test_boundary_fresh_dst_seqno_zero` | Claims seqno=0 for fresh dst (actual=1) | ✅ FAILED |
| 4 | `test_boundary_almost_overflow_state_changes` | Claims state changes on overflow | ✅ FAILED |
| 5 | `test_boundary_zero_max_seqno` | Uses max_seqno=0, asserts successful send | ✅ FAILED |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_wrong_seqno` | seqno off by +1 (2 instead of 1) | ✅ FAILED |
| 2 | `test_mutation_wrong_dst` | Different dst in returned message | ✅ FAILED |
| 3 | `test_mutation_wrong_message_payload` | Wrong message payload (99 vs 42) | ✅ FAILED |
| 4 | `test_mutation_none_when_should_be_some` | None returned when room exists | ✅ FAILED |
| 5 | `test_mutation_state_unchanged_after_send` | send_state not updated after send | ✅ FAILED |

## Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_different_dst_same_message` | Same output for different destinations | ✅ FAILED |
| 2 | `test_logical_receive_state_changes` | receive_state mutated by send | ✅ FAILED |
| 3 | `test_logical_seqno_not_monotonic` | Sequential sends yield same seqno | ✅ FAILED |
| 4 | `test_logical_other_endpoint_modified` | Other endpoints' ack state corrupted | ✅ FAILED |
| 5 | `test_logical_result_is_ack` | Returned message is Ack not Message | ✅ FAILED |

---

## Conclusion

The specification for `send_single_message` correctly rejects all 15 adversarial queries:

- **Boundary**: Invalid inputs (oversized dst, seqno overflow, zero max_seqno) are properly constrained.
- **Behavioral**: Mutated outputs (wrong seqno, wrong dst, wrong payload, wrong result variant, missing state update) are all rejected.
- **Logical**: Unintended properties (cross-dst equality, receive_state mutation, seqno non-monotonicity, side-effects on other endpoints, wrong message variant) are not entailed.

**No specification weaknesses were detected.** The spec precisely constrains the seqno computation, message construction, state update semantics, and overflow behavior.
