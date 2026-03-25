# Adversarial Test Summary: `host_model_next_get_request`

## Target
`host_impl_v__impl2__host_model_next_get_request.rs` — handles incoming `GetRequest` messages in IronKV's host protocol. Core specs: `next_get_request` and `next_get_request_reply`.

## Results: All 15 tests FAILED verification ✅

All tests correctly fail, meaning the specification properly rejects every adversarial query.

### Boundary Tests (5/5 failed) — `boundary_tests.rs`

| Test | Property Queried | Result |
|------|-----------------|--------|
| B1 | `SetRequest` message passed to `next_get_request` | ✅ Rejected |
| B2 | `Reply` message passed to `next_get_request` | ✅ Rejected |
| B3 | `Ack` SingleMessage (non-Message variant) | ✅ Rejected |
| B4 | `InvalidMessage` SingleMessage | ✅ Rejected |
| B5 | `should_send=true` with empty output set | ✅ Rejected |

**Conclusion**: The spec correctly guards the `GetRequest` message type requirement and output constraints.

### Behavioral Mutation Tests (5/5 failed) — `behavioral_mutation_tests.rs`

| Test | Mutated Behavior | Result |
|------|-----------------|--------|
| M1 | Claim `delegation_map` changes after get request | ✅ Rejected |
| M2 | Claim `hashtable` changes after get request | ✅ Rejected |
| M3 | Claim `num_delegations` changes after get request | ✅ Rejected |
| M4 | Wrong reply value (not from hashtable lookup) when owner==me | ✅ Rejected |
| M5 | Reply message when owner≠me (should be Redirect) | ✅ Rejected |

**Conclusion**: The spec correctly preserves state invariants (delegation_map, h, num_delegations) and enforces the correct reply/redirect distinction.

### Logical Tests (5/5 failed) — `logical_tests.rs`

| Test | Logical Property Queried | Result |
|------|-------------------------|--------|
| L1 | Determinism: two post-states from same inputs must be equal | ✅ Rejected (not entailed — existential `b` flag allows non-determinism) |
| L2 | `constants` preserved by `next_get_request` | ✅ Rejected (not stated in spec) |
| L3 | `received_packet` is `None` after `next_get_request` | ✅ Rejected (not constrained by abstract spec) |
| L4 | Output always has exactly one packet | ✅ Rejected (`should_send=false` path produces empty output) |
| L5 | `sd` (single delivery state) unchanged | ✅ Rejected (`send_single_message` modifies `sd` on send) |

**Conclusion**: The spec does NOT over-constrain: it correctly avoids guaranteeing determinism, constants preservation, or packet-always-sent properties that are not part of the abstract `next_get_request` specification.

## Spec Weakness Analysis

The logical tests reveal interesting semantic boundaries:
- **L2/L3**: `next_get_request` does NOT constrain `post.constants` or `post.received_packet`. These are handled by the higher-level `next_get_request_postconditions` (a `closed spec`), not the abstract transition relation itself. This is a design-level separation between the abstract protocol spec and the implementation-level postconditions.
- **L1**: Non-determinism is intentional — the `exists |sm, m, b|` allows the boolean flag `b` to vary, permitting both "send" and "no-send" outcomes at the abstract level.
