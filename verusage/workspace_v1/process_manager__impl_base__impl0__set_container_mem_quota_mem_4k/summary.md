# Test Execution Summary: `set_container_mem_quota_mem_4k`

## Target Function
`ProcessManager::set_container_mem_quota_mem_4k(&mut self, container_ptr: ContainerPtr, new_quota: usize)`

Sets the `mem_4k` field of a container's `Quota` via `spec_set_mem_4k(new_quota)`, preserving all other state.

**Preconditions**: `old(self).wf()`, `old(self).container_dom().contains(container_ptr)`

**Postconditions**: Preserves `wf()`, all domains unchanged, all entities except target container unchanged, target container only has `quota.mem_4k` modified.

---

## Results: All 18 tests FAIL verification ✅

### Boundary Tests (6/6 FAIL) — `boundary_tests.rs`
| Test | Description | Result |
|------|-------------|--------|
| B1 | `container_ptr` not in domain → can't derive `is_init()` | FAIL ✅ |
| B2 | `!pm.wf()` → can't derive `schedulers_wf()` | FAIL ✅ |
| B3 | `container_ptr` not in domain → can't derive `addr == container_ptr` | FAIL ✅ |
| B4 | Zero ptr not in domain → can't access fields | FAIL ✅ |
| B5 | Empty container domain → can't find any container | FAIL ✅ |
| B6 | `new_quota = usize::MAX` → can't prove `quota < usize::MAX` | FAIL ✅ |

### Behavioral Mutation Tests (6/6 FAIL) — `behavioral_mutation_tests.rs`
| Test | Description | Result |
|------|-------------|--------|
| M1 | Quota unchanged after operation (contradicts postcondition) | FAIL ✅ |
| M2 | `owned_procs` of target changed (should be preserved) | FAIL ✅ |
| M3 | Non-target container modified (should be preserved) | FAIL ✅ |
| M4 | `proc_dom` changed (should be preserved) | FAIL ✅ |
| M5 | A thread modified (should be preserved) | FAIL ✅ |
| M6 | Scheduler of target changed (should be preserved) | FAIL ✅ |

### Logical Tests (6/6 FAIL) — `logical_tests.rs`
| Test | Description | Result |
|------|-------------|--------|
| L1 | `can_have_children` preserved for target (NOT in postcondition) | FAIL ✅ |
| L2 | `quota.mem_2m` also changed (only `mem_4k` should change) | FAIL ✅ |
| L3 | `new_quota` must be positive (spec allows 0) | FAIL ✅ |
| L4 | `root_container` preserved (NOT in postcondition) | FAIL ✅ |
| L5 | `cpu_list` preserved (NOT in postcondition) | FAIL ✅ |
| L6 | Same-value set is a no-op (entire container unchanged) | FAIL ✅ |

---

## Observations

### Spec Weaknesses Identified (Logical Tests)
The following properties are **not explicitly guaranteed** by the postcondition, meaning the spec is potentially **incomplete**:

1. **`can_have_children` not preserved (L1, L6)**: The postcondition lists many preserved fields of the target container but omits `can_have_children`. A caller cannot prove this field is unchanged after the operation.

2. **`root_container` not preserved (L4)**: The `root_container` field of `ProcessManager` is not mentioned in the postcondition. Callers cannot prove it remains the same.

3. **`cpu_list` not preserved (L5)**: The `cpu_list` field of `ProcessManager` is not mentioned in the postcondition. Callers cannot prove CPU assignments are unchanged.

> Note: While the **implementation** likely preserves these fields (and `wf()` may implicitly constrain some), the **specification** does not expose these guarantees to callers. This represents a gap between implementation behavior and specification coverage.

### Spec Strengths Confirmed
- All boundary tests correctly rejected invalid inputs (B1-B6)
- All behavioral mutations correctly rejected incorrect outputs (M1-M6)
- The spec correctly constrains `mem_4k` update via `spec_set_mem_4k` and preserves all explicitly listed fields
