# Adversarial Test Summary: `marshal_v__impl4__is_marshalable`

## Target Specification
Marshalable trait + fold_left lemmas for IronKV marshaling infrastructure.

## Results: 15/15 tests FAILED (as expected)

All adversarial tests were correctly rejected by the specification, indicating the spec is adequately tight for the tested properties.

---

### Boundary Tests (5/5 FAILED ✓)

| Test | Violation | Result |
|------|-----------|--------|
| B1 | Empty seq for `lemma_seq_fold_left_sum_right` (requires `s.len() > 0`) | REJECTED |
| B2 | Negative index `i = -1` (requires `0 <= i`) | REJECTED |
| B3 | Index exceeds length `i = 3, len = 2` (requires `i <= s.len()`) | REJECTED |
| B4 | Negative accumulator `low = -1` (requires `0 <= low`) | REJECTED |
| B5 | Non-zero index on empty seq `i = 1, len = 0` | REJECTED |

**Conclusion**: All preconditions are properly enforced.

---

### Behavioral Mutation Tests (5/5 FAILED ✓)

| Test | Mutation | Result |
|------|----------|--------|
| M1 | Fold sum off by one (+1 added to equality) | REJECTED |
| M2 | Reversed inequality (claimed subrange > whole) | REJECTED |
| M3 | u64 serialize produces 0 bytes (wrong length) | REJECTED |
| M4 | usize/u64 serialize mismatch (contradicts definition) | REJECTED |
| M5 | Vec\<u8\> serialize has no length prefix | REJECTED |

**Conclusion**: The spec correctly rejects all mutated output relations.

---

### Logical Tests (5/5 FAILED ✓)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| L1 | u64 serialize injectivity (0 vs 1 produce same bytes) | REJECTED — spec doesn't entail injectivity |
| L2 | Strict monotonicity of fold (< instead of <=) | REJECTED — spec only guarantees ≤ |
| L3 | Fold commutativity over sequence reordering | REJECTED — not provable from spec |
| L4 | Fold independence from initial accumulator | REJECTED — clearly false |
| L5 | usize serialize always 8 bytes | REJECTED — spec doesn't expose `spec_u64_to_le_bytes` length axiom |

**Conclusion**: The spec does not entail unintended logical properties. Notably:
- **L1, L5**: The spec treats `ghost_serialize` as opaque for `u64` (via `spec_u64_to_le_bytes`), so concrete properties like injectivity and fixed-length output are not derivable. This is a deliberate abstraction boundary, though it means client code cannot reason about serialization length without additional lemmas.
- **L2**: The `<=` bound is tight — strict `<` does not hold when `f` produces empty sequences, confirming the spec correctly captures the weak inequality.
- **L3**: Fold commutativity requires arithmetic reasoning beyond what the decomposition lemma provides.

---

## Overall Assessment

The specification is **well-constrained** for the tested properties:
- Preconditions reject all invalid inputs
- Postconditions reject all mutated behaviors  
- No unintended logical consequences are derivable

**Potential weakness noted**: The spec is opaque about `spec_u64_to_le_bytes` properties (length = 8, injectivity). Client code relying on these must import separate axioms.
