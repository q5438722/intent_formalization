# Adversarial Test Summary: `marshal_v__impl3__lemma_view_equal_symmetric`

## Target Specification

The `Marshalable` trait defines an abstract `view_equal` spec function and a `lemma_view_equal_symmetric` proof that guarantees **symmetry**: `self.view_equal(other) == other.view_equal(self)`. Concrete implementations exist for `Option<T>` and `(T, U)`.

## Results

**All 16 adversarial tests FAILED verification as expected.** The spec correctly rejects all tested invalid properties.

### Boundary Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_some_equals_none` | Some(a) view_equal None should not hold | FAIL ✅ |
| `test_boundary_none_equals_some` | None view_equal Some(a) should not hold | FAIL ✅ |
| `test_boundary_tuple_partial_first_only` | Tuple equality with only 1st component matching | FAIL ✅ |
| `test_boundary_tuple_partial_second_only` | Tuple equality with only 2nd component matching | FAIL ✅ |
| `test_boundary_nested_option_mismatch` | Some(Some(a)) vs Some(None) mismatch | FAIL ✅ |

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_mutation_none_none_is_false` | Negated correct output (None-None = true) | FAIL ✅ |
| `test_mutation_symmetry_negated` | Negated symmetry result | FAIL ✅ |
| `test_mutation_anti_symmetric` | Contradicts symmetry (a=b ∧ ¬b=a) | FAIL ✅ |
| `test_mutation_tuple_equal_negated` | Negated correct tuple equality | FAIL ✅ |
| `test_mutation_some_equal_negated` | Negated correct Some equality | FAIL ✅ |

### Logical Tests (6/6 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_reflexivity` | Reflexivity (a.view_equal(a)) — not guaranteed | FAIL ✅ |
| `test_logical_transitivity` | Transitivity — not guaranteed | FAIL ✅ |
| `test_logical_view_equal_implies_eq` | view_equal ⇒ structural equality — not guaranteed | FAIL ✅ |
| `test_logical_common_neighbor` | Common neighbor property — not guaranteed | FAIL ✅ |
| `test_logical_option_reflexivity` | Option reflexivity — depends on inner | FAIL ✅ |
| `test_logical_tuple_reflexivity` | Tuple reflexivity — depends on components | FAIL ✅ |

## Findings

The specification is **tight and well-bounded**:

1. **Boundary**: The concrete `Option` and tuple definitions correctly reject mismatched variants and partial component matches.
2. **Behavioral**: Symmetry is enforced — mutated/negated outputs are all rejected.
3. **Logical**: The spec does **not** over-promise. Notably:
   - **Reflexivity** is not entailed (by design — `view_equal` is abstract in the trait).
   - **Transitivity** is not entailed.
   - **Structural equality** is not conflated with `view_equal`.
   - The spec is not accidentally an equivalence relation.

No specification weaknesses (unintended entailments) were detected.
