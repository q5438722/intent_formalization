# Adversarial Test Summary: `share_mapping`

## Target
`kernel__create_and_share_pages__impl0__share_mapping.rs` — the `share_mapping` function which maps a shared page into a target process's address space, incrementing its reference counter.

## Results

| File | Tests | All Failed? | Verdict |
|------|-------|-------------|---------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors) | Spec correctly rejects invalid inputs |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5 errors) | Spec correctly rejects mutated behaviors |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors) | Spec correctly rejects unentailed properties |
| `correctness_tests.rs` (combined) | 15 | ✅ Yes (15 errors) | All adversarial queries rejected |

## Boundary Tests (Precondition Violations)
| ID | Description | Result |
|----|-------------|--------|
| BT1 | VA=0 not 4k-valid (L4 index < KERNEL_MEM_END_L4INDEX) | ✅ FAIL |
| BT2 | Unaligned page pointer (ptr=1) invalid | ✅ FAIL |
| BT3 | page_index == NUM_PAGES rejected (off-by-one) | ✅ FAIL |
| BT4 | PageEntry with present=true not empty | ✅ FAIL |
| BT5 | insert_page_mapping rejects domain mismatch | ✅ FAIL |

## Behavioral Mutation Tests (Incorrect Output Relations)
| ID | Description | Result |
|----|-------------|--------|
| BM1 | Non-target page mapping changed → rejected | ✅ FAIL |
| BM2 | New mapping not actually inserted → rejected | ✅ FAIL |
| BM3 | PageEntry with non-zero addr not empty | ✅ FAIL |
| BM4 | Wrong mapping inserted instead of correct one → rejected | ✅ FAIL |
| BM5 | PageEntry with write=true not empty | ✅ FAIL |

## Logical Tests (Unentailed Properties)
| ID | Description | Result |
|----|-------------|--------|
| LT1 | insert_page_mapping determinism (disagreement rejected) | ✅ FAIL |
| LT2 | page_ptr_valid doesn't fix ptr to a unique value | ✅ FAIL |
| LT3 | page_ptr↔page_index roundtrip consistency (negation rejected) | ✅ FAIL |
| LT4 | va_4k_valid does NOT imply va_2m_valid | ✅ FAIL |
| LT5 | page_index2page_ptr injectivity (equality of distinct rejected) | ✅ FAIL |

## Conclusion

All 15 adversarial tests were **correctly rejected** by the specification. The spec for `share_mapping` and its supporting definitions (`insert_page_mapping`, validity predicates, `PageEntry::is_empty`) demonstrate appropriate strength:

- **Boundary control**: Invalid inputs (misaligned pointers, out-of-range indices, kernel-space VAs) are properly excluded.
- **Behavioral precision**: The `insert_page_mapping` spec precisely constrains which pages change and how — mutating non-target pages, omitting the new mapping, or inserting wrong mappings are all rejected.
- **Logical soundness**: The spec does not entail properties beyond what is stated — determinism is correctly enforced, validity predicates are non-trivial, and structural properties (roundtrip consistency, injectivity) hold as expected.

No specification weaknesses were found in these tests.
