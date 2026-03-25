# Adversarial Proof Test Summary: `send_packet_seq`

## Target Specification

```rust
pub fn send_packet_seq(cpackets: &Vec<CPacket>, netc: &mut NetClient) -> (rc: (bool, Ghost<Seq<NetEvent>>))
requires
    old(netc).ok(),
    outbound_packet_seq_is_valid(cpackets@),
    outbound_packet_seq_has_correct_srcs(cpackets@, old(netc).my_end_point()),
ensures
    netc.my_end_point() == old(netc).my_end_point(),
    ({
        let (ok, Ghost(net_events)) = rc;
        {
            &&& netc.ok() <==> ok
            &&& ok ==> netc.history() == old(netc).history() + net_events
            &&& ok ==> send_log_entries_reflect_packets(net_events, cpackets@)
            &&& ok ==> only_sent_marshalable_data(net_events)
            &&& forall |i| 0 <= i < net_events.len() ==> net_events[i] is Send
        }
    })
```

This function iterates over a sequence of `CPacket`s, sends each via `send_packet`, and returns the combined net events. The specification guarantees: endpoint preservation, history extension, event-packet correspondence, marshalable data invariant, and all events being Sends.

---

## Test Results Overview

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 | 0 |
| Behavioral Mutation | 5 | 5 | 0 |
| Logical | 5 | 5 | 0 |
| **Total** | **15** | **15** | **0** |

**Verdict**: The specification is **consistent** — all 15 adversarial tests were correctly rejected.

---

## Boundary Tests (`boundary_tests.rs`)

Tests that violate preconditions to check if invalid inputs are rejected.

| Test | Violation | Result |
|---|---|---|
| B1: `test_boundary_invalid_message_accepted` | InvalidMessage packet satisfies `outbound_packet_is_valid` | ✅ FAILED |
| B2: `test_boundary_non_abstractable_dst` | Packet with oversized dst id satisfies `outbound_packet_is_valid` | ✅ FAILED |
| B3: `test_boundary_seq_with_invalid_packet` | Seq containing an InvalidMessage satisfies `outbound_packet_seq_is_valid` | ✅ FAILED |
| B4: `test_boundary_wrong_src` | Mismatched source satisfies `outbound_packet_seq_has_correct_srcs` | ✅ FAILED |
| B5: `test_boundary_non_send_event_reflects` | Receive event satisfies `send_log_entry_reflects_packet` | ✅ FAILED |

**Analysis**: All precondition specs properly reject invalid inputs. `outbound_packet_is_valid` correctly requires abstractability, marshalability, and non-InvalidMessage. The sequence-level predicates correctly propagate to individual elements.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

Tests that start from valid inputs but mutate expected output relations.

| Test | Mutation | Result |
|---|---|---|
| M1: `test_mutation_negate_length_equality` | Assert `events.len() != cpackets.len()` after `send_log_entries_reflect_packets` | ✅ FAILED |
| M2: `test_mutation_negate_event_is_send` | Assert event is NOT Send after `send_log_entry_reflects_packet` | ✅ FAILED |
| M3: `test_mutation_valid_packet_is_invalid` | Assert packet IS InvalidMessage after `outbound_packet_is_valid` | ✅ FAILED |
| M4: `test_mutation_reflected_packet_not_abstractable` | Assert packet is NOT abstractable after reflection holds | ✅ FAILED |
| M5: `test_mutation_marshalable_data_negated` | Assert Send event is NOT marshalable after `only_sent_marshalable_data` | ✅ FAILED |

**Analysis**: The specification correctly rejects all mutated behavioral claims. The length equality, Send-type constraint, InvalidMessage exclusion, abstractability, and marshalability invariants are all properly enforced and cannot be negated.

---

## Logical Tests (`logical_tests.rs`)

Tests for properties NOT explicitly guaranteed by the specification.

| Test | Property Tested | Result |
|---|---|---|
| L1: `test_logical_demarshal_always_marshalable` | `sht_demarshal_data(data)` is always marshalable | ✅ FAILED |
| L2: `test_logical_abstractify_injective` | `abstractify_net_packet_to_sht_packet` is injective | ✅ FAILED |
| L3: `test_logical_valid_implies_bounded_serialization` | Valid packet implies serialization length ≤ 1024 | ✅ FAILED |
| L4: `test_logical_empty_seq_is_nonempty` | Empty valid sequence implies non-empty | ✅ FAILED |
| L5: `test_logical_all_events_same_dst` | Reflected packets all have the same destination | ✅ FAILED |

**Analysis**: The specification correctly does NOT entail any of these overly-strong properties:
- **L1**: `sht_demarshal_data` uses `choose` and gives no guarantee when no valid pre-image exists — marshalability is not universal.
- **L2**: `abstractify_net_packet_to_sht_packet` maps through `sht_demarshal_data` which can collapse distinct data; injectivity is not guaranteed.
- **L3**: No upper bound on serialization length is imposed by `outbound_packet_is_valid`.
- **L4**: `outbound_packet_seq_is_valid` is vacuously true for empty sequences; it does not imply non-emptiness.
- **L5**: `send_log_entries_reflect_packets` handles each packet independently; packets may have different destinations.

---

## Conclusion

The `send_packet_seq` specification is **well-bounded**:
1. **Preconditions are enforced** — invalid inputs (InvalidMessage, non-abstractable endpoints, wrong sources, non-Send events) are correctly rejected.
2. **Behavioral correctness is tight** — mutated output relations (negated lengths, types, marshalability) are all rejected.
3. **No unintended entailments** — the spec does not admit overly-strong logical consequences (universal marshalability, injectivity, serialization bounds, non-emptiness, uniform destinations).

No specification weaknesses were discovered.
