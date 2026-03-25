use vstd::prelude::*;

fn main() {}

verus!{

pub type IOid = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type EndpointPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    SuccessSeqUsize { value: Ghost<Seq<usize>> },
    SuccessPairUsize { value1: usize, value2: usize },
    SuccessThreeUsize { value1: usize, value2: usize, value3: usize },
    ErrorNoQuota,
    ErrorVaInUse,
    CpuIdle,
    Error,
    Else,
    NoQuota,
    VaInUse,
}

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

impl SyscallReturnStruct {

    #[verifier::external_body]
    pub fn NoSwitchNew(error_code: RetValueType) -> (ret: Self)
        ensures
            ret.error_code == error_code,
            ret.pcid.is_None(),
            ret.cr3.is_None(),
            ret.switch_decision == SwitchDecision::NoSwitch,
    {
        unimplemented!()
    }

}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These probe for determinism, stronger inequalities, injectivity,
// structural assumptions, and cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: syscall_io_mmap has NO ensures clause. Therefore we cannot
// prove that the return is always NoSwitch. The spec does not guarantee
// the switch_decision of the overall syscall.
// SHOULD FAIL
proof fn test_logical_syscall_no_ensures_switch() {
    // Simulate: we only know the function was called with valid args.
    // Without ensures, we cannot derive the return's switch_decision.
    let ret_switch: SwitchDecision = arbitrary();
    assume(true); // No postcondition to work from
    assert(ret_switch == SwitchDecision::NoSwitch);
}

// Test 2: syscall_io_mmap has NO ensures. We cannot prove that when
// the process has no ioid, the returned error_code is Error.
// Even though the code does this, the spec doesn't guarantee it.
// SHOULD FAIL
proof fn test_logical_syscall_no_ensures_error_code() {
    let ioid_op: Option<IOid> = None;
    let ret_error: RetValueType = arbitrary();
    assume(ioid_op.is_None());
    // The spec gives no postcondition on the return value
    assert(ret_error == RetValueType::Error);
}

// Test 3: page_ptr2page_index does NOT ensure the result < NUM_PAGES.
// The spec only says ret == ptr / 4096, but does not bound the result.
// SHOULD FAIL
proof fn test_logical_page_ptr2index_bounded(ptr: usize) {
    assume(ptr % 0x1000 == 0);
    // spec: ret = ptr / 4096; no upper-bound guarantee
    let ret = spec_page_ptr2page_index(ptr);
    assert(ret < NUM_PAGES);
}

// Test 4: page_index2page_ptr does NOT ensure the result is page_ptr_valid.
// The ensures only says ret == i * 4096, not that the result is aligned
// AND within bounds (page_ptr_valid checks both).
// SHOULD FAIL
proof fn test_logical_page_index2ptr_valid(i: usize) {
    assume(0 <= i && i < NUM_PAGES);
    let ret = spec_page_index2page_ptr(i);
    // page_ptr_valid requires: ptr % 0x1000 == 0 && ptr / 0x1000 < NUM_PAGES
    // The spec doesn't guarantee the result is < NUM_PAGES * 4096 (within usize)
    assert(page_ptr_valid(ret));
}

// Test 5: spec_page_ptr2page_index is NOT guaranteed to be injective
// in the general case. Two different valid pointers could theoretically
// yield the same index if the spec were weaker. Here we try to prove
// injectivity for arbitrary (non-page-aligned) values.
// SHOULD FAIL
proof fn test_logical_page_ptr2index_injective(ptr1: usize, ptr2: usize) {
    assume(ptr1 != ptr2);
    // Without requiring alignment, different ptrs may map to same index
    assert(spec_page_ptr2page_index(ptr1) != spec_page_ptr2page_index(ptr2));
}

// Test 6: range_alloc_and_map_io's ensures only guarantees self.wf().
// It does NOT guarantee that the returned number of pages equals va_range.len.
// SHOULD FAIL
proof fn test_logical_range_alloc_num_pages(num_page: usize, va_range_len: usize) {
    assume(va_range_len > 0);
    // The spec does NOT guarantee num_page == va_range_len
    // It only ensures self.wf() after the call
    assert(num_page == va_range_len);
}

// Test 7: syscall_io_mmap has NO ensures clause.
// We cannot prove that the kernel state remains well-formed after the call.
// SHOULD FAIL
proof fn test_logical_syscall_preserves_wf(pre_wf: bool, post_wf: bool) {
    assume(pre_wf); // Assume the kernel was wf before
    // No ensures on syscall_io_mmap, so post_wf is unknown
    assert(post_wf);
}

// Test 8: The negation of io_space_range_free does NOT guarantee
// that a SPECIFIC (known) index is the violating one.
// Try to assert a concrete index is the violating index.
// SHOULD FAIL
proof fn test_logical_io_space_violation_at_specific_index(
    io_space_dom: Set<VAddr>,
    va_range: Seq<VAddr>,
    va_range_len: int,
) {
    assume(va_range.len() == va_range_len);
    assume(va_range_len > 1);
    // Assume range is NOT free (some entry collides)
    assume(exists|j: int| 0 <= j < va_range_len && io_space_dom.contains(va_range[j]));
    // But the spec does NOT tell us WHICH index. Asserting index 0 is the one
    // that collides is unwarranted.
    assert(io_space_dom.contains(va_range[0]));
}

// Test 9: NoSwitchNew's ensures constrain all fields, so structural
// equality holds. Instead, test whether the spec guarantees that
// NoSwitchNew with DIFFERENT error_codes produces DIFFERENT results.
// The spec doesn't guarantee this (error_code field IS different,
// but =~= on enums with Ghost fields may not be derivable).
// SHOULD FAIL
proof fn test_logical_noswitch_different_inputs_different_results(
    ret1: SyscallReturnStruct,
    ret2: SyscallReturnStruct,
    e1: RetValueType,
    e2: RetValueType,
) {
    assume(ret1.error_code == e1);
    assume(ret2.error_code == e2);
    assume(ret1.pcid.is_None() && ret2.pcid.is_None());
    assume(ret1.cr3.is_None() && ret2.cr3.is_None());
    assume(ret1.switch_decision == SwitchDecision::NoSwitch);
    assume(ret2.switch_decision == SwitchDecision::NoSwitch);
    // Even with different error codes, spec doesn't let us prove ret1 != ret2
    // because RetValueType variants like SuccessSeqUsize contain Ghost fields
    // and equality on Ghost is opaque.
    assert(!(ret1 =~= ret2));
}

// Test 10: The quota check in syscall_io_mmap uses
// quota.mem_4k < va_range.len * 4. This does NOT guarantee that
// quota.mem_4k >= 4 implies at least 1 page can be mapped.
// (Since len could be > 1.)
// SHOULD FAIL
proof fn test_logical_quota_one_page_sufficient() {
    let quota_mem_4k: usize = 4;
    let va_range_len: usize = arbitrary();
    assume(va_range_len > 0);
    // Cannot conclude that quota is sufficient for arbitrary len
    assert(quota_mem_4k >= va_range_len * 4);
}

}
