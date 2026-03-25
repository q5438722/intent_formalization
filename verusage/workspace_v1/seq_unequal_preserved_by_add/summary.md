# Test Summary: `seq_unequal_preserved_by_add`

## Specification Under Test

```rust
pub proof fn seq_unequal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    requires s1 != s2
    ensures s1 + suffix != s2 + suffix
```

**Semantics:** If two sequences are unequal, appending the same suffix preserves their inequality.

---

## Results Overview

| Category | Tests | All Failed? | Spec Weakness Found? |
|----------|-------|-------------|----------------------|
| Boundary | 4 | ✅ Yes (4/4) | No |
| Behavioral Mutation | 4 | ✅ Yes (4/4) | No |
| Logical | 4 | ✅ Yes (4/4) | No |

**Total: 12/12 tests failed verification as expected.**

---

## Boundary Tests (4/4 FAILED ✅)

All tests correctly rejected by Verus due to precondition violation (`s1 != s2`).

| Test | Input | Failure Mode |
|------|-------|--------------|
| `boundary_test_1` | `s1=[1,2,3], s2=[1,2,3]` | Equal concrete sequences violate precondition |
| `boundary_test_2` | `s1=[], s2=[]` | Both empty sequences are equal |
| `boundary_test_3` | `s=s` (same variable) | Identity violates precondition |
| `boundary_test_4` | `s1=[0], s2=[0], suffix=[]` | Equal singletons with empty suffix |

## Behavioral Mutation Tests (4/4 FAILED ✅)

All tests correctly rejected by Verus — mutated output relations contradict the postcondition.

| Test | Mutation | Failure Mode |
|------|----------|--------------|
| `mutation_test_1` | Assert `s1+suffix == s2+suffix` | Negates postcondition (same-length unequal) |
| `mutation_test_2` | Assert `s1+suffix == s2+suffix` | Negates postcondition (differ at last element) |
| `mutation_test_3` | Assert `s1+suffix == s2+suffix` | Negates postcondition (different lengths) |
| `mutation_test_4` | Assert `s1 == s2` after valid call | Contradicts known precondition |

## Logical Tests (4/4 FAILED ✅)

All tests correctly rejected — unintended properties are NOT entailed by the spec.

| Test | Property Tested | Why Not Entailed |
|------|----------------|------------------|
| `logical_test_1` | `s1 != s2 ⟹ s1.len() != s2.len()` | Sequences can differ in values only, not length |
| `logical_test_2` | `s1 != s2 ⟹ s1[0] != s2[0]` | Sequences can share first element and differ elsewhere |
| `logical_test_3` | `s1+suffix1 != s2+suffix2` (different suffixes) | Different suffixes can compensate for inequality |
| `logical_test_4` | `suffix.len() > 0` | The spec works with empty suffix too |

---

## Conclusion

The specification `seq_unequal_preserved_by_add` is **consistent** with respect to all 12 adversarial queries:

- **Boundary completeness:** The precondition `s1 != s2` correctly guards against invalid inputs.
- **Behavioral correctness:** The postcondition `s1 + suffix != s2 + suffix` correctly rejects mutated output relations.
- **Logical tightness:** The spec does not entail unintended stronger properties (length inequality, element-wise inequality, cross-suffix inequality, or suffix non-emptiness).

No specification weaknesses were detected.
