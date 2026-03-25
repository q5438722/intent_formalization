use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_v2l4index(va: usize) -> L4Index {
    (va >> 39 & 0x1ff) as usize
}

pub open spec fn spec_v2l3index(va: usize) -> L3Index {
    (va >> 30 & 0x1ff) as usize
}

pub open spec fn spec_v2l2index(va: usize) -> L2Index {
    (va >> 21 & 0x1ff) as usize
}

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

pub open spec fn spec_va2index(va: usize) -> (L4Index, L3Index, L2Index, L1Index) {
    (spec_v2l4index(va), spec_v2l3index(va), spec_v2l2index(va), spec_v2l1index(va))
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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests probe for determinism, stronger inequalities, structural
// assumptions, and cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: Spec guarantees ret.0 <= 3 but NOT ret.0 >= 1.
// Could be 0 if all L4/L3/L2 entries already exist.
// Claim: at least 1 page is always allocated.
// SHOULD FAIL
proof fn test_logical_always_allocates_at_least_one(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 >= 1);
}

// Test 2: Spec guarantees ret.0 <= 3 but NOT ret.0 == 3.
// Claim: exactly 3 pages are always allocated.
// SHOULD FAIL
proof fn test_logical_always_allocates_exactly_three(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 == 3);
}

// Test 3: Spec does NOT guarantee determinism.
// Two calls with same constraints could return different ret.0.
// Claim: same ret.0 from two calls.
// SHOULD FAIL
proof fn test_logical_determinism(ret0_a: usize, ret0_b: usize)
    requires
        ret0_a <= 3,
        ret0_b <= 3,
{
    assert(ret0_a == ret0_b);
}

// Test 4: Spec says ret.0 <= 3 and free_pages decrease by ret.0.
// Claim: free pages always decrease by exactly 3.
// SHOULD FAIL
proof fn test_logical_free_pages_always_decrease_by_3(
    old_free: usize,
    new_free: usize,
    ret0: usize,
)
    requires
        ret0 <= 3,
        old_free >= 3,
        new_free == old_free - ret0,
{
    assert(new_free == old_free - 3);
}

// Test 5: Spec does NOT guarantee ret.0 is strictly less than 3.
// Claim: ret.0 < 3 always (i.e., never allocates 3).
// SHOULD FAIL
proof fn test_logical_ret_strictly_less_than_3(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 < 3);
}

// Test 6: Spec preserves IO space for all procs but does NOT say
// the target VA gets added to IO space (this fn only creates
// intermediate table levels, not the final mapping).
// Claim: after create_iommu_table_entry, VA is in IO space.
// SHOULD FAIL
proof fn test_logical_va_added_to_io_space(
    old_io_dom: Set<VAddr>,
    new_io_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        !old_io_dom.contains(va),
        new_io_dom =~= old_io_dom,
{
    assert(new_io_dom.contains(va));
}

// Test 7: Spec preserves container owned_pages for ALL containers.
// Does NOT guarantee owned_pages grows (pages go to page tables, not ownership).
// Claim: some new page was added to container's owned pages.
// SHOULD FAIL
proof fn test_logical_owned_pages_grow(
    old_owned: Set<PagePtr>,
    new_owned: Set<PagePtr>,
    new_page: PagePtr,
)
    requires
        new_owned =~= old_owned,
        !old_owned.contains(new_page),
{
    assert(new_owned.contains(new_page));
}

// Test 8: Quota subtraction: spec says old_quota.mem_4k - ret.0 == new_quota.mem_4k.
// Claim: new quota is always strictly less than old quota (ret.0 > 0).
// But ret.0 could be 0.
// SHOULD FAIL
proof fn test_logical_quota_always_decreases(
    old_quota: Quota,
    new_quota: Quota,
    ret0: usize,
)
    requires
        ret0 <= 3,
        old_quota.mem_4k >= 3,
        old_quota.spec_subtract_mem_4k(new_quota, ret0),
{
    assert(new_quota.mem_4k < old_quota.mem_4k);
}

// Test 9: Two different valid VAs do NOT necessarily produce
// different returned page map pointers (ret.1).
// Claim: different VAs always yield different ret.1.
// SHOULD FAIL
proof fn test_logical_different_va_different_ret1(
    ret1_a: PageMapPtr,
    ret1_b: PageMapPtr,
    va_a: VAddr,
    va_b: VAddr,
)
    requires
        spec_va_4k_valid(va_a),
        spec_va_4k_valid(va_b),
        va_a != va_b,
{
    assert(ret1_a != ret1_b);
}

// Test 10: Spec does NOT guarantee ret.0 is even.
// Claim: ret.0 is always even.
// SHOULD FAIL
proof fn test_logical_ret_is_even(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 % 2 == 0);
}

}
