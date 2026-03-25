use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

global size_of usize == 8;

pub type PAddr = usize;

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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry_perm))]
pub fn usize2page_entry_perm(v: usize) -> (ret: PageEntryPerm)
    ensures
        ret =~= spec_usize2page_entry_perm(v),
        v == 0 ==> ret.present == false && ret.ps == false && ret.write == false
            && ret.execute_disable == false && ret.user == false,
{
    unimplemented!()
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

#[verifier::external_body]
pub fn page_entry2usize(page_entry: &PageEntry) -> (ret: usize)
    requires
        MEM_valid(page_entry.addr),
    ensures
        usize2present(ret) == page_entry.perm.present,
        usize2ps(ret) == page_entry.perm.ps,
        usize2write(ret) == page_entry.perm.write,
        usize2execute_disable(ret) == page_entry.perm.execute_disable,
        usize2user(ret) == page_entry.perm.user,
        usize2pa(ret) == page_entry.addr,
        usize2page_entry_perm(ret) =~= page_entry.perm,
{
    unimplemented!()
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — valid inputs, mutated assertions
// Each test calls page_entry2usize with valid input and asserts
// a WRONG property that contradicts a postcondition.
// ============================================================

// SHOULD FAIL — present bit is negated: postcondition says present == true
fn test_mutation_negate_present() {
    assert(0usize & (!0x0000_ffff_ffff_f000u64) as usize == 0usize) by(bit_vector);
    let entry = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm { present: true, ps: false, write: false, execute_disable: false, user: false },
    };
    let ret = page_entry2usize(&entry);
    assert(usize2present(ret) == false);
}

// SHOULD FAIL — address mutated: postcondition says addr == 0x1000, not 0x2000
fn test_mutation_wrong_address() {
    assert(0x1000usize & (!0x0000_ffff_ffff_f000u64) as usize == 0usize) by(bit_vector);
    let entry = PageEntry {
        addr: 0x1000usize,
        perm: PageEntryPerm { present: false, ps: false, write: false, execute_disable: false, user: false },
    };
    let ret = page_entry2usize(&entry);
    assert(spec_usize2pa(ret) == 0x2000usize);
}

// SHOULD FAIL — write bit flipped: postcondition says write == false
fn test_mutation_flip_write() {
    assert(0usize & (!0x0000_ffff_ffff_f000u64) as usize == 0usize) by(bit_vector);
    let entry = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm { present: true, ps: true, write: false, execute_disable: false, user: false },
    };
    let ret = page_entry2usize(&entry);
    assert(usize2write(ret) == true);
}

}
