# Adversarial Test Results — `verus_extra__map_fold_ok`

## Target Specification

- **Function**: `map_fold_ok<A, B>(s: Set<A>, f: spec_fn(A) -> B)`
- **Requires**: `s.finite()`
- **Ensures**: `map_fold(s, f) =~= s.map(f)`

The lemma proves that folding a set with insert operations (`map_fold`) produces the same result as the built-in `Set::map` operation.

---

## Test Results Summary

| # | Category | Test Name | Expected | Actual | Status |
|---|----------|-----------|----------|--------|--------|
| 1 | Boundary | `boundary_test_infinite_set` | FAIL | precondition not satisfied | ✅ |
| 2 | Boundary | `boundary_test_predicate_set_not_provably_finite` | FAIL | precondition not satisfied | ✅ |
| 3 | Boundary | `boundary_test_empty_set_contains_element` | FAIL | assertion failed | ✅ |
| 4 | Behavioral | `mutation_wrong_output_equals_original` | FAIL | assertion failed | ✅ |
| 5 | Behavioral | `mutation_deny_correct_element` | FAIL | assertion failed | ✅ |
| 6 | Behavioral | `mutation_claim_spurious_element` | FAIL | assertion failed | ✅ |
| 7 | Logical | `logical_cardinality_preserved_noninjective` | FAIL | assertion failed | ✅ |
| 8 | Logical | `logical_subset_preservation_unproven` | FAIL | assertion failed | ✅ |
| 9 | Logical | `logical_cross_function_equivalence` | FAIL | assertion failed | ✅ |

**Result: 9/9 adversarial tests correctly rejected.**

---

## Analysis by Category

### Boundary Tests (3/3 rejected)

1. **Infinite set**: `Set::new(|i| true)` is not finite — the `requires s.finite()` precondition correctly rejects it.
2. **Predicate-defined set**: `Set::new(|i| 0 <= i < 10)` is not *provably* finite in Verus (requires explicit construction via `insert`), so the precondition is enforced.
3. **Empty set wrong claim**: After proving `map_fold({}, f) =~= {}.map(f) = {}`, the verifier correctly rejects `{}.contains(0)`.

### Behavioral Mutation Tests (3/3 rejected)

4. **Wrong output**: `map_fold({1,2,3}, x+1) = {2,3,4}`, not `{1,2,3}` — correctly rejected.
5. **Deny correct element**: `map_fold({1,2}, x*2) = {2,4}` — denying `2 ∈ result` is correctly rejected.
6. **Claim spurious element**: `map_fold({1,2}, x+10) = {11,12}` — claiming `1 ∈ result` is correctly rejected.

### Logical Tests (3/3 rejected)

7. **Cardinality preservation**: Constant function `f(x)=0` maps `{1,2,3}` to `{0}`, so `len=1 ≠ 3`. Spec does not entail cardinality preservation for non-injective functions.
8. **Subset preservation (unproven)**: Although `s1⊆s2 ⟹ map(s1)⊆map(s2)` is mathematically true, the spec does not prove it without an explicit lemma call — the verifier correctly cannot derive it from `map_fold_ok`'s ensures alone.
9. **Cross-function equivalence**: `map_fold(s, x+1) ≠ map_fold(s, x*10)` — the spec correctly distinguishes results under different mapping functions.

---

## Conclusion

The specification for `map_fold_ok` is **consistent**: it correctly rejects all 9 adversarial queries across boundary violations, behavioral mutations, and unintended logical inferences. The precondition (`s.finite()`) is properly enforced, the postcondition (`=~= s.map(f)`) is tight enough to reject incorrect outputs, and the spec does not entail unintended properties like cardinality preservation or distributivity.

**No specification weaknesses were detected.**
