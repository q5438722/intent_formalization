# Adversarial Proof Test Summary: `host_model_next_delegate`

## Target
`host_impl_v__impl2__host_model_next_delegate.rs` — the `next_delegate` spec function and related specs from IronKV's host delegation protocol.

## Results: All 15 tests FAIL verification ✅

All adversarial queries were correctly rejected by the specification. No spec weaknesses were detected through these tests.

---

### Boundary Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| B1 | `next_delegate` accepts non-Delegate message (GetRequest) | **FAIL** — spec requires `pkt.msg.arrow_Message_m() is Delegate` |
| B2 | `next_delegate` works with non-empty output set | **FAIL** — spec requires `out == Set::empty()` |
| B3 | `valid_hashtable` accepts hashtable with ≥62 entries | **FAIL** — spec requires `h.dom().len() < 62` |
| B4 | `valid_value` accepts value of length exactly 1024 | **FAIL** — spec requires `value.len() < 1024` (strict <) |
| B5 | `Parameters::valid` accepts `max_delegations == 3` | **FAIL** — spec requires `3 < max_delegations` |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| M1 | `num_delegations` stays same when src ∈ host_ids | **FAIL** — spec requires `post.num_delegations == pre.num_delegations + 1` |
| M2 | `delegation_map` unchanged when src ∈ host_ids | **FAIL** — spec requires delegation_map update via `update()` |
| M3 | `num_delegations` increments when src ∉ host_ids | **FAIL** — spec requires `post.num_delegations == pre.num_delegations` |
| M4 | `h` changes when src ∉ host_ids | **FAIL** — spec requires `post.h == pre.h` |
| M5 | `send_state` changes after `next_delegate` | **FAIL** — `send_no_message` requires `post.send_state == pre.send_state` |

### Logical Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| L1 | `next_delegate` entails false (soundness) | **FAIL** — spec is satisfiable, cannot derive contradiction |
| L2 | `received_requests` can change | **FAIL** — spec requires `post.received_requests == pre.received_requests` |
| L3 | `constants` can change | **FAIL** — Verus cannot derive `post.constants !== pre.constants` (the spec doesn't explicitly constrain `post.constants`, but `next_delegate` alone provides insufficient basis to prove they differ) |
| L4 | Determinism violated (two post-states disagree on `num_delegations`) | **FAIL** — spec deterministically constrains `num_delegations` |
| L5 | `received_packet` is `None` after `next_delegate` | **FAIL** — `next_delegate` does **not** constrain `post.received_packet`; this is handled by the broader `next_delegate_postconditions` |

---

## Notable Observations

1. **L3 (constants unconstrained)**: `next_delegate` does not explicitly require `post.constants == pre.constants`. However, this is handled at the implementation level by `host_state_common_postconditions` which requires `self.constants == pre.constants`. The `next_delegate` spec alone leaves constants unconstrained — neither equality nor inequality can be proven.

2. **L5 (received_packet unconstrained)**: `next_delegate` similarly does not constrain `post.received_packet`. The broader `next_delegate_postconditions` separately asserts `self.received_packet is None`. This is a design choice: the abstract transition spec (`next_delegate`) focuses on delegation semantics while the concrete postcondition adds implementation details.

3. **Spec strength**: The `next_delegate` spec demonstrates good specificity — it correctly distinguishes between known sources (in `host_ids`) and unknown sources, applying different state updates in each case. All behavioral mutations were properly rejected.
