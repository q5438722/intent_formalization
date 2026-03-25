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
// BOUNDARY TESTS - Edge cases at spec boundaries
// clone_up_to_view ensures: c@ == self@
// These tests check that the spec properly constrains behavior at edge values.
// All tests should FAIL verification.
// ============================================================

// Test B1: Message with seqno=0 — the clone view must preserve seqno=0
// The postcondition c@ == self@ means the view seqno matches exactly.
// SHOULD FAIL
proof fn test_b1_seqno_zero_not_positive() {
    let msg = make_msg(0);
    assert(msg.arrow_Message_seqno() > 0); // SHOULD FAIL: seqno is 0
}

// Test B2: InvalidMessage variant — clone cannot become a Message variant
// The postcondition preserves variant identity through view equality.
// SHOULD FAIL
proof fn test_b2_invalid_message_is_not_message() {
    let inv: SingleMessage<Message> = SingleMessage::InvalidMessage {};
    assert(inv is Message); // SHOULD FAIL: it's InvalidMessage, not Message
}

// Test B3: Ack with ack_seqno=0 — clone view must preserve ack_seqno=0
// SHOULD FAIL
proof fn test_b3_ack_seqno_zero_not_positive() {
    let ack: SingleMessage<Message> = SingleMessage::Ack { ack_seqno: 0 };
    assert(ack.arrow_Ack_ack_seqno() > 0); // SHOULD FAIL: ack_seqno is 0
}

// Test B4: AckState with num_packets_acked=0 — clone view preserves zero
// SHOULD FAIL
proof fn test_b4_ack_state_zero_packets_not_positive() {
    let state = AckState::<Message>::new();
    assert(state.num_packets_acked > 0); // SHOULD FAIL: it's 0
}

// Test B5: AckState with empty un_acked — clone view preserves empty list
// SHOULD FAIL
proof fn test_b5_ack_state_empty_un_acked_not_nonempty() {
    let state = AckState::<Message>::new();
    assert(state.un_acked.len() > 0); // SHOULD FAIL: un_acked is empty
}

}
