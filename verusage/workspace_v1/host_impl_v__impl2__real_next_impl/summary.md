# Adversarial Proof Test Results — `host_impl_v__impl2__real_next_impl`

## Target
IronKV host protocol state machine (`real_next_impl`) — key spec functions: `next`, `next_step`, `receive_packet_next`, `process_received_packet_next`, and sub-step transition specs.

## Summary
All **15 tests** across 3 files **FAILED verification** as expected, confirming the spec correctly rejects all queried undesirable properties.

---

## Boundary Tests (5/5 FAIL ✓)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_incomplete_delegation_map` | `next` holds without `pre.wf()` | FAIL ✓ — spec rejects incomplete delegation map |
| 2 | `test_boundary_constants_differ` | `next` holds when constants change | FAIL ✓ — spec enforces `pre.constants == post.constants` |
| 3 | `test_boundary_invalid_send_in_ios` | `next` holds with `InvalidMessage` sends | FAIL ✓ — `no_invalid_sends` rejects this |
| 4 | `test_boundary_receive_empty_ios` | `receive_packet_next` holds with empty ios | FAIL ✓ — spec requires `ios.len() >= 1` |
| 5 | `test_boundary_value_at_max_len` | `valid_value` for value of length 1024 | FAIL ✓ — spec requires `len < 1024` (strict) |

---

## Behavioral Mutation Tests (5/5 FAIL ✓)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_retransmit_changes_h` | `spontaneously_retransmit` changes hashtable | FAIL ✓ — spec enforces `post == pre` |
| 2 | `test_mutation_stutter_with_nonempty_ios` | Stutter step with non-empty ios | FAIL ✓ — spec requires `ios.len() == 0` |
| 3 | `test_mutation_get_request_changes_delegation_map` | `next_get_request` changes delegation_map | FAIL ✓ — spec preserves delegation_map |
| 4 | `test_mutation_reply_produces_output` | `next_reply` produces non-empty output | FAIL ✓ — spec sets `out == Set::empty()` |
| 5 | `test_mutation_process_with_receive_io` | `process_received_packet_next` with Receive io | FAIL ✓ — spec requires all ios to be Send |

---

## Logical Tests (5/5 FAIL ✓)

| # | Test | Property Queried | Result | Insight |
|---|------|-----------------|--------|---------|
| 1 | `test_logical_next_not_deterministic` | `next` is deterministic (same pre+ios → unique post) | FAIL ✓ | Spec is a **relation**, not a function — multiple steps may produce different posts |
| 2 | `test_logical_next_requires_nonempty_ios` | `next` requires non-empty ios | FAIL ✓ | **Stutter** step allows empty ios |
| 3 | `test_logical_valid_key_rejects_zero` | Some key is invalid | FAIL ✓ | `valid_key` trivially returns `true` for all keys |
| 4 | `test_logical_receive_always_modifies_state` | `receive_packet_next` always modifies state | FAIL ✓ | **Timeout** case preserves state (`post == pre`) |
| 5 | `test_logical_receive_preserves_h` | Hashtable preserved during ReceivePacket step | FAIL ✓ | **Spec weakness detected**: `receive_packet` uses `..post` instead of `..pre`, leaving `h` unconstrained in the first disjunct |

---

## Notable Finding

**Test L5** exposes a potential spec weakness in `receive_packet()`:
```rust
post == AbstractHostState {sd: post.sd, received_packet: post.received_packet, ..post}
```
The `..post` spread makes this `post == post` (trivially true), meaning fields like `h`, `delegation_map`, `num_delegations`, and `received_requests` are **unconstrained** in the receive path's first disjunct. This is likely a translation artifact — `..pre` was probably intended (matching the pattern used in `next_reply`, `next_redirect`, etc.).
