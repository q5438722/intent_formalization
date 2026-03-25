# Adversarial Test Summary: `finite_set_to_seq_contains_all_set_elements`

## Specification Under Test

The target spec establishes a biconditional for **finite** sets:
```
∀ e: A. s.contains(e) ⟺ s.to_seq().contains(e)
```
This is proved using two `external_body` axioms (forward and reverse directions).

---

## Results: ALL 16 TESTS FAILED ✅

All adversarial tests were correctly rejected by the specification.

### Boundary Tests (5/5 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| B1 | Main lemma called on infinite set (`Set::new(\|x\| true)`) | ❌ `requires s.finite()` violated |
| B2 | Forward axiom with element not in set (`empty set, e=42`) | ❌ `requires s.contains(e)` violated |
| B3 | Reverse axiom on infinite set | ❌ `requires s.finite()` violated |
| B4 | Forward axiom on infinite set | ❌ `requires s.finite()` violated |
| B5 | Main lemma on infinite non-negative integers, then assert membership | ❌ `requires s.finite()` violated |

**Conclusion**: All preconditions are properly guarded. Invalid inputs are correctly rejected.

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| M1 | Assert absent element (42) appears in sequence of {1,2,3} | ❌ Assertion failed |
| M2 | Assert present element (1) is NOT in sequence of {1,2,3} | ❌ Assertion failed |
| M3 | Assert wrong set membership (99 ∈ {1,2}) | ❌ Assertion failed |
| M4 | Assert seq contains element (20) not in set {10} | ❌ Assertion failed |
| M5 | Assert negated biconditional for element 5 in set {5} | ❌ Assertion failed |

**Conclusion**: The spec correctly rejects all mutated behaviors. False positives, false negatives, and contradictory claims are all caught.

### Logical Tests (6/6 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| L1 | Assert `s.to_seq().len() == s.len()` (length preservation) | ❌ Not entailed by spec |
| L2 | Assert no duplicate elements in `to_seq()` | ❌ Not entailed by spec |
| L3 | Assert ascending ordering in `to_seq()` | ❌ Not entailed by spec |
| L4 | Assert biconditional for infinite set (over-generalization) | ❌ Not entailed by spec |
| L5 | Assert cross-set transfer (prove about s2 using lemma on s1) | ❌ Not entailed by spec |
| L6 | Assert concrete sequence structure (`seq =~= seq![1]`) | ❌ Not entailed by spec |

**Conclusion**: The spec does not entail any unintended logical properties. Stronger claims about length, uniqueness, ordering, cross-set reasoning, and concrete structure are all correctly rejected.

---

## Overall Assessment

The specification for `finite_set_to_seq_contains_all_set_elements` is **well-bounded**:

- **Precondition completeness**: The `requires s.finite()` guard prevents misuse on infinite sets. Element-level preconditions on helper axioms are also properly enforced.
- **Postcondition precision**: The biconditional is tight enough to reject mutated behaviors (false positives and negatives) but does not over-specify structural properties like ordering or length.
- **Logical containment**: The spec does not leak unintended entailments — it cannot be used to derive length, uniqueness, ordering, or cross-set facts.

**No specification weaknesses were detected.**
