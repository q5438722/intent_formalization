# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__make_send_only_event_results/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: sends_empty_despite_nonempty_input → `make_send_only_event_results`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-empty all-Send input producing empty sends would mean the spec loses track of outgoing events

### φ2: empty_input_nonempty_event_seq → `make_send_only_event_results`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An empty input producing a non-empty event sequence would fabricate events out of nothing

### φ3: sends_contain_receive_event → `make_send_only_event_results`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Having a Receive inside sends would violate well_typed_events and corrupt the event classification invariant

### φ4: event_seq_longer_than_sends → `make_send_only_event_results`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** event_seq being longer than sends (with empty recvs/clocks) would mean concatenation conjured phantom events

### φ5: ios_differs_from_event_seq → `make_send_only_event_results`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** ios diverging from event_seq would break the refinement contract between intended and observed events

