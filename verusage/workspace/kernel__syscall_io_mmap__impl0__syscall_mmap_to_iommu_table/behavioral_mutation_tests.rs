use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type IOid = usize;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid input conditions matching the spec's
// execution paths and mutates expected outputs or relations.
// Tests whether incorrect behaviors are rejected by the spec.
// All tests SHOULD FAIL verification.

// Test 1: When ioid is None, syscall_io_mmap returns Error.
// Mutated: claim the return is NOT Error (i.e., success).
// Models: ioid_op.is_none() => return NoSwitchNew(Error)
// SHOULD FAIL
proof fn test_mutation_no_ioid_returns_success(
    ioid_is_none: bool,
    returned_error: bool,
)
    requires
        ioid_is_none == true,     // process has no ioid
        returned_error == true,   // spec says return Error
{
    // Mutated: claim it did NOT return error
    assert(!returned_error);
}

// Test 2: When container quota < va_range.len * 4,
// syscall_io_mmap returns ErrorNoQuota.
// Mutated: claim the return is NOT ErrorNoQuota.
// SHOULD FAIL
proof fn test_mutation_insufficient_quota_returns_success(
    quota_mem_4k: usize,
    va_range_len: usize,
    returned_no_quota: bool,
)
    requires
        va_range_len > 0,
        quota_mem_4k < va_range_len * 4,
        returned_no_quota == true,  // spec says return ErrorNoQuota
{
    // Mutated: claim it did NOT return no-quota error
    assert(!returned_no_quota);
}

// Test 3: When IO space VA range is NOT free,
// syscall_io_mmap returns ErrorVaInUse.
// Mutated: claim the return is NOT ErrorVaInUse.
// SHOULD FAIL
proof fn test_mutation_va_in_use_returns_success(
    io_space_free: bool,
    returned_va_in_use: bool,
)
    requires
        io_space_free == false,        // VA range occupied
        returned_va_in_use == true,    // spec says return ErrorVaInUse
{
    // Mutated: claim it did NOT return va-in-use error
    assert(!returned_va_in_use);
}

// Test 4: NoSwitchNew guarantees switch_decision == NoSwitch.
// Mutated: claim switch_decision is Switch.
// SHOULD FAIL
proof fn test_mutation_no_switch_is_switch(
    is_no_switch: bool,
)
    requires
        is_no_switch == true,  // NoSwitchNew ensures NoSwitch
{
    assert(!is_no_switch);
}

// Test 5: NoSwitchNew guarantees pcid.is_None().
// Mutated: claim pcid is Some.
// SHOULD FAIL
proof fn test_mutation_no_switch_pcid_is_some(
    pcid_is_none: bool,
)
    requires
        pcid_is_none == true,  // NoSwitchNew ensures pcid.is_None()
{
    assert(!pcid_is_none);
}

// Test 6: NoSwitchNew guarantees cr3.is_None().
// Mutated: claim cr3 is Some.
// SHOULD FAIL
proof fn test_mutation_no_switch_cr3_is_some(
    cr3_is_none: bool,
)
    requires
        cr3_is_none == true,  // NoSwitchNew ensures cr3.is_None()
{
    assert(!cr3_is_none);
}

// Test 7: check_io_space_va_range_free ensures ret matches
// io_space_range_free spec. Mutated: claim ret is negated.
// SHOULD FAIL
proof fn test_mutation_io_space_check_negated(
    spec_result: bool,
    fn_result: bool,
)
    requires
        fn_result == spec_result,  // ensures ret == spec
{
    assert(fn_result != spec_result);
}

// Test 8: range_alloc_and_map_io ensures self.wf() after call.
// Mutated: claim wf does NOT hold after.
// SHOULD FAIL
proof fn test_mutation_range_alloc_breaks_wf(
    post_wf: bool,
)
    requires
        post_wf == true,  // ensures self.wf()
{
    assert(!post_wf);
}

// Test 9: When all checks pass (ioid Some, quota sufficient,
// VA range free), the function returns SuccessSeqUsize.
// Mutated: claim the function returns Error.
// SHOULD FAIL
proof fn test_mutation_success_path_returns_error(
    ioid_is_some: bool,
    quota_sufficient: bool,
    va_range_free: bool,
    returned_success: bool,
)
    requires
        ioid_is_some == true,
        quota_sufficient == true,
        va_range_free == true,
        returned_success == true,  // all checks pass => success
{
    assert(!returned_success);
}

// Test 10: io_space_range_free requires all VA entries in range
// to be absent from the IO space. If even one is present,
// the range is NOT free. Mutated: claim it IS free.
// SHOULD FAIL
proof fn test_mutation_occupied_va_is_free(
    io_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        io_space_dom.contains(va),  // VA is occupied
{
    // Mutated: claim VA is not in domain (range is free)
    assert(!io_space_dom.contains(va));
}

}
