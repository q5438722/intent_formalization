# Adversarial Proof Test Summary

**Target**: `net_sht_v__send_packet-poly.rs` (IronKV `send_packet` function)

## Results Overview

| Category | Tests | All Failed? | Spec Weakness Found? |
|---|---|---|---|
| Boundary Tests | 4 | ✅ Yes (4/4 failed) | No |
| Behavioral Mutation Tests | 4 | ✅ Yes (4/4 failed) | No |
| Logical Tests | 4 | ✅ Yes (4/4 failed) | No |

**Total: 12/12 tests correctly rejected by the specification.**

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_boundary_endpoint_at_exact_limit` | `id.len() == 0x100000` violates strict `<` in `valid_physical_address` | FAILED ✅ |
| 2 | `test_boundary_oversized_packet_data` | Data > `u64::MAX` violates `net_packet_bound` | FAILED ✅ |
| 3 | `test_boundary_timeout_is_not_send` | `TimeoutReceive` is not `Send` | FAILED ✅ |
| 4 | `test_boundary_receive_is_not_send` | `Receive` is not `Send` | FAILED ✅ |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Mutation Applied | Result |
|---|---|---|---|
| 1 | `test_mutation_abstractify_dst_swapped` | Assert `dst == src` (swapped) | FAILED ✅ |
| 2 | `test_mutation_abstractify_src_swapped` | Assert `src == dst` (swapped) | FAILED ✅ |
| 3 | `test_mutation_different_endpoints_equal` | Assert two distinct endpoints are equal | FAILED ✅ |
| 4 | `test_mutation_abstractify_msg_wrong_variant` | Assert msg is hardcoded `Ack{0}` | FAILED ✅ |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property Queried | Result |
|---|---|---|---|
| 1 | `test_logical_abstractable_no_address_guarantee` | `net_packet_is_abstractable` ⇒ `valid_physical_address` | FAILED ✅ |
| 2 | `test_logical_demarshal_not_always_ack` | `sht_demarshal_data` always returns `Ack` | FAILED ✅ |
| 3 | `test_logical_same_msg_different_dst_not_equal` | Same msg ⇒ equal packets (ignoring dst) | FAILED ✅ |
| 4 | `test_logical_abstractify_msg_not_always_invalid` | `abstractify` msg always `InvalidMessage` | FAILED ✅ |

---

## Conclusion

The specification of `send_packet` and its supporting functions is **consistent** with respect to all 12 adversarial queries:

- **Boundary integrity**: Preconditions correctly reject invalid inputs at edge cases (exact limits, wrong event types).
- **Behavioral correctness**: The spec distinguishes correct from incorrect output relationships (dst/src preservation, structural equality, message variant).
- **Logical soundness**: The spec does not entail unintended properties (trivial abstractability does not imply address validity, `choose`-based deserialization is not over-constrained, structural distinctness is preserved through abstractification).

No specification weaknesses were identified.
