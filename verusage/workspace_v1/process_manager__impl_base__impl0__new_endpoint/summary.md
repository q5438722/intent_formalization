# Adversarial Proof Test Summary: `new_endpoint`

**Target**: `process_manager__impl_base__impl0__new_endpoint.rs`  
**Function**: `ProcessManager::new_endpoint` — allocates a new endpoint from a page, assigns it to a thread's endpoint descriptor, updates the owning container's quota and endpoint set.

---

## Results Overview

| Test File | Tests | Failed (expected) | Passed (unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 5 | 5 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 5 | 5 ✅ | 0 |
| `logical_tests.rs` | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15 ✅** | **0** |

All 15 adversarial tests correctly fail verification, indicating the specification properly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (5/5 FAIL ✅)

| # | Test | Violated Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_endpoint_index_at_max` | `endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` | FAIL ✅ |
| 2 | `test_boundary_subtract_underflow` | `quota.mem_4k > 0` (subtract from 0) | FAIL ✅ |
| 3 | `test_boundary_page_in_endpoint_dom` | `page_closure.contains(page_ptr_1) == false` | FAIL ✅ |
| 4 | `test_boundary_page_in_container_dom` | `page_closure.contains(page_ptr_1) == false` | FAIL ✅ |
| 5 | `test_boundary_zero_quota` | `quota.mem_4k > 0` | FAIL ✅ |

## Behavioral Mutation Tests (5/5 FAIL ✅)

| # | Test | Mutated Output | Result |
|---|------|---------------|--------|
| 1 | `test_mutation_subtract_wrong_amount` | mem_4k: 100-1=80 (should be 99) | FAIL ✅ |
| 2 | `test_mutation_subtract_changes_mem2m` | mem_2m changed (should be preserved) | FAIL ✅ |
| 3 | `test_mutation_subtract_changes_pcid` | pcid changed (should be preserved) | FAIL ✅ |
| 4 | `test_mutation_set_mem_4k_wrong` | set_mem_4k(50) gives 60 (should be 50) | FAIL ✅ |
| 5 | `test_mutation_endpoint_state_receive` | SEND =~= RECEIVE (distinct values) | FAIL ✅ |

## Logical Tests (5/5 FAIL ✅)

| # | Test | Unstated Property Tested | Result |
|---|------|--------------------------|--------|
| 1 | `test_logical_subtract_determinism` | Two valid results must disagree on mem_4k | FAIL ✅ |
| 2 | `test_logical_zero_subtract_changes_value` | Subtracting 0 changes mem_4k | FAIL ✅ |
| 3 | `test_logical_subtract_changes_mem1g` | Subtraction changes mem_1g | FAIL ✅ |
| 4 | `test_logical_disjoint_ptr_in_both_domains` | ptr in both endpoint_dom and container_dom | FAIL ✅ |
| 5 | `test_logical_rf_counter_always_above_one` | rf_counter always > 1 | FAIL ✅ |

---

## Conclusion

The `new_endpoint` specification correctly:
- **Rejects invalid inputs**: out-of-range endpoint indices, zero quotas, pages already in use
- **Rejects incorrect behaviors**: wrong subtraction amounts, mutated preserved fields, wrong endpoint states
- **Rejects unintended reasoning**: false disjointness violations, overstrong rf_counter bounds, false non-determinism claims

No specification weaknesses were detected in these 15 adversarial tests.
