# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__net_sht_v__send_packet_seq/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: success_empty_events → `send_packet_seq`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Successful send of non-empty packets should produce at least one net event, not zero

### φ2: events_exceed_packets → `send_packet_seq`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Each packet should produce exactly one send event; more events than packets indicates a spec gap

### φ3: success_history_unchanged → `send_packet_seq`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Sending non-empty packets must grow the history; unchanged history means events were silently lost

### φ4: single_packet_multiple_events → `send_packet_seq`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single outbound packet should yield exactly one send event; more than one indicates event duplication

### φ5: to_vec_always_empty → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A map-to-vec conversion should not always yield an empty vector regardless of map contents

