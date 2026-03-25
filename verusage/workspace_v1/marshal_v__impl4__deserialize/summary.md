# Adversarial Test Summary: `marshal_v__impl4__deserialize`

## Target
Marshalable trait and implementations for serialization/deserialization of `u64`, `usize`, `Option<T>`, `Vec<T>`, `Vec<u8>`, and `(T, U)` tuples, plus three sequence helper lemmas.

## Results: 15/15 tests FAILED as expected ✅

The specification correctly rejects all adversarial queries across all three categories.

---

### Boundary Tests (5/5 FAILED ✅)

| # | Test | Violated Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_subrange_i_greater_than_j` | `0 <= i <= j` (i=2 > j=1) | precondition not satisfied |
| 2 | `test_boundary_subrange_k_exceeds_len` | `k <= s.len()` (k=4 > len=3) | precondition not satisfied |
| 3 | `test_boundary_subrange_negative_index` | `0 <= i` (i=-1) | precondition not satisfied |
| 4 | `test_boundary_fold_sum_empty_seq` | `s.len() > 0` (empty seq) | precondition not satisfied |
| 5 | `test_boundary_fold_append_empty_seq` | `s.len() > 0` (empty seq) | precondition not satisfied |

**Conclusion**: All three lemmas properly guard their preconditions. Invalid index orderings, out-of-bounds values, and empty sequences are correctly rejected.

---

### Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutated Property | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_option_none_wrong_tag` | None tag: `seq![0]` → `seq![1]` | assertion failed |
| 2 | `test_mutation_option_some_missing_tag` | Some omits tag prefix byte | assertion failed |
| 3 | `test_mutation_pair_reversed_order` | `(a,b)` order: `a+b` → `b+a` | assertion failed |
| 4 | `test_mutation_u64_serialize_empty` | u64 serializes to empty seq | assertion failed |
| 5 | `test_mutation_usize_differs_from_u64` | usize ≠ u64 for same value | assertion failed |

**Conclusion**: The open spec function bodies fully determine serialization behavior. Incorrect tag bytes, missing prefixes, reversed component order, empty outputs, and cross-type inequality are all rejected.

---

### Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_derive_false` | Derive `false` from valid lemma use | assertion failed |
| 2 | `test_logical_u64_serialize_length_zero` | u64 serialize length = 0 | assertion failed |
| 3 | `test_logical_none_some_same_length` | None and Some have equal length | assertion failed |
| 4 | `test_logical_pair_length_first_only` | Pair length = first component only | assertion failed |
| 5 | `test_logical_non_injective_serialization` | Two different u64 → same bytes | assertion failed |

**Conclusion**: The specification does not admit unsound reasoning, maintains proper serialization length invariants, distinguishes None/Some formats, and preserves serialization injectivity.

---

## Overall Assessment

The specification is **consistent** with respect to all 15 adversarial queries tested. The `requires` clauses on the helper lemmas properly reject invalid inputs, the `open spec fn` definitions on `ghost_serialize` fully determine serialization format, and no unintended logical properties are entailable from the spec.

**Note**: Some tests triggered `recommendation not met` warnings for `ghost_serialize`'s `recommends self.is_marshalable()` clause. These are advisory warnings; the primary verification errors (precondition/assertion failures) confirm the spec's correctness.
