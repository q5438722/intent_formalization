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

// ===== Behavioral Mutation Tests =====
// These assume valid preconditions and postconditions, then assert wrong behaviors.

// M1: After insert, the value at the key should be last_seqno
// Asserting a different value should fail
// SHOULD FAIL
proof fn test_mutation_wrong_value(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    let new_table = old_table.insert(src, last_seqno);
    // Map::insert guarantees new_table[src] == last_seqno
    assert(new_table[src] != last_seqno);
}

// M2: After insert, the key should be in the table
// Asserting it's NOT present should fail
// SHOULD FAIL
proof fn test_mutation_key_not_present(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    let new_table = old_table.insert(src, last_seqno);
    assert(!new_table.contains_key(src));
}

// M3: After insert, a key that wasn't in old_table and isn't src should NOT appear
// Asserting a spurious key exists should fail
// SHOULD FAIL
proof fn test_mutation_spurious_key(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    other: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    assume(src != other);
    assume(!old_table.contains_key(other));
    let new_table = old_table.insert(src, last_seqno);
    // insert(src, _) should not add 'other'
    assert(new_table.contains_key(other));
}

// M4: After insert of a new key, the table should change
// Asserting it's unchanged should fail
// SHOULD FAIL
proof fn test_mutation_table_unchanged(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    last_seqno: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    assume(!old_table.contains_key(src));
    let new_table = old_table.insert(src, last_seqno);
    // src is new, so new_table != old_table
    assert(new_table =~= old_table);
}

// M5: After insert, an existing key's value should NOT change
// Asserting it did change should fail
// SHOULD FAIL
proof fn test_mutation_existing_value_changed(
    old_table: TombstoneTable,
    src: AbstractEndPoint,
    other: AbstractEndPoint,
    last_seqno: nat,
    old_val: nat,
) {
    assume(table_abstractable(old_table));
    assume(src.valid_physical_address());
    assume(src != other);
    assume(old_table.contains_key(other));
    assume(old_table[other] == old_val);
    let new_table = old_table.insert(src, last_seqno);
    // insert(src, _) does not change value at 'other'
    assert(new_table[other] != old_val);
}

}
