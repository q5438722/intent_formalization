# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_receive_packet/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: fresh_packet_silently_dropped → `receive_packet`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, fresh messages are silently dropped even when the buffer is empty, violating reliable delivery

### φ2: new_message_implies_should_ack → `new_single_message`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** new_single_message (seqno == last+1) and should_ack (seqno <= last) must be mutually exclusive; overlap collapses sequencing

### φ3: receive_ack_always_noop → `receive_ack`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If ack processing never advances send-state, retransmission queues grow without bound and acked data is never freed

### φ4: ack_routed_wrong_direction → `send_ack`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Acks must go back to the sender (pkt.src); routing to pkt.dst means the ack never reaches the originator

### φ5: receive_never_emits_packets → `host_model_receive_packet`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If receive_packet never emits outbound packets, host_model_receive_packet can never send acks, breaking the reliable delivery protocol entirely

