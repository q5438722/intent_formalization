use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2ps(v: usize) -> bool {
    (v & PAGE_ENTRY_PS_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

pub open spec fn usize2execute_disable(v: usize) -> bool {
    (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0
}

pub open spec fn usize2user(v: usize) -> bool {
    (v & PAGE_ENTRY_USER_MASK as usize) != 0
}

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
    }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

#[derive(Clone, Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone, Debug)]
pub struct PageEntry {
    pub addr: PAddr,
    pub perm: PageEntryPerm,
}

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs, then asserts a WRONG output
// or relation (mutated from the correct one).
// All tests SHOULD FAIL verification.

// Test 1: usize2page_entry(0) should produce an empty entry.
// Mutate: claim present == true when it should be false.
// SHOULD FAIL
proof fn test_mutation_zero_entry_present() {
    let entry = spec_usize2page_entry(0usize);
    assert(entry.perm.present == true);
}

// Test 2: usize2page_entry(0) should have addr == 0.
// Mutate: claim addr != 0.
// SHOULD FAIL
proof fn test_mutation_zero_entry_nonzero_addr() {
    let entry = spec_usize2page_entry(0usize);
    assert(entry.addr != 0);
}

// Test 3: usize2page_entry_perm(0) should give all-false permissions.
// Mutate: claim write == true.
// SHOULD FAIL
proof fn test_mutation_zero_perm_write() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.write == true);
}

// Test 4: usize2page_entry_perm(0) should have execute_disable == false.
// Mutate: claim execute_disable == true.
// SHOULD FAIL
proof fn test_mutation_zero_perm_execute_disable() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.execute_disable == true);
}

// Test 5: usize2page_entry_perm(0) should have user == false.
// Mutate: claim user == true.
// SHOULD FAIL
proof fn test_mutation_zero_perm_user() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.user == true);
}

// Test 6: create_entry_and_alloc_and_map ensures ret.0 <= 4.
// Mutate: claim ret.0 > 4.
// SHOULD FAIL
proof fn test_mutation_create_entry_uses_more_than_4_pages(ret0: usize)
    requires
        ret0 <= 4,
{
    assert(ret0 > 4);
}

// Test 7: Quota::spec_subtract_mem_4k subtracts from mem_4k, leaves others unchanged.
// Mutate: claim mem_2m also changes.
// SHOULD FAIL
proof fn test_mutation_quota_subtract_changes_mem_2m() {
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 96, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 4));
}

// Test 8: range_alloc_and_map ensures address space for non-target procs is preserved.
// Mutate: claim address spaces for other procs can differ.
// SHOULD FAIL
proof fn test_mutation_other_proc_addr_space_changes(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    va: VAddr,
    entry: MapEntry,
)
    requires
        old_space =~= new_space,
        old_space.dom().contains(va),
{
    assert(old_space[va].addr != new_space[va].addr);
}

// Test 9: page_ptr2page_index and page_index2page_ptr are inverses.
// Mutate: claim they don't compose to identity.
// SHOULD FAIL
proof fn test_mutation_ptr_index_roundtrip_wrong() {
    let i: usize = 100;
    assert(spec_page_ptr2page_index(spec_page_index2page_ptr(i)) != i);
}

// Test 10: usize2pa always returns MEM_valid result.
// Mutate: claim that for v=0, the result is NOT MEM_valid.
// SHOULD FAIL
proof fn test_mutation_usize2pa_not_mem_valid() {
    let result = spec_usize2pa(0usize);
    assert(!MEM_valid(result));
}

// Test 11: range_alloc_and_map ensures all VAs in range are in the
// resulting address space. Mutate: claim some VA is NOT in the space.
// SHOULD FAIL
proof fn test_mutation_mapped_va_not_in_space(
    va_set: Set<VAddr>,
    va: VAddr,
)
    requires
        va_set.contains(va),
{
    assert(!va_set.contains(va));
}

// Test 12: range_alloc_and_map returns page_diff sequence same length as va_range.
// Mutate: claim lengths differ.
// SHOULD FAIL
proof fn test_mutation_page_diff_length_mismatch(
    va_range_len: usize,
    page_diff_len: int,
)
    requires
        page_diff_len == va_range_len,
{
    assert(page_diff_len != va_range_len);
}

}
