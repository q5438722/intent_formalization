use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
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

impl Quota {
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, stronger bounds,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: create_entry_and_share ensures ret <= 3, but NOT ret > 0.
// The spec does not guarantee that any pages are actually consumed.
// Claim: ret is always >= 1.
// SHOULD FAIL
proof fn test_logical_ret_always_positive(ret: usize)
    requires
        ret <= 3,
{
    assert(ret >= 1);
}

// Test 2: create_entry_and_share ensures ret <= 3.
// The spec does NOT guarantee ret is always exactly 3.
// Claim: ret is always exactly 3.
// SHOULD FAIL
proof fn test_logical_ret_always_exactly_three(ret: usize)
    requires
        ret <= 3,
{
    assert(ret == 3);
}

// Test 3: create_entry_and_share is NOT deterministic — the same
// inputs do not guarantee the same return value.
// Claim: two calls with identical states produce identical ret.
// SHOULD FAIL
proof fn test_logical_determinism(
    ret1: usize,
    ret2: usize,
)
    requires
        ret1 <= 3,
        ret2 <= 3,
{
    assert(ret1 == ret2);
}

// Test 4: The spec preserves containers for non-owning containers,
// but does NOT explicitly guarantee the owning container's
// `root_process` field is unchanged. Claim: root_process
// is preserved.
// SHOULD FAIL
proof fn test_logical_root_process_preserved(
    old_root: Option<ProcPtr>,
    new_root: Option<ProcPtr>,
)
    requires
        old_root.is_Some(),
{
    assert(old_root == new_root);
}

// Test 5: The spec says src_proc and target_proc can be different,
// but the implementation asserts src != target || src_va != target_va.
// The spec does NOT explicitly forbid src_proc == target_proc.
// However, the combined preconditions (src_va mapped, target_va unmapped)
// mean src == target implies src_va != target_va.
// Claim: src_proc_ptr != target_proc_ptr always.
// SHOULD FAIL
proof fn test_logical_src_target_always_different(
    src_proc_ptr: ProcPtr,
    target_proc_ptr: ProcPtr,
    src_va: VAddr,
    target_va: VAddr,
    src_mapped: bool,
    target_mapped: bool,
)
    requires
        src_mapped == true,
        target_mapped == false,
{
    assert(src_proc_ptr != target_proc_ptr);
}

// Test 6: The spec does NOT guarantee that target_va and src_va
// are always different. They could be the same VA in different
// address spaces. Claim: they are always different.
// SHOULD FAIL
proof fn test_logical_va_always_different(
    src_va: VAddr,
    target_va: VAddr,
)
    requires
        spec_va_4k_valid(src_va),
        spec_va_4k_valid(target_va),
{
    assert(src_va != target_va);
}

// Test 7: The spec preserves quota.mem_2m (via spec_subtract_mem_4k).
// But the spec does NOT guarantee quota.mem_4k > 0 after the call.
// Claim: quota.mem_4k is always > 0 after.
// SHOULD FAIL
proof fn test_logical_quota_positive_after(
    old_quota: Quota,
    new_quota: Quota,
    ret: usize,
)
    requires
        old_quota.spec_subtract_mem_4k(new_quota, ret),
        old_quota.mem_4k >= 3,
        ret <= 3,
{
    assert(new_quota.mem_4k > 0);
}

// Test 8: The spec says page_mapping domain is unchanged.
// It does NOT guarantee the page_io_mapping is also unchanged.
// Claim: page_io_mapping domain is also guaranteed unchanged.
// SHOULD FAIL
proof fn test_logical_io_mapping_domain_unchanged(
    old_io_dom: Set<PagePtr>,
    new_io_dom: Set<PagePtr>,
)
    requires
        old_io_dom.len() > 0,
{
    assert(old_io_dom =~= new_io_dom);
}

// Test 9: The spec says page_is_mapped status is preserved for
// all pages. It does NOT say the total number of mapped pages
// is preserved. Claim: total mapped page count is unchanged.
// (This is actually implied, but testing an explicit count form.)
// SHOULD FAIL
proof fn test_logical_explicit_mapped_count_unchanged(
    old_count: nat,
    new_count: nat,
)
    requires
        old_count > 0,
{
    assert(old_count == new_count);
}

// Test 10: The spec preserves container fields for the owning
// container (owned_procs, children, etc.) but does NOT explicitly
// preserve `can_have_children`. Claim: it is preserved.
// SHOULD FAIL
proof fn test_logical_can_have_children_preserved(
    old_val: bool,
    new_val: bool,
)
    requires
        old_val == true,
{
    assert(old_val == new_val);
}

// Test 11: va_4k_valid does NOT imply page_ptr_valid. These are
// different validity domains (virtual vs physical).
// SHOULD FAIL
proof fn test_logical_va_valid_implies_page_ptr_valid(va: VAddr)
    requires
        spec_va_4k_valid(va),
{
    assert(page_ptr_valid(va));
}

// Test 12: The spec says forall mapped pages p != src_entry.addr,
// their page_mapping is unchanged. It does NOT say that the
// src_entry.addr's mapping ONLY has one new element. The set
// could have had prior entries. Claim: after the call, the
// mapping for src page has exactly 2 entries.
// SHOULD FAIL
proof fn test_logical_src_page_mapping_size_exactly_two(
    old_mapping: Set<(ProcPtr, VAddr)>,
    new_mapping: Set<(ProcPtr, VAddr)>,
    target_proc_ptr: ProcPtr,
    target_va: VAddr,
)
    requires
        new_mapping == old_mapping.insert((target_proc_ptr, target_va)),
        old_mapping.len() > 0,
{
    assert(new_mapping.len() == 2);
}

}
