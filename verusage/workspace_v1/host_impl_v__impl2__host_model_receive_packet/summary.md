# Test Execution Summary: `host_model_receive_packet`

## Target
`host_impl_v__impl2__host_model_receive_packet.rs` — the `receive_packet` spec for IronKV's packet reception protocol.

## Results Overview

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |

All 15 tests **failed verification** as intended — the spec correctly rejects all tested invalid/mutated/unintended properties.

---

## Boundary Tests (boundary_tests.rs)

| ID | Description | Result |
|---|---|---|
| B1 | InvalidMessage cannot be buffered (received_packet stays None) | FAIL ✅ |
| B2 | Full buffer + state change = unreachable | FAIL ✅ |
| B3 | `okay_to_ignore_packets()` is opaque — can't prove it's true | FAIL ✅ |
| B4 | Fresh packet buffers exactly `pkt`, not a different packet | FAIL ✅ |
| B5 | Parameters with wrong `max_delegations` ≠ `static_params()` | FAIL ✅ |

## Behavioral Mutation Tests (behavioral_mutation_tests.rs)

| ID | Description | Result |
|---|---|---|
| M1 | Fresh packet MUST be buffered — asserting None fails | FAIL ✅ |
| M2 | Full buffer → sd unchanged — asserting change fails | FAIL ✅ |
| M3 | Duplicate packet stays unbuffered — asserting Some fails | FAIL ✅ |
| M4 | Full buffer → output empty — asserting non-empty fails | FAIL ✅ |
| M5 | Fresh packet → receive_state updated — asserting same fails | FAIL ✅ |

## Logical Tests (logical_tests.rs)

| ID | Description | Result | Finding |
|---|---|---|---|
| L1 | Soundness: `receive_packet` does not entail `false` | FAIL ✅ | Spec is sound |
| L2 | Determinism: two post-states cannot have different `received_packet` | FAIL ✅ | No spurious non-determinism |
| L3 | Frame: `post.h == pre.h` when only first disjunct applies | FAIL ✅ | **Spec weakness detected** |
| L4 | Frame: `post.num_delegations == pre.num_delegations` | FAIL ✅ | **Spec weakness detected** |
| L5 | Frame: `post.received_requests == pre.received_requests` | FAIL ✅ | **Spec weakness detected** |

---

## Key Finding: Tautological Frame Condition

Tests L3–L5 reveal a **spec weakness** in `receive_packet`'s first disjunct:

```rust
post == AbstractHostState { sd: post.sd, received_packet: post.received_packet, ..post }
```

The `..post` fills remaining fields from `post` itself, making this equivalent to `post == post` (always true). This is a **tautological frame condition** that fails to constrain `post.h`, `post.num_delegations`, `post.delegation_map`, `post.constants`, and `post.received_requests`.

The correct frame should use `..pre`:
```rust
post == AbstractHostState { sd: post.sd, received_packet: post.received_packet, ..pre }
```

This would properly enforce that only `sd` and `received_packet` change during packet reception. The weakness is compensated at the concrete level by `host_state_common_postconditions` (which enforces `self.constants == pre.constants` and `self.valid()`), but the abstract spec `receive_packet` alone does not provide a complete frame.
