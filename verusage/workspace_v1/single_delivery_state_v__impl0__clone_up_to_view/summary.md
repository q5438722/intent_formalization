# Test Results Summary: `clone_up_to_view` Specification

**Target**: `single_delivery_state_v__impl0__clone_up_to_view.rs`
**Spec under test**: `CSingleMessage::clone_up_to_view` (`ensures c@ == self@`) and `CAckState::clone_up_to_view` (`ensures o@ == self@`)

---

## Results Overview

| Category | Tests | All Failed (as expected) |
|----------|-------|--------------------------|
| Boundary | 5/5 | ✅ Yes |
| Behavioral Mutation | 5/5 | ✅ Yes |
| Logical | 5/5 | ✅ Yes |
| **Total** | **15/15** | **✅ All rejected** |

---

## Boundary Tests (5/5 FAILED ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| B1 | Message seqno=0 implies seqno>0 | Rejected ✅ |
| B2 | InvalidMessage is a Message variant | Rejected ✅ |
| B3 | Ack ack_seqno=0 implies ack_seqno>0 | Rejected ✅ |
| B4 | AckState::new() has num_packets_acked>0 | Rejected ✅ |
| B5 | AckState::new() has non-empty un_acked | Rejected ✅ |

**Conclusion**: The spec correctly preserves edge values (zero seqno, zero ack, empty lists) and variant identity through view equality.

## Behavioral Mutation Tests (5/5 FAILED ✅)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| M1 | Seqno 5→6 | Rejected ✅ |
| M2 | Dst empty→non-empty | Rejected ✅ |
| M3 | Variant Message→Ack | Rejected ✅ |
| M4 | num_packets_acked 5→6 | Rejected ✅ |
| M5 | un_acked empty→one element | Rejected ✅ |

**Conclusion**: The spec rejects all field-level and variant-level mutations. The view equality postcondition (`c@ == self@`) is tight enough to detect changes to seqno, dst, message type, packet count, and list contents.

## Logical Tests (5/5 FAILED ✅)

| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| L1 | Messages with same seqno but different keys are equal | Rejected ✅ |
| L2 | AckState::new() has num_packets_acked=1 | Rejected ✅ |
| L3 | num_packets_acked equals un_acked length | Rejected ✅ |
| L4 | Swapping un_acked elements preserves equality | Rejected ✅ |
| L5 | Messages with same seqno/dst but different payload types are equal | Rejected ✅ |

**Conclusion**: The spec does not entail unintended logical properties. It correctly distinguishes keys, message types, and element ordering. The `num_packets_acked` and `un_acked` fields are properly independent.

---

## Overall Assessment

The `clone_up_to_view` specification is **consistent**: it rejects all 15 adversarial queries across boundary, behavioral, and logical dimensions. The postcondition `c@ == self@` (view equality) is sufficiently tight to:

1. **Preserve values at boundaries** — zero values and empty collections are faithfully reflected
2. **Reject all tested mutations** — changes to any field or variant are detected
3. **Prevent unintended reasoning** — the spec does not collapse distinct states, enforce false invariants, or allow order-insensitive reasoning

No spec weaknesses were identified in this test suite.
