# Adversarial Test Summary: `lemma_seqno_in_un_acked_list`

## Target Specification

```
proof fn lemma_seqno_in_un_acked_list(&self, dst: AbstractEndPoint, k: int)
    requires self.valid(dst), 0 <= k < self.un_acked@.len()
    ensures  self.un_acked@[k].arrow_Message_seqno() == self.num_packets_acked + k + 1
```

## Results: All 12 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (4/4 rejected)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_k_at_length` | `k == len` (off-by-one upper bound) | ❌ precondition `k < len` |
| `test_boundary_k_negative` | `k == -1` (negative index) | ❌ precondition `0 <= k` |
| `test_boundary_empty_list` | empty list, `k == 0` | ❌ precondition `k < len` (len=0) |
| `test_boundary_invalid_state` | `!state.valid(dst)` | ❌ precondition `self.valid(dst)` |

### Behavioral Mutation Tests (4/4 rejected)

| Test | Mutated Postcondition | Result |
|------|----------------------|--------|
| `test_mutation_off_by_one_minus` | `seqno == acked + k` (missing +1) | ❌ postcondition failed |
| `test_mutation_off_by_one_plus` | `seqno == acked + k + 2` (extra +1) | ❌ postcondition failed |
| `test_mutation_ignore_acked` | `seqno == k + 1` (drop acked) | ❌ postcondition failed |
| `test_mutation_ignore_index` | `seqno == acked` (drop k) | ❌ postcondition failed |

### Logical Tests (4/4 rejected)

| Test | Unwarranted Property | Result |
|------|---------------------|--------|
| `test_logical_without_validity` | Result holds without `valid(dst)` | ❌ postcondition failed |
| `test_logical_strictly_greater` | `seqno > acked + k + 1` (strict) | ❌ postcondition failed |
| `test_logical_cross_state_equality` | Two states share seqnos at same index | ❌ postcondition failed |
| `test_logical_seqno_zero` | `seqno == 0` possible | ❌ postcondition failed |

---

## Conclusion

The specification for `lemma_seqno_in_un_acked_list` is **consistent** with respect to all 12 adversarial queries:

- **Boundary**: Preconditions correctly guard against out-of-bounds indices and invalid states.
- **Behavioral**: The exact formula `acked + k + 1` is precise — no off-by-one or partial-term variants pass.
- **Logical**: The spec does not entail unwarranted properties (no validity bypass, no strict inequality, no cross-state conflation, no zero-seqno).
