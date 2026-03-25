# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_model_v__impl2__send_single_cmessage/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: payload_differs_from_input → `send_single_cmessage`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the spec allows a successfully sent message to carry a different payload than the one requested

### φ2: seqno_zero_on_success → `send_single_cmessage`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the spec permits a zero sequence number on a successful send, but new_seqno must be >= 1

### φ3: un_acked_no_growth → `send_single_message`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, a successful send does not grow the un_acked list, contradicting the push in the spec

### φ4: missing_key_nonempty_unacked → `ack_state_lookup`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, looking up a missing endpoint yields a non-empty un_acked list instead of the expected empty default

### φ5: some_value_always_valid → `valid_optional_value`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, every abstract value is considered valid, meaning valid_optional_value imposes no constraint on Some values

