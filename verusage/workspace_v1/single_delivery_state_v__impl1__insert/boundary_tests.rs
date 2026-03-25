use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type definitions (from target file) =====

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl AbstractEndPoint {
    pub open spec fn valid_physical_address(self) -> bool {
        self.id.len() < 0x100000
    }

    pub open spec fn abstractable(self) -> bool {
        self.valid_physical_address()
    }
}

pub type TombstoneTable = Map<AbstractEndPoint, nat>;

pub open spec fn table_abstractable(table: TombstoneTable) -> bool {
    forall |k: AbstractEndPoint| #[trigger] table.contains_key(k) ==> k.valid_physical_address()
}

// ===== Boundary Tests =====

// B1: Endpoint at exact boundary (id.len() == 0x100000) should NOT be valid
// valid_physical_address requires strictly < 0x100000, so len == 0x100000 is invalid
// SHOULD FAIL
proof fn test_boundary_exact_limit_is_valid() {
    let ep = AbstractEndPoint { id: Seq::new(0x100000 as nat, |i: int| 0u8) };
    assert(ep.valid_physical_address());
}

// B2: Endpoint with oversized id (> 0x100000) should NOT be valid
// SHOULD FAIL
proof fn test_boundary_oversized_is_valid() {
    let ep = AbstractEndPoint { id: Seq::new(0x200000 as nat, |i: int| 0u8) };
    assert(ep.valid_physical_address());
}

// B3: A table containing a key with invalid address cannot be abstractable
// SHOULD FAIL
proof fn test_boundary_invalid_key_abstractable() {
    let bad_ep = AbstractEndPoint { id: Seq::new(0x100000 as nat, |i: int| 0u8) };
    let table: TombstoneTable = Map::empty().insert(bad_ep, 0 as nat);
    assert(table_abstractable(table));
}

// B4: Inserting an invalid endpoint into an abstractable table should NOT preserve abstractable
// Without the precondition src.valid_physical_address(), abstractable breaks
// SHOULD FAIL
proof fn test_boundary_insert_invalid_preserves_abstractable(
    old_table: TombstoneTable,
    last_seqno: nat,
) {
    let bad_src = AbstractEndPoint { id: Seq::new(0x200000 as nat, |i: int| 0u8) };
    assume(table_abstractable(old_table));
    // bad_src does NOT have valid_physical_address
    let new_table = old_table.insert(bad_src, last_seqno);
    // This should fail: new_table contains bad_src which is invalid
    assert(table_abstractable(new_table));
}

// B5: An empty table IS abstractable (vacuously true)
// Asserting it's NOT abstractable should fail
// SHOULD FAIL
proof fn test_boundary_empty_not_abstractable() {
    let table: TombstoneTable = Map::empty();
    assert(!table_abstractable(table));
}

}
