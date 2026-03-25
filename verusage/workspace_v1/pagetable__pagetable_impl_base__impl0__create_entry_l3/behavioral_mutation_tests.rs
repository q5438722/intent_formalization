use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type PAddr = usize;
pub type VAddr = usize;
pub type PageMapPtr = usize;
pub type PagePtr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;

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

impl PageEntry {
    pub open spec fn is_empty(&self) -> bool {
        &&& self.addr == 0
        &&& self.perm.present == false
        &&& self.perm.ps == false
        &&& self.perm.write == false
        &&& self.perm.execute_disable == false
        &&& self.perm.user == false
    }
}

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test mutates an expected output or relationship and checks if
// the specification correctly rejects the incorrect behavior.

// Test 1: PageEntry with nonzero addr (but all perms false) is NOT empty.
// is_empty() requires addr == 0. Mutating addr to 0x1000 should be rejected.
// SHOULD FAIL
proof fn test_mutation_nonzero_addr_not_empty() {
    let pe = PageEntry {
        addr: 0x1000usize,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(pe.is_empty());
}

// Test 2: PageEntry with user bit set (but everything else correct) is NOT empty.
// is_empty() requires user == false. Mutating user to true should be rejected.
// SHOULD FAIL
proof fn test_mutation_user_set_not_empty() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: false,
            execute_disable: false,
            user: true,
        },
    };
    assert(pe.is_empty());
}

// Test 3: create_entry_l3 postcondition says page_closure grows by page_map_ptr.
// Mutate: assert it grew by a DIFFERENT pointer. This should be rejected.
// SHOULD FAIL
proof fn test_mutation_closure_wrong_ptr(
    old_closure: Set<PagePtr>,
    new_closure: Set<PagePtr>,
    page_map_ptr: PagePtr,
    wrong_ptr: PagePtr,
)
    requires
        new_closure =~= old_closure.insert(page_map_ptr),
        page_map_ptr != wrong_ptr,
        !old_closure.contains(page_map_ptr),
        !old_closure.contains(wrong_ptr),
{
    assert(new_closure =~= old_closure.insert(wrong_ptr));
}

// Test 4: create_entry_l3 postcondition says the new L3 mapping addr == page_map_ptr.
// Mutate: assert addr equals a different value. This should be rejected.
// SHOULD FAIL
proof fn test_mutation_l3_addr_wrong(
    resolved_addr: PageMapPtr,
    page_map_ptr: PageMapPtr,
    wrong_addr: PageMapPtr,
)
    requires
        resolved_addr == page_map_ptr,
        page_map_ptr != wrong_addr,
{
    assert(resolved_addr == wrong_addr);
}

// Test 5: create_entry_l3 postcondition says mapping_4k is preserved.
// Mutate: assert that a VA present in old mapping_4k is absent in new mapping_4k.
// This contradicts the preservation postcondition.
// SHOULD FAIL
proof fn test_mutation_mapping_4k_va_lost(
    old_m: Map<VAddr, MapEntry>,
    new_m: Map<VAddr, MapEntry>,
    va: VAddr,
)
    requires
        new_m =~= old_m,
        old_m.dom().contains(va),
{
    assert(!new_m.dom().contains(va));
}

}
