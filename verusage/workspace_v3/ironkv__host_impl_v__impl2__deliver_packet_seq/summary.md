# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__deliver_packet_seq/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ios_length_mismatch → `deliver_packet_seq`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If ios length differs from packets length, deliver_packet_seq's bijection between sent packets and io events is broken

### φ2: empty_packets_nonempty_ios → `deliver_packet_seq`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Mapping an empty packet sequence should yield empty ios; non-empty output from nothing would fabricate send events

### φ3: mapped_ios_not_all_sends → `deliver_packet_seq`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** map_sent_packet_seq_to_ios wraps every packet in Send; if all_ios_are_sends were false, the ensures of deliver_packet_seq would be inconsistent

### φ4: invalid_send_passes_check → `no_invalid_sends`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** no_invalid_sends must reject any Send carrying InvalidMessage; passing here means the guard is vacuously true or mis-specified

### φ5: singleton_packet_set_empty → `deliver_packet_seq`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Abstractifying a singleton packet sequence to an empty set would silently discard outbound packets, violating deliver_packet_seq's completeness guarantee

