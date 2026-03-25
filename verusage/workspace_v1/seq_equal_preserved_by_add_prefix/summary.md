# Adversarial Proof Test Summary

**Target**: `seq_equal_preserved_by_add_prefix`  
**Spec**: `ensures s1 == s2 <==> prefix + s1 == prefix + s2` (no preconditions)

---

## Results: All 9 tests FAILED verification ✅ (as intended)

The specification correctly rejects all adversarial properties.

### Boundary Tests (3/3 rejected)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_boundary_empty_prefix_unequal_seqs` | `[] + [1,2,3] == [] + [4,5,6]` (unequal seqs, empty prefix) | FAIL ✅ |
| `test_boundary_different_lengths` | `[1,2] == [1,2,3]` (different-length seqs) | FAIL ✅ |
| `test_boundary_nonempty_vs_empty` | `[10,20] + [1] == [10,20] + []` (nonempty vs empty) | FAIL ✅ |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_mutation_negate_forward` | `prefix + s != prefix + s` when `s1 == s2` (negated forward) | FAIL ✅ |
| `test_mutation_negate_backward` | `[1] == [2]` despite being different (negated backward) | FAIL ✅ |
| `test_mutation_wrong_element` | `(prefix + s1)[1] == 99` when actual value is 20 | FAIL ✅ |

### Logical Tests (3/3 rejected)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_logical_different_prefixes_imply_concat_equal` | `p1 + s == p2 + s` for `p1 ≠ p2` (over-generalization) | FAIL ✅ |
| `test_logical_concat_commutativity` | `prefix + s == s + prefix` (commutativity) | FAIL ✅ |
| `test_logical_unentailed_length_relationship` | `prefix.len() <= s1.len()` (unentailed structural claim) | FAIL ✅ |

---

## Conclusion

The specification `s1 == s2 <==> prefix + s1 == prefix + s2` is **tight and consistent**:

- **Boundary**: Correctly rejects invalid equality claims for edge-case inputs (empty, different-length sequences).
- **Behavioral**: Correctly rejects all mutated relations (negated forward/backward directions, wrong element values).
- **Logical**: Does not over-entail — rejects claims about different prefixes, commutativity, and unrelated structural properties.

No weaknesses detected. The biconditional specification precisely captures the intended property without admitting unintended reasoning.
