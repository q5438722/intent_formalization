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
// LOGICAL TESTS: Properties NOT explicitly guaranteed
// Field-level tests to avoid Z3 issues with generic struct equality
// ============================================================

// L1: Receiving a message only affects the specific source's tombstone
// Assert a DIFFERENT source's tombstone changed — should fail because
// map insert only modifies the specified key
// SHOULD FAIL
proof fn test_logical_insert_affects_other_keys() {
    let pre_rs: TombstoneTable = arbitrary();
    let src = AbstractEndPoint { id: seq![1u8] };
    let src2 = AbstractEndPoint { id: seq![2u8] };
    let last_seqno = tombstone_table_lookup(src, pre_rs);

    // From receive_real_packet: only src's entry changes
    let post_rs = pre_rs.insert(src, (last_seqno + 1) as nat);

    // Assert src2's tombstone changed — SHOULD FAIL (insert doesn't affect other keys)
    assert(tombstone_table_lookup(src2, post_rs) != tombstone_table_lookup(src2, pre_rs));
}

// L2: new_single_message is NOT universally true for all Message packets
// The seqno must match last_seqno + 1, which is not always the case
// SHOULD FAIL
proof fn test_logical_always_new_message() {
    let sd_rs: TombstoneTable = arbitrary();
    let pkt_src: AbstractEndPoint = arbitrary();
    let pkt_seqno: nat = arbitrary();

    let last_seqno = tombstone_table_lookup(pkt_src, sd_rs);

    // new_single_message requires seqno == last_seqno + 1, not always true
    assert(pkt_seqno == last_seqno + 1);
}

// L3: The ack message is of variant Ack, NOT variant Message
// send_ack ensures ack.msg is Ack — asserting it's Message should fail
// SHOULD FAIL
proof fn test_logical_ack_is_message_type() {
    let ack_msg: PMsg = arbitrary();

    assume(ack_msg is Ack);

    // Ack and Message are different enum variants
    assert(ack_msg is Message);
}

// L4: After receiving a fresh packet from tombstone 0, tombstone is 1, NOT >= 2
// Tests that the spec doesn't allow stronger tombstone increments
// SHOULD FAIL
proof fn test_logical_tombstone_jumps_by_two() {
    let pre_rs: TombstoneTable = arbitrary();
    let src: AbstractEndPoint = arbitrary();

    assume(tombstone_table_lookup(src, pre_rs) == 0 as nat);
    let post_rs = pre_rs.insert(src, 1 as nat);

    // Tombstone should be 1, NOT >= 2
    assert(tombstone_table_lookup(src, post_rs) >= 2 as nat);
}

// L5: new_single_message and should_ack are mutually exclusive
// new_single_message: seqno == last_seqno + 1 (implies seqno > last_seqno)
// should_ack: seqno <= last_seqno
// These cannot both be true simultaneously
// SHOULD FAIL
proof fn test_logical_new_and_should_ack_both_true() {
    let last_seqno: nat = arbitrary();
    // From new_single_message: seqno == last_seqno + 1
    let msg_seqno: nat = last_seqno + 1;

    // Try to also satisfy should_ack: seqno <= last_seqno
    // This is impossible since last_seqno + 1 > last_seqno
    assert(msg_seqno <= last_seqno);
}


} // verus!
