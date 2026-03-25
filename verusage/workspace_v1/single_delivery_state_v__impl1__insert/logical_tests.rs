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
}

pub type TombstoneTable = Map<AbstractEndPoint, nat>;

pub open spec fn table_abstractable(table: TombstoneTable) -> bool {
    forall |k: AbstractEndPoint| #[trigger] table.contains_key(k) ==> k.valid_physical_address()
}

// ===== Logical Tests =====
// These test properties NOT explicitly guaranteed by the specification.

// L1: Soundness check - assuming the spec should not derive false
// SHOULD FAIL
proof fn test_logical_unsound(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    let new_table = old_table.insert(src, last_seqno);
    assume(table_abstractable(new_table));
    assume(new_table =~= old_table.insert(src, last_seqno));
    // Cannot derive false from consistent assumptions
    assert(false);
}

// L2: Insert should be deterministic
// Two calls to insert with same args should yield identical results
// Asserting they differ should fail
// SHOULD FAIL
proof fn test_logical_not_deterministic(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    let t1 = old_table.insert(src, last_seqno);
    let t2 = old_table.insert(src, last_seqno);
    // Map::insert is a pure function, so t1 == t2
    assert(!(t1 =~= t2));
}

// L3: Abstractable does NOT imply the table is empty
// There exist non-empty abstractable tables
// SHOULD FAIL
proof fn test_logical_abstractable_implies_empty(
    table: TombstoneTable,
) {
    assume(table_abstractable(table));
    // This is too strong: abstractable tables can have keys
    assert(table =~= Map::empty());
}

// L4: Valid endpoints do NOT all share the same id
// There exist multiple distinct valid endpoints
// SHOULD FAIL
proof fn test_logical_valid_endpoints_same_id(
    ep1: AbstractEndPoint,
    ep2: AbstractEndPoint,
) {
    assume(ep1.valid_physical_address());
    assume(ep2.valid_physical_address());
    // Being valid does not imply equality
    assert(ep1 == ep2);
}

// L5: Inserting key k twice with different values does NOT preserve the first value
// The second insert overwrites
// SHOULD FAIL
proof fn test_logical_first_value_preserved(
    table: TombstoneTable,
    src: AbstractEndPoint,
    v1: nat,
    v2: nat,
) {
    assume(table_abstractable(table));
    assume(src.valid_physical_address());
    assume(v1 != v2);
    let t1 = table.insert(src, v1);
    let t2 = t1.insert(src, v2);
    // Second insert overwrites: t2[src] == v2, not v1
    assert(t2[src] == v1);
}

}
