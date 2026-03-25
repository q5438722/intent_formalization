use vstd::prelude::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type EndpointPtr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type PAddr = usize;
pub type VAddr = usize;

pub const NUM_CPUS: usize = 32;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or
// uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: schedule_idle_cpu requires 0 <= cpu_id < NUM_CPUS.
// Passing cpu_id = NUM_CPUS (32) is exactly out of bounds.
// SHOULD FAIL
proof fn test_boundary_cpu_id_at_max() {
    let cpu_id: CpuId = NUM_CPUS;
    assert(0 <= cpu_id && cpu_id < NUM_CPUS);
}

// Test 2: schedule_idle_cpu requires 0 <= cpu_id < NUM_CPUS.
// Passing cpu_id = usize::MAX is far out of bounds.
// SHOULD FAIL
proof fn test_boundary_cpu_id_overflow() {
    let cpu_id: CpuId = usize::MAX;
    assert(cpu_id < NUM_CPUS);
}

// Test 3: schedule_idle_cpu requires 0 <= cpu_id < NUM_CPUS.
// Passing cpu_id = 33 is out of bounds.
// SHOULD FAIL
proof fn test_boundary_cpu_id_beyond_max() {
    let cpu_id: CpuId = 33usize;
    assert(cpu_id < NUM_CPUS);
}

// Test 4: get_cr3_by_pcid requires 0 <= pcid < PCID_MAX.
// Passing pcid = PCID_MAX violates the precondition.
// SHOULD FAIL
proof fn test_boundary_pcid_at_max() {
    let pcid: Pcid = PCID_MAX;
    assert(0 <= pcid && pcid < PCID_MAX);
}

// Test 5: get_cr3_by_pcid requires 0 <= pcid < PCID_MAX.
// Passing pcid = usize::MAX overflows.
// SHOULD FAIL
proof fn test_boundary_pcid_overflow() {
    let pcid: Pcid = usize::MAX;
    assert(pcid < PCID_MAX);
}

// Test 6: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 violates alignment.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 7: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 0xFFF (4095) violates alignment.
// SHOULD FAIL
proof fn test_boundary_page_ptr_just_below_alignment() {
    let ptr: usize = 0xFFF;
    assert(ptr % 0x1000 == 0);
}

// Test 8: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 9: pop_scheduler_for_idle_cpu requires cpu is active.
// If active == false, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_cpu_inactive() {
    let active: bool = false;
    assert(active == true);
}

// Test 10: pop_scheduler_for_idle_cpu requires current_thread.is_None().
// If current_thread.is_Some(), the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_cpu_has_current_thread(thread_ptr: ThreadPtr) {
    let current_thread: Option<ThreadPtr> = Some(thread_ptr);
    assert(current_thread is None);
}

// Test 11: pop_scheduler_for_idle_cpu requires scheduler.len() != 0.
// If scheduler is empty, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_scheduler_empty() {
    let scheduler_len: usize = 0;
    assert(scheduler_len != 0);
}

// Test 12: Array::get requires 0 <= i < N. For cpu_list with N=NUM_CPUS,
// passing i = NUM_CPUS is out of bounds.
// SHOULD FAIL
proof fn test_boundary_array_get_out_of_bounds() {
    let i: usize = NUM_CPUS;
    let n: usize = NUM_CPUS;
    assert(0 <= i && i < n);
}

}
