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
// LOGICAL TESTS - Properties NOT explicitly guaranteed
// clone_up_to_view ensures: c@ == self@
// These tests assert properties that the spec does NOT entail.
// All should FAIL verification.
// ============================================================

// Test L1: Two Messages with same seqno/dst but different keys are NOT equal
// View equality distinguishes keys; the spec does not abstract them away.
// SHOULD FAIL
proof fn test_l1_different_keys_not_equal() {
    let msg1 = SingleMessage::Message::<Message> {
        seqno: 5,
        dst: empty_dst(),
        m: Message::GetRequest { key: SHTKey { ukey: 1 } },
    };
    let msg2 = SingleMessage::Message::<Message> {
        seqno: 5,
        dst: empty_dst(),
        m: Message::GetRequest { key: SHTKey { ukey: 2 } },
    };
    assert(msg1 == msg2); // SHOULD FAIL: different keys (ukey 1 vs 2)
}

// Test L2: AckState::new() does NOT have num_packets_acked == 1
// The spec defines new() with num_packets_acked = 0.
// SHOULD FAIL
proof fn test_l2_wrong_initial_num_packets() {
    let state = AckState::<Message>::new();
    assert(state.num_packets_acked == 1); // SHOULD FAIL: it's 0, not 1
}

// Test L3: num_packets_acked is NOT required to equal un_acked length
// These are independent fields; the spec does not enforce this invariant.
// SHOULD FAIL
proof fn test_l3_packets_acked_independent_of_length() {
    let state = AckState::<Message> {
        num_packets_acked: 5,
        un_acked: seq![],
    };
    assert(state.num_packets_acked == state.un_acked.len()); // SHOULD FAIL: 5 != 0
}

// Test L4: Swapping un_acked elements does NOT preserve equality
// Seq is ordered; element order matters for view equality.
// SHOULD FAIL
proof fn test_l4_swap_breaks_equality() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let state1 = AckState::<Message> { num_packets_acked: 0, un_acked: seq![msg1, msg2] };
    let state2 = AckState::<Message> { num_packets_acked: 0, un_acked: seq![msg2, msg1] };
    assert(state1.un_acked =~= state2.un_acked); // SHOULD FAIL: different element order
}

// Test L5: Two Messages with same seqno/dst but different message types are NOT equal
// GetRequest vs Reply have different view representations.
// SHOULD FAIL
proof fn test_l5_different_message_types_not_equal() {
    let msg_a = SingleMessage::Message::<Message> {
        seqno: 5,
        dst: empty_dst(),
        m: Message::GetRequest { key: SHTKey { ukey: 0 } },
    };
    let msg_b = SingleMessage::Message::<Message> {
        seqno: 5,
        dst: empty_dst(),
        m: Message::Reply { key: SHTKey { ukey: 0 }, value: None },
    };
    assert(msg_a == msg_b); // SHOULD FAIL: GetRequest != Reply
}

}
