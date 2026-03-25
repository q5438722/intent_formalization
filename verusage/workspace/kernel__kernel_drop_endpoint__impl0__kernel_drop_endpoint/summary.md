# Test Summary: `kernel_drop_endpoint`

## Target
`kernel__kernel_drop_endpoint__impl0__kernel_drop_endpoint.rs`  
Functions tested: `kernel_drop_endpoint` (Kernel), `drop_endpoint` (ProcessManager)

---

## Results Overview

| Test File | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 6 | 6 | 0 |
| `behavioral_mutation_tests.rs` | 8 | 8 | 0 |
| `logical_tests.rs` | 7 | 7 | 0 |
| **Total** | **21** | **21** | **0** |

**All 21 adversarial tests FAILED verification as expected.** No spec weaknesses detected.

---

## Boundary Tests (6/6 failed ✅)

| # | Test | Violated Precondition |
|---|---|---|
| 1 | `test_boundary_edp_idx_at_max` | `edp_idx == 128` (must be < 128) |
| 2 | `test_boundary_edp_idx_usize_max` | `edp_idx == usize::MAX` |
| 3 | `test_boundary_thread_not_in_domain` | Thread not in `thread_dom()` |
| 4 | `test_boundary_blocked_thread_drops_blocking_endpoint` | Blocked thread drops its own blocking endpoint |
| 5 | `test_boundary_kernel_not_wf` | Kernel not well-formed |
| 6 | `test_boundary_edp_idx_negative_wrap` | `edp_idx == 200` (out of range) |

## Behavioral Mutation Tests (8/8 failed ✅)

| # | Test | Mutated Property |
|---|---|---|
| 1 | `test_behavioral_descriptor_not_cleared` | Claim descriptor NOT cleared at `edp_idx` |
| 2 | `test_behavioral_thread_state_changed` | Claim thread state changed |
| 3 | `test_behavioral_other_thread_changed` | Claim other thread was modified |
| 4 | `test_behavioral_proc_dom_shrank` | Claim process domain shrank |
| 5 | `test_behavioral_container_dom_changed` | Claim container domain changed |
| 6 | `test_behavioral_blocking_index_changed` | Claim blocking index changed |
| 7 | `test_behavioral_owning_proc_changed` | Claim owning_proc changed |
| 8 | `test_behavioral_process_container_changed` | Claim process container changed |

## Logical Tests (7/7 failed ✅)

| # | Test | Unintended Property |
|---|---|---|
| 1 | `test_logical_endpoint_dom_shrinks` | Endpoint domain shrinks after drop |
| 2 | `test_logical_other_descriptor_also_cleared` | Another descriptor also cleared |
| 3 | `test_logical_idempotent_drop` | Dropping None descriptor is a no-op on page_closure |
| 4 | `test_logical_determinism` | Two calls return identical pages |
| 5 | `test_logical_thread_dom_min_size` | Thread domain has ≥ 2 elements |
| 6 | `test_logical_all_containers_fully_identical` | All container fields unchanged |
| 7 | `test_logical_idx_zero_always_none` | Index 0 always returns None |

---

## Conclusion

The specification for `kernel_drop_endpoint` correctly rejects all 21 adversarial queries across boundary violations, behavioral mutations, and logical overreach. The spec appears consistent with respect to the tested semantic boundaries.
