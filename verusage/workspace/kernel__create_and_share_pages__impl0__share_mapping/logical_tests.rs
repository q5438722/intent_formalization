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
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

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

// ===================== LOGICAL TESTS =====================
// Properties NOT explicitly guaranteed by share_mapping's postconditions.
// These test whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism of mapping ordering.
// Claim: sharing two VAs to the same page produces the same mapping set
// regardless of order. The spec uses set insert, so this happens to be true
// for sets, but the spec does not guarantee this for reference counters'
// intermediate states.
// More specifically: claim ref counter after 2 shares equals old + 1 (not +2).
// SHOULD FAIL
proof fn test_logical_double_share_ref_counter(
    old_ref: nat,
    after_first: nat,
    after_second: nat,
)
    requires
        after_first == old_ref + 1,
        after_second == after_first + 1,
{
    // Claim: after two shares, ref counter is old + 1 (wrong, should be old + 2)
    assert(after_second == old_ref + 1);
}

// Test 2: The spec does NOT guarantee that sharing is symmetric.
// Claim: if A shares to B, then B can share to A with the same entry.
// But this requires B's address space to contain the VA, which it only does
// AFTER the share. The spec doesn't guarantee the src_va precondition
// is satisfied in reverse.
// SHOULD FAIL
proof fn test_logical_share_symmetry(
    a_space: Map<VAddr, MapEntry>,
    b_space: Map<VAddr, MapEntry>,
    src_va: VAddr,
    target_va: VAddr,
)
    requires
        a_space.dom().contains(src_va),
        !b_space.dom().contains(target_va),
        !a_space.dom().contains(target_va),
{
    // After sharing: B now has target_va.
    // Claim: A also has target_va (which was never mapped in A).
    assert(a_space.dom().contains(target_va));
}

// Test 3: The spec does NOT guarantee that self-sharing (same proc) is allowed.
// Claim: src_proc_ptr == target_proc_ptr AND src_va != target_va should
// allow the operation. But the precondition says
// get_address_space(target_proc_ptr).dom().contains(target_va) == false.
// If src_proc_ptr == target_proc_ptr, then the src address space IS the target space.
// The target_va must NOT be in this space. This is consistent.
// But claim: after self-share, the original src_va is removed. (Not guaranteed.)
// SHOULD FAIL
proof fn test_logical_self_share_removes_src(
    addr_space: Map<VAddr, MapEntry>,
    src_va: VAddr,
    target_va: VAddr,
    entry: MapEntry,
)
    requires
        addr_space.dom().contains(src_va),
        !addr_space.dom().contains(target_va),
        src_va != target_va,
{
    let new_space = addr_space.insert(target_va, entry);
    // Claim: src_va was removed (move semantics). Not true — share is copy.
    assert(!new_space.dom().contains(src_va));
}

// Test 4: The spec does NOT guarantee that the entry's write permission
// is preserved or changed. Claim: shared entry always has write == false.
// SHOULD FAIL
proof fn test_logical_shared_entry_read_only(
    entry: MapEntry,
)
    requires
        entry.write == true,
{
    // Claim: sharing forces read-only
    assert(entry.write == false);
}

// Test 5: The spec does NOT guarantee that sharing decreases the free page count.
// It actually guarantees the count stays the same. But claim it increases.
// SHOULD FAIL
proof fn test_logical_free_pages_increase(
    old_free: usize,
    new_free: usize,
)
    requires
        old_free == new_free,  // postcondition: unchanged
{
    // Claim: sharing somehow frees a page
    assert(new_free == old_free + 1);
}

// Test 6: The spec does NOT guarantee that page_is_mapped changes for any page.
// It explicitly says page_is_mapped is unchanged. But test the opposite.
// SHOULD FAIL
proof fn test_logical_mapped_status_changed(
    old_mapped: bool,
    new_mapped: bool,
)
    requires
        old_mapped == new_mapped,  // postcondition: unchanged
        old_mapped == true,
{
    // Claim: sharing unmaps the page
    assert(new_mapped == false);
}

// Test 7: The spec does NOT guarantee that endpoint_dom changes.
// Claim: a new endpoint appeared.
// SHOULD FAIL
proof fn test_logical_endpoint_dom_changed(
    old_dom: Set<EndpointPtr>,
    new_dom: Set<EndpointPtr>,
    new_ep: EndpointPtr,
)
    requires
        new_dom == old_dom,  // postcondition: preserved
        !old_dom.contains(new_ep),
{
    // Claim: a new endpoint appeared after sharing
    assert(new_dom.contains(new_ep));
}

// Test 8: The spec does NOT guarantee that the target proc's pcid changes.
// Claim: pcid_to_proc_ptr mapping changed.
// SHOULD FAIL
proof fn test_logical_pcid_mapping_changed(
    old_proc_ptr: ProcPtr,
    new_proc_ptr: ProcPtr,
)
    requires
        old_proc_ptr == new_proc_ptr,  // postcondition: pcid_to_proc_ptr preserved
{
    // Claim: the pcid now points to a different proc
    assert(old_proc_ptr != new_proc_ptr);
}

// Test 9: The spec does NOT guarantee that the container's owned_pages changes.
// It explicitly preserves container_owned_pages. Claim it changed.
// SHOULD FAIL
proof fn test_logical_container_owned_pages_changed(
    old_pages: Set<PagePtr>,
    new_pages: Set<PagePtr>,
    new_page: PagePtr,
)
    requires
        new_pages =~= old_pages,  // postcondition: preserved
        !old_pages.contains(new_page),
{
    // Claim: a new page appeared in container's owned set
    assert(new_pages.contains(new_page));
}

// Test 10: The spec does NOT imply that two different target VAs
// can be shared simultaneously in one call. Claim: after one share_mapping
// call, TWO new entries appear in the target address space.
// SHOULD FAIL
proof fn test_logical_double_insert_in_single_call(
    old_addr_space: Map<VAddr, MapEntry>,
    new_addr_space: Map<VAddr, MapEntry>,
    target_va_1: VAddr,
    target_va_2: VAddr,
    entry: MapEntry,
)
    requires
        !old_addr_space.dom().contains(target_va_1),
        !old_addr_space.dom().contains(target_va_2),
        target_va_1 != target_va_2,
        new_addr_space =~= old_addr_space.insert(target_va_1, entry),  // postcondition: only one insert
{
    // Claim: target_va_2 is also in the new space
    assert(new_addr_space.dom().contains(target_va_2));
}

}
