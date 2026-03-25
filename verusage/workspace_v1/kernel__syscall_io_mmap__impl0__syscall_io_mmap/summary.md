# Adversarial Proof Test Summary: `syscall_io_mmap`

## Target
`kernel__syscall_io_mmap__impl0__syscall_io_mmap.rs` — Implements IO memory mapping syscall for the Atmosphere verified kernel.

## Results Overview

| Test File | Total Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 7 | 7 | 0 |
| `behavioral_mutation_tests.rs` | 7 | 7 | 0 |
| `logical_tests.rs` | 7 | 7 | 0 |
| **Total** | **21** | **21** | **0** |

All 21 adversarial tests correctly **fail verification**, indicating the specification rejects all tested invalid inputs, incorrect behaviors, and unintended logical claims.

---

## Boundary Tests (7/7 FAILED ✓)

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_boundary_va_zero_is_valid` | VA=0 is in kernel space (L4 index < KERNEL_MEM_END_L4INDEX) | FAILED ✓ |
| 2 | `test_boundary_unaligned_page_ptr` | Pointer 1 is not 4K-aligned | FAILED ✓ |
| 3 | `test_boundary_page_index_at_limit` | Index = NUM_PAGES is out of range | FAILED ✓ |
| 4 | `test_boundary_ptr_roundtrip_unaligned` | ptr→index→ptr truncates unaligned addresses | FAILED ✓ |
| 5 | `test_boundary_va_kernel_space` | VA 0x1000 is in kernel space | FAILED ✓ |
| 6 | `test_boundary_index2ptr_out_of_bounds` | index2ptr(NUM_PAGES) exceeds valid range | FAILED ✓ |
| 7 | `test_boundary_usize2entry_nonzero_is_empty` | Entry with present bit set is not empty | FAILED ✓ |

**Conclusion**: Boundary conditions on page pointers, indices, and virtual addresses are correctly enforced by the spec.

---

## Behavioral Mutation Tests (7/7 FAILED ✓)

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_zero_entry_present` | Assert `present=true` for zero entry (should be false) | FAILED ✓ |
| 2 | `test_mutation_zero_perm_write` | Assert `write=true` for zero perm (should be false) | FAILED ✓ |
| 3 | `test_mutation_usize2pa_identity` | Assert `usize2pa(0xF) == 0xF` (mask strips low bits) | FAILED ✓ |
| 4 | `test_mutation_noswitch_returns_switch` | Assert `NoSwitch == Switch` | FAILED ✓ |
| 5 | `test_mutation_2m_valid_for_unaligned` | Assert index 1 is 2MB-valid (requires 512 alignment) | FAILED ✓ |
| 6 | `test_mutation_2m_ptr_for_4k_aligned` | Assert 0x1000 is 2MB-aligned (requires 0x200000 alignment) | FAILED ✓ |
| 7 | `test_mutation_fold_lemma_zero_sum` | Assert quota sum = 0 when a container has positive quota | FAILED ✓ |

**Conclusion**: The spec correctly rejects mutated output values and inverted boolean fields. The `fold_mem_4k_lemma` correctly constrains the quota sum.

---

## Logical Tests (7/7 FAILED ✓)

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_wf_implies_total_quota` | `wf()` alone implies `total_mem_4k_quota_wf()` | FAILED ✓ |
| 2 | `test_logical_varange_view_determinism` | Two VaRange4K with same start/len have same view (closed spec fn) | FAILED ✓ |
| 3 | `test_logical_io_space_range_free_implies_empty` | Range-free implies entire IO space is empty | FAILED ✓ |
| 4 | `test_logical_disjoint_closure_implies_disjoint_mapping` | Disjoint page closures implies disjoint VA mappings | FAILED ✓ |
| 5 | `test_logical_wf_implies_free_pages` | `wf()` implies at least 1 free page | FAILED ✓ |
| 6 | `test_logical_ioid_implies_nonempty_io_space` | Having an ioid implies non-empty IO space | FAILED ✓ |
| 7 | `test_logical_fold_strict_inequality` | Free pages **strictly** greater than any quota (only ≥ guaranteed) | FAILED ✓ |

**Conclusion**: The spec correctly refuses to entail overly strong logical claims. Key findings:
- `wf()` and `total_mem_4k_quota_wf()` are independent — the spec correctly separates them.
- Closed spec fns (`VaRange4K::view`) are opaque — determinism is not provable.
- Range-freedom is local (per VA range), not global — correctly scoped.
- Page closure disjointness does NOT imply VA mapping disjointness — isolation is at the physical level, not virtual.
- The spec makes no minimum-resources guarantees.

---

## Notable Observations

1. **`syscall_io_mmap` has no `ensures` clause** — the function's return value and post-state are unspecified. This is a potential spec gap: callers cannot reason about the outcome.

2. **`range_alloc_and_map_io` only ensures `self.wf()`**, not `self.total_wf()` — the quota-tracking invariant (`total_mem_4k_quota_wf`) may be broken after IO mapping. This is the most significant spec weakness discovered.

3. **`VaRange4K::view` is a closed spec fn** — its relationship to `start` and `len` is opaque, which prevents structural reasoning about VA ranges at the proof level.

4. **During initial testing**, `pcid_to_proc_ptr` injectivity was provable from the spec (via `pcid_ioid_wf`), confirming the spec IS strong enough for pcid↔proc bijection reasoning.
