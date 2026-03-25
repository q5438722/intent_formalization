# Adversarial Proof Test Summary: `maybe_ack_packet_impl`

**Target**: `single_delivery_model_impl2__maybe_ack_packet_impl.rs`
**Specification under test**: `SingleDelivery::maybe_ack_packet`, `should_ack_single_message`, `send_ack`

## Results: All 9 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification, indicating the spec is **consistent** with respect to the tested properties.

---

### Boundary Tests (3/3 FAILED as expected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_ack_not_message_with_nonempty_acks` | Non-Message pkt with non-empty acks (violates `pkt.msg is Message` precondition) | ❌ FAILED |
| 2 | `test_boundary_seqno_exceeds_tombstone` | seqno=6 > tombstone=5 (edge case at boundary), non-empty acks | ❌ FAILED |
| 3 | `test_boundary_empty_acks_when_should_ack` | should_ack true (seqno=0, tombstone=0), but empty acks instead of singleton | ❌ FAILED |

**Conclusion**: The spec correctly rejects invalid inputs at all tested boundaries.

---

### Behavioral Mutation Tests (3/3 FAILED as expected)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_wrong_ack_seqno` | ack_seqno=99 instead of matching pkt seqno=5 | ❌ FAILED |
| 2 | `test_mutation_wrong_ack_source` | ack.src=pkt.src instead of pkt.dst (not swapped) | ❌ FAILED |
| 3 | `test_mutation_ack_is_message_type` | ack.msg is Message instead of Ack | ❌ FAILED |

**Conclusion**: The spec correctly rejects all mutated (incorrect) output behaviors.

---

### Logical Tests (3/3 FAILED as expected)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_should_ack_implies_nonzero_seqno` | should_ack ⟹ seqno > 0 (false: seqno=0 is valid when tombstone=0) | ❌ FAILED |
| 2 | `test_should_ack_upward_monotonicity` | should_ack(seqno=5) ⟹ should_ack(seqno=6) (false: not upward-monotone) | ❌ FAILED |
| 3 | `test_ack_seqno_bounded_by_max` | maybe_ack_packet ⟹ ack_seqno < max_seqno (false: can equal max_seqno) | ❌ FAILED |

**Conclusion**: The spec does not accidentally entail any of the tested unintended logical properties.

---

## Overall Assessment

The `maybe_ack_packet` specification demonstrates strong consistency:
- **Boundary control**: Invalid inputs (wrong message type, out-of-range seqno, wrong ack set cardinality) are all rejected.
- **Behavioral precision**: Mutated outputs (wrong seqno, wrong src/dst, wrong msg variant) are all rejected.
- **Logical tightness**: The spec does not over-constrain (allows seqno=0, is not monotone upward, allows max_seqno values) — no unintended entailments detected.
