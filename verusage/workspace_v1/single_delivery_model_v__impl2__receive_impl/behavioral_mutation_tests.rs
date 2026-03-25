extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;

fn main() {}

verus! {

// === Minimal Type Definitions (spec-level only) ===

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

pub type TombstoneTable = Map<AbstractEndPoint, nat>;

pub trait KeyTrait {}
pub trait VerusClone {}

#[derive(Eq, PartialEq, Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

impl VerusClone for SHTKey {}
impl KeyTrait for SHTKey {}

pub type AbstractKey = SHTKey;
pub type AbstractValue = Seq<u8>;
pub type Hashtable = Map<AbstractKey, AbstractValue>;

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

pub enum Message {
    GetRequest { key: AbstractKey },
    SetRequest { key: AbstractKey, value: Option<AbstractValue> },
    Reply { key: AbstractKey, value: Option<AbstractValue> },
    Redirect { key: AbstractKey, id: AbstractEndPoint },
    Shard { range: KeyRange<AbstractKey>, recipient: AbstractEndPoint },
    Delegate { range: KeyRange<AbstractKey>, h: Hashtable },
}

pub enum SingleMessage<MT> {
    Message {
        seqno: nat,
        dst: AbstractEndPoint,
        m: MT,
    },
    Ack {
        ack_seqno: nat,
    },
    InvalidMessage {},
}

pub type PMsg = SingleMessage<Message>;

pub struct Packet {
    pub dst: AbstractEndPoint,
    pub src: AbstractEndPoint,
    pub msg: PMsg,
}

// === Spec Functions ===

pub open spec fn tombstone_table_lookup(src: AbstractEndPoint, t: TombstoneTable) -> nat {
    if t.dom().contains(src) { t[src] } else { 0 }
}


// ============================================================
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// Field-level tests using let-bindings to avoid Z3 map equality issues
// ============================================================

// M1: receive_real_packet with new message must change receive_state
// Spec says: post_rs = pre_rs.insert(src, last_seqno+1)
// Mutate: assert receive_state didn't change
// SHOULD FAIL
proof fn test_mutation_fresh_packet_no_state_change() {
    let pre_rs: TombstoneTable = arbitrary();
    let src: AbstractEndPoint = arbitrary();
    let last_seqno = tombstone_table_lookup(src, pre_rs);

    // From receive_real_packet postcondition when new_single_message is true
    let post_rs = pre_rs.insert(src, (last_seqno + 1) as nat);

    // Mutated: assert no change — SHOULD FAIL
    assert(post_rs == pre_rs);
}

// M2: send_ack swaps src/dst directions
// Spec says: ack.dst == pkt.src
// Mutate: assert ack.dst == pkt.dst (wrong direction)
// SHOULD FAIL
proof fn test_mutation_ack_wrong_direction() {
    let pkt_src = AbstractEndPoint { id: seq![1u8] };
    let pkt_dst = AbstractEndPoint { id: seq![2u8] };

    // From send_ack: ack.dst == pkt.src
    let ack_dst = pkt_src;

    // Mutated: assert ack.dst == pkt.dst (wrong direction) — SHOULD FAIL
    assert(ack_dst == pkt_dst);
}

// M3: send_ack seqno must match
// Spec says: ack.msg.ack_seqno == pkt.msg.seqno
// Mutate: assert seqno mismatch
// SHOULD FAIL
proof fn test_mutation_ack_wrong_seqno() {
    let msg_seqno: nat = 42;
    let ack_seqno: nat = arbitrary();

    // From send_ack: ack_seqno == msg_seqno
    assume(ack_seqno == msg_seqno);

    // Mutated: assert mismatch — SHOULD FAIL
    assert(ack_seqno != 42 as nat);
}

// M4: Tombstone value after fresh packet must be last_seqno + 1
// Spec says: post_rs = pre_rs.insert(src, last_seqno+1) with last_seqno = 0
// Mutate: assert tombstone becomes 2 instead of 1
// SHOULD FAIL
proof fn test_mutation_wrong_tombstone_value() {
    let pre_rs: TombstoneTable = arbitrary();
    let src: AbstractEndPoint = arbitrary();

    assume(tombstone_table_lookup(src, pre_rs) == 0 as nat);
    // receive_real_packet inserts (src, 0+1) = (src, 1)
    let post_rs = pre_rs.insert(src, 1 as nat);

    // Mutated: assert tombstone is 2 instead of 1 — SHOULD FAIL
    assert(tombstone_table_lookup(src, post_rs) == 2 as nat);
}

// M5: When should_ack is true, acks must be non-empty (singleton set)
// Spec says: acks == set![ack]
// Mutate: assert acks is empty
// SHOULD FAIL
proof fn test_mutation_should_ack_but_empty_acks() {
    let ack: Packet = arbitrary();
    let acks: Set<Packet> = arbitrary();

    // From send_ack: acks == set![ack]
    assume(acks == set![ack]);

    // Mutated: assert acks is empty — SHOULD FAIL
    assert(acks.is_empty());
}


} // verus!
