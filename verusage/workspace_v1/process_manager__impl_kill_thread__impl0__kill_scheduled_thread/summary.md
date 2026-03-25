# Adversarial Proof Tests: `kill_scheduled_thread`

## Target
`ProcessManager::kill_scheduled_thread` — removes a SCHEDULED thread from the process manager, returning its page and permission.

## Results Summary

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (23 verified, 5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (23 verified, 5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (23 verified, 5 errors) |

**All 15 tests correctly fail verification**, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (precondition violations)

| Test | Property Violated | Result |
|------|-------------------|--------|
| B1 | `thread_ptr` not in `thread_dom` | ✅ FAIL |
| B2 | Thread state is RUNNING (not SCHEDULED) | ✅ FAIL |
| B3 | Thread state is BLOCKED (not SCHEDULED) | ✅ FAIL |
| B4 | Endpoint descriptor at index 0 is `Some` | ✅ FAIL |
| B5 | Edge value `thread_ptr = 0` not in domain | ✅ FAIL |

## Behavioral Mutation Tests (incorrect postconditions)

| Test | Mutated Postcondition | Result |
|------|----------------------|--------|
| M1 | Thread still in `thread_dom` after kill | ✅ FAIL |
| M2 | `page_closure` unchanged after kill | ✅ FAIL |
| M3 | `owned_threads.len()` unchanged | ✅ FAIL |
| M4 | `proc_dom` lost a process | ✅ FAIL |
| M5 | `container_dom` changed after kill | ✅ FAIL |

## Logical Tests (unintended reasoning)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| L1 | Returned page ptr is always 0 | ✅ FAIL |
| L2 | `proc_dom` is always non-empty | ✅ FAIL |
| L3 | All threads share the same owning process | ✅ FAIL |
| L4 | Killing a thread changes other threads' state | ✅ FAIL |
| L5 | Returned page is a container page | ✅ FAIL |

## Conclusion

The specification for `kill_scheduled_thread` is **consistent** with respect to the tested properties. It correctly:
- Rejects all invalid inputs (boundary violations)
- Rejects all incorrect behavioral mutations
- Does not entail any of the tested unintended logical properties
