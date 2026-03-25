# Adversarial Proof Test Summary

**Target**: `net_sht_v__receive_with_demarshal.rs`  
**Function under test**: `receive_with_demarshal` and its spec-level helpers  
**Result**: All 15 adversarial tests correctly **FAILED verification** — the specification rejects all tested invalid properties.

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

| Test | Property Queried | Result |
|------|-----------------|--------|
| B1: `test_boundary_ep_at_exact_limit` | `AbstractEndPoint` with `id.len() == 0x100000` is `valid_physical_address` | FAIL ✅ — spec correctly requires `< 0x100000` |
| B2: `test_boundary_ep_over_limit` | `AbstractEndPoint` with `id.len() == 0x200000` is `abstractable` | FAIL ✅ — rejected by `valid_physical_address` |
| B3: `test_boundary_demarshal_empty_data` | `sht_demarshal_data(empty)` result is `is_marshalable` | FAIL ✅ — recommends violated, no valid deserialization for empty data |
| B4: `test_boundary_abstractify_non_abstractable_ep` | Packet from `abstractify` with non-abstractable endpoints has abstractable dst | FAIL ✅ — non-abstractable endpoint propagates through |
| B5: `test_boundary_demarshal_invalid_tag` | `sht_demarshal_data(seq![42])` result is `is_marshalable` | FAIL ✅ — tag 42 has no valid CSingleMessage deserialization |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✅

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| M1: `test_mutation_swap_src_dst` | Assert `pkt.dst == ep2` (swapped with src) | FAIL ✅ — spec preserves dst correctly |
| M2: `test_mutation_wrong_src` | Assert `pkt.src == ep1` (wrong endpoint) | FAIL ✅ — spec preserves src correctly |
| M3: `test_mutation_dst_id_changed` | Assert `pkt.dst.id =~= seq![99u8]` (arbitrary id) | FAIL ✅ — spec preserves endpoint identity |
| M4: `test_mutation_different_packets_same_dst` | Assert two different packets produce same dst | FAIL ✅ — distinct endpoints produce distinct results |
| M5: `test_mutation_lsht_src_wrong` | Assert LSHT packet src has wrong id | FAIL ✅ — `abstractify_net_packet_to_lsht_packet` preserves src |

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✅

| Test | Unintended Property Queried | Result |
|------|----------------------------|--------|
| L1: `test_logical_abstractable_never_false` | `net_packet_is_abstractable` returns false | FAIL ✅ — spec always returns true (Note: this is trivially true, a potential weakness) |
| L2: `test_logical_demarshal_different_data_equal` | `sht_demarshal_data(d1) == sht_demarshal_data(d2)` for distinct data | FAIL ✅ — spec does not conflate different inputs |
| L3: `test_logical_demarshal_never_invalid` | `sht_demarshal_data(data)` is never `InvalidMessage` for all data | FAIL ✅ — spec does not guarantee valid deserialization |
| L4: `test_logical_abstractify_not_injective` | `abstractify_net_packet_to_sht_packet` is injective | FAIL ✅ — injectivity not guaranteed (msg undergoes lossy transformation) |
| L5: `test_logical_no_roundtrip` | `sht_demarshal_data(data).ghost_serialize() == data` for all data | FAIL ✅ — roundtrip not guaranteed for arbitrary data |

---

## Observations

1. **Spec is well-bounded**: All boundary tests fail, showing preconditions and `recommends` clauses correctly guard against invalid inputs.
2. **Spec preserves field identity**: All behavioral mutations on `abstractify_*` functions are correctly rejected — the spec faithfully propagates `dst`, `src`, and `msg` fields.
3. **Spec avoids over-commitment**: Logical tests confirm the spec does not entail unintended properties (injectivity, universal validity, roundtrip guarantees).
4. **Potential weakness**: `net_packet_is_abstractable` is trivially `true` for all inputs (L1). This means the `recommends` clause on `abstractify_net_packet_to_sht_packet` provides no real guard. If the intent was to restrict certain packets, this is too weak. However, the comment in the source (`// NetPacketIsAbstractable is true`) suggests this is intentional.
