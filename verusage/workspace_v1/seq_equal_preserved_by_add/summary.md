# Adversarial Test Summary: `seq_equal_preserved_by_add`

## Specification Under Test

```
pub proof fn seq_equal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    ensures s1 == s2 <==> s1 + suffix == s2 + suffix
```

No preconditions. The postcondition is a biconditional: sequence equality is preserved by appending the same suffix, and conversely, equal concatenations (with the same suffix) imply equal base sequences.

---

## Results Overview

| Category | Tests | All Failed? | Verdict |
|----------|-------|-------------|---------|
| Boundary | 4 | ✅ Yes | Spec correctly rejects edge-case abuse |
| Behavioral Mutation | 4 | ✅ Yes | Spec correctly rejects mutated relations |
| Logical | 4 | ✅ Yes | Spec correctly rejects unintended inferences |

**Total: 12/12 tests failed verification as expected.**

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `boundary_test_1_unequal_seqs_equal_concat` | s1≠s2 ∧ s1+suffix == s2+suffix | ❌ FAILED (correct) |
| 2 | `boundary_test_2_different_lengths_empty_suffix` | len(s1)≠len(s2) ∧ s1==s2 (with empty suffix) | ❌ FAILED (correct) |
| 3 | `boundary_test_3_same_base_different_suffixes` | s+suffix1 == s+suffix2 (suffix1≠suffix2) | ❌ FAILED (correct) |
| 4 | `boundary_test_4_empty_vs_nonempty` | empty == non-empty after concat | ❌ FAILED (correct) |

## Behavioral Mutation Tests (`mutation_tests.rs`)

| # | Test | Mutation Applied | Result |
|---|------|-----------------|--------|
| 1 | `mutation_test_1_equal_seqs_unequal_concat` | s1==s2 but claim s1+suffix ≠ s2+suffix | ❌ FAILED (correct) |
| 2 | `mutation_test_2_equal_concat_unequal_seqs` | s1==s2 but claim s1 ≠ s2 | ❌ FAILED (correct) |
| 3 | `mutation_test_3_unequal_seqs_forced_equal_concat` | s1≠s2 but claim s1+suffix == s2+suffix | ❌ FAILED (correct) |
| 4 | `mutation_test_4_swapped_operand_order` | Claim s1+suffix == suffix+s2 (wrong order) | ❌ FAILED (correct) |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `logical_test_1_prefix_cancellation_not_implied` | prefix+s1 == prefix+s2 (prefix, not suffix) | ❌ FAILED (correct) |
| 2 | `logical_test_2_suffix_decomposition_uniqueness` | concat equality ⟹ suffix equality | ❌ FAILED (correct) |
| 3 | `logical_test_3_commutativity_not_implied` | s1+s2 == s2+s1 (commutativity) | ❌ FAILED (correct) |
| 4 | `logical_test_4_stronger_elementwise_claim` | (s1+suffix)[0] == (s2+suffix)[0] for s1≠s2 | ❌ FAILED (correct) |

---

## Conclusion

The specification `s1 == s2 <==> s1 + suffix == s2 + suffix` is **well-bounded**:

1. **Input boundaries are sound**: The spec correctly rejects attempts to equate sequences of different lengths, different content, or with different suffixes.
2. **Behavioral correctness is tight**: Both directions of the biconditional are enforced — mutating either direction is correctly rejected.
3. **No unintended logical entailments detected**: The spec does not accidentally imply prefix cancellation, suffix decomposition uniqueness, concatenation commutativity, or element-wise equality for unequal sequences.

No specification weaknesses were found in this evaluation.
