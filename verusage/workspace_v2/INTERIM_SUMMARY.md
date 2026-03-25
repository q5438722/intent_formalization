# Spec Consistency — Interim Summary

**Date:** 2026-03-24  
**Coverage:** 242/720 files processed (34%)  
**Projects covered:** anvil-controller, anvil-library, atmosphere, ironkv (partial)

## Key Numbers

| Metric | Count |
|--------|-------|
| Files processed | 242 |
| Files with true positives | 165 (68%) |
| Total true positives | 379 |
| Total false positives filtered | ~840 (est.) |

## TP Pattern Distribution

| Pattern | Count | % |
|---------|-------|---|
| **Unverified axiom (external_body)** | 213 | 56% |
| Other / uncategorized | 106 | 28% |
| Weak/missing precondition | 28 | 7% |
| Missing relation properties (symmetry, transitivity) | 17 | 4% |
| Platform-dependent assumption | 8 | 2% |
| Missing injectivity/uniqueness | 4 | 1% |
| Numeric overflow/range gap | 3 | 1% |

## Key Findings

### 1. `external_body` is the dominant issue (56% of TPs)

Across all projects, the most common spec gap is **unverified axioms via `#[verifier::external_body]`**. These are proof obligations that Verus trusts on faith without checking. Many of them appear provable but haven't been discharged.

**Examples:**
- `anvil-controller`: `marshal_preserves_integrity` — marshal/unmarshal roundtrip is unverified across all ResourceView implementors
- `anvil-library`: temporal predicate equality axioms — intensional equality assumed from extensional equivalence
- `atmosphere`: `lemma_usize_u64` — claims all u64 values roundtrip through usize without any requires clause

### 2. Weak preconditions (7%)

Some specs accept inputs that are too broad. The `requires` clauses don't sufficiently constrain inputs, potentially allowing invalid states to pass verification.

### 3. Missing relation properties (4%)

`view_equal` and similar relations sometimes lack explicit symmetry/transitivity guarantees at the trait level, even when concrete implementations happen to satisfy them.

### 4. Platform-dependent assumptions (2%)

Several specs implicitly assume 64-bit architecture (e.g., `usize` range = `u64` range) without documenting this constraint.

## Per-Project Breakdown

### anvil-controller (26 files, 38 TPs)
- Dominated by `external_body` marshal/unmarshal axioms
- ResourceView trait macro generates unverified roundtrip lemmas

### anvil-library (77 files, 188 TPs)
- Temporal logic library has many `external_body` axioms
- Predicate equality, stability lemmas, and TLA operators trusted without proof

### atmosphere (60 files, 151 TPs)
- OS kernel verification with many trusted lemmas
- `usize`/`u64` conversion assumptions, page allocator invariants

### ironkv (2 files, 2 TPs)
- Serialization injectivity (`external_body` on usize impl)
- Platform-dependent usize marshalability

## Pipeline Health

- Generator produces ~5 φ per file
- ~20% of φ pass Verus entailment check (are actually entailed by spec)
- Critic filters ~50% of verified φ as false positives
- End-to-end: ~1.6 true positives per file on average

## Remaining Work

- 478 files still processing (est. completion: ~22:30 Beijing time)
- Remaining projects: ironkv (most), memory-allocator, node-replication, nrkernel, vest
- Final aggregated report after completion
