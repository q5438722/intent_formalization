# Adversarial Proof Test Summary: `single_delivery_state_v__impl0__truncate`

## Target
`truncate_un_ack_list` — spec function that removes leading `SingleMessage::Message` elements whose `seqno <= seqno_acked`.

## Results: All 15 tests FAILED verification (as expected ✅)

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (`boundary_tests.rs`) — 5/5 FAIL ✅

| Test | Property Queried | Result |
|------|-----------------|--------|
| B1 | Empty input returns non-empty output | FAIL ✅ |
| B2 | Ack variants are removed by truncation | FAIL ✅ |
| B3 | seqno_acked=0 removes message with seqno=1 | FAIL ✅ |
| B4 | seqno_acked < all seqnos still removes elements | FAIL ✅ |
| B5 | InvalidMessage variants are removed | FAIL ✅ |

**Conclusion**: The spec correctly distinguishes `Message` variants from `Ack`/`InvalidMessage` and respects the `seqno <= seqno_acked` boundary.

---

### Behavioral Mutation Tests (`mutation_tests.rs`) — 5/5 FAIL ✅

| Test | Property Queried | Result |
|------|-----------------|--------|
| M1 | Truncating [1,2,3] at 2 yields length 3 | FAIL ✅ |
| M2 | First element after truncation has wrong seqno | FAIL ✅ |
| M3 | Truncating [1,5] at 3 preserves removed element | FAIL ✅ |
| M4 | Result length is 2 instead of 1 | FAIL ✅ |
| M5 | Full truncation still leaves elements | FAIL ✅ |

**Conclusion**: The spec correctly computes truncation results and rejects all mutated output claims.

---

### Logical Tests (`logical_tests.rs`) — 5/5 FAIL ✅

| Test | Property Queried | Result |
|------|-----------------|--------|
| L1 | Truncation is the identity function | FAIL ✅ |
| L2 | First element always has seqno == seqno_acked + 1 (for arbitrary lists) | FAIL ✅ |
| L3 | Truncation always makes list strictly shorter | FAIL ✅ |
| L4 | Truncation result is order-independent | FAIL ✅ |
| L5 | Number of removed elements equals seqno_acked | FAIL ✅ |

**Conclusion**: The spec does not entail any of these unintended stronger properties.

---

## Overall Assessment

The `truncate_un_ack_list` specification is **consistent** with respect to all 15 adversarial queries:
- **Boundary integrity**: Invalid variant types and edge-case seqno values are handled correctly.
- **Behavioral correctness**: Incorrect output lengths, elements, and emptiness claims are all rejected.
- **Logical soundness**: No unintended properties (identity, order-independence, stronger bounds) are entailed.

No specification weaknesses were detected.
