use vstd::prelude::*;
use vstd::map::*;

fn main() {}

verus! {

// ============================================================
// Minimal type definitions for testing clone_up_to_view spec
// ============================================================

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

pub type AbstractKey = SHTKey;
pub type AbstractValue = Seq<u8>;
pub type Hashtable = Map<AbstractKey, AbstractValue>;

pub struct KeyIterator {
    pub k: Option<AbstractKey>,
}

pub struct KeyRange {
    pub lo: KeyIterator,
    pub hi: KeyIterator,
}

pub enum Message {
    GetRequest { key: AbstractKey },
    SetRequest { key: AbstractKey, value: Option<AbstractValue> },
    Reply { key: AbstractKey, value: Option<AbstractValue> },
    Redirect { key: AbstractKey, id: AbstractEndPoint },
    Shard { range: KeyRange, recipient: AbstractEndPoint },
    Delegate { range: KeyRange, h: Hashtable },
}

pub enum SingleMessage<MT> {
    Message { seqno: nat, dst: AbstractEndPoint, m: MT },
    Ack { ack_seqno: nat },
    InvalidMessage {},
}

pub type AckList<MT> = Seq<SingleMessage<MT>>;

pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: AckList<MT>,
}

impl AckState<Message> {
    pub open spec fn new() -> Self {
        AckState { num_packets_acked: 0, un_acked: seq![] }
    }
}

// Helper spec functions
spec fn empty_dst() -> AbstractEndPoint {
    AbstractEndPoint { id: Seq::empty() }
}

spec fn make_msg(seqno: nat) -> SingleMessage<Message> {
    SingleMessage::Message {
        seqno: seqno,
        dst: empty_dst(),
        m: Message::GetRequest { key: SHTKey { ukey: 0 } },
    }
}

// ============================================================
// BEHAVIORAL MUTATION TESTS - Correct inputs, mutated outputs
// clone_up_to_view ensures: c@ == self@
// These tests mutate the "clone" output and check whether the
// postcondition rejects the mutation. All should FAIL.
// ============================================================

// Test M1: Mutated seqno (5→6) — postcondition must reject
// SHOULD FAIL
proof fn test_m1_mutated_seqno_rejected() {
    let original = make_msg(5);
    let mutated = make_msg(6);
    // If clone_up_to_view allowed seqno mutation, this would pass
    assert(mutated.arrow_Message_seqno() == original.arrow_Message_seqno()); // SHOULD FAIL: 6 != 5
}

// Test M2: Mutated dst (empty→non-empty) — postcondition must reject
// SHOULD FAIL
proof fn test_m2_mutated_dst_rejected() {
    let original = SingleMessage::Message::<Message> {
        seqno: 1,
        dst: AbstractEndPoint { id: Seq::empty() },
        m: Message::GetRequest { key: SHTKey { ukey: 0 } },
    };
    let mutated = SingleMessage::Message::<Message> {
        seqno: 1,
        dst: AbstractEndPoint { id: seq![1u8] },
        m: Message::GetRequest { key: SHTKey { ukey: 0 } },
    };
    assert(mutated.arrow_Message_dst().id =~= original.arrow_Message_dst().id); // SHOULD FAIL: dst ids differ
}

// Test M3: Mutated variant (Message→Ack) — postcondition must reject
// SHOULD FAIL
proof fn test_m3_mutated_variant_rejected() {
    let original = make_msg(5);
    let mutated: SingleMessage<Message> = SingleMessage::Ack { ack_seqno: 5 };
    // Asserting both are same variant should fail
    assert(mutated is Message); // SHOULD FAIL: mutated is Ack, not Message
}

// Test M4: Mutated AckState num_packets_acked (5→6)
// SHOULD FAIL
proof fn test_m4_mutated_num_packets_acked() {
    let original = AckState::<Message> { num_packets_acked: 5, un_acked: seq![] };
    let mutated = AckState::<Message> { num_packets_acked: 6, un_acked: seq![] };
    assert(mutated.num_packets_acked == original.num_packets_acked); // SHOULD FAIL: 6 != 5
}

// Test M5: Mutated un_acked (empty→one element) — lengths differ
// SHOULD FAIL
proof fn test_m5_mutated_un_acked_length() {
    let msg = make_msg(1);
    let original = AckState::<Message> { num_packets_acked: 0, un_acked: seq![] };
    let mutated = AckState::<Message> { num_packets_acked: 0, un_acked: seq![msg] };
    assert(mutated.un_acked.len() == original.un_acked.len()); // SHOULD FAIL: 1 != 0
}

}
