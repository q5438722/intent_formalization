# Verusage Adversarial Test Completeness Summary

Analysis of all 341 `summary.md` files in `verusage/workspace/`.

## Overview

| Status | Count |
|--------|-------|
| **Complete** (all tests rejected as intended) | **333** |
| **Incomplete** (has fail-to-reject tests) | **6** |
| **Spec weakness noted** (all tests rejected, but weakness identified) | **2** |
| **Total** | **341** |

**97.7%** of cases have all adversarial tests rejected as intended — full completeness. All **Boundary** and **Behavioral Mutation** tests across all 341 cases are rejected. Failures to reject occur **only in Logical Tests**.

---

## Incomplete Cases (Fail-to-Reject Tests)

| # | Case | Category | Fail-to-Reject Test | Reason |
|---|------|----------|---------------------|--------|
| 1 | `marshal_v__impl0__serialized_size` | Logical | `test_logical_usize_always_marshalable` | `is_marshalable` for `usize` is vacuously true — Verus assumes `usize` fits within 64 bits, so `*self as int <= u64::MAX` always holds |
| 2 | `marshal_v__impl1__lemma_serialize_injective` | Logical | `test_logical_symmetry_beyond_spec` | Symmetry of `view_equal` is derivable beyond spec guarantee |
| 3 | `marshal_v__impl1__lemma_serialize_injective` | Logical | `test_logical_converse_of_lemma` | Converse of the lemma is provable (unintended stronger property) |
| 4 | `marshal_v__impl4__lemma_same_views_serialize_the_same` | Logical | `test_logical_usize_always_marshalable` | Same vacuous `is_marshalable` for `usize` issue |
| 5 | `marshal_v__impl5__lemma_same_views_serialize_the_same` | Logical | `forall \|x: usize\| x.is_marshalable()` | Same vacuous `is_marshalable` for `usize` issue |
| 6 | `marshal_v__impl5__lemma_serialization_is_not_a_prefix_of` | Logical (LT4) | `view_equal(x, x)` reflexivity | Reflexivity of `view_equal` not in spec but derivable |
| 7 | `va_range__impl2_new` | Logical | `test_logical_lemma_unsoundness` | Derives `false` via `external_body` lemma bug — unconstrained quantifier variable (`j` not bounded) |

---

## Spec Weaknesses Detected (All Tests Rejected, But Weakness Noted)

| # | Case | Weakness |
|---|------|----------|
| 1 | `memory_manager__spec_impl__impl0__create_pagetable_l2_entry` | (a) Null pointer (0) passes `page_ptr_valid`; (b) `page_table_pages@[page_map_ptr]` value not constrained in postcondition |
| 2 | `single_delivery_model_impl2__receive_ack_impl` | `receive_state` not constrained in update branch — spec uses `..post` (tautological) instead of `..pre` |

---

## Key Observations

- All **Boundary** and **Behavioral Mutation** tests pass across all 341 cases — no specification is so weak that incorrect inputs or mutated outputs are accepted.
- The most common root cause of fail-to-reject (4/7 tests) is the **vacuous `usize` marshalability** — a Verus architecture assumption (`usize ≤ u64::MAX`), not a spec bug per se.
- **1 soundness issue**: `va_range__impl2_new` allows deriving `false` through an under-constrained `external_body` lemma.
