use vstd::prelude::*;

fn main() {}

verus! {

// ===== Minimal type definitions needed for spec-level reasoning =====

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

pub struct AbstractParameters {
    pub max_seqno: nat,
    pub max_delegations: nat,
}

impl AbstractParameters {
    pub open spec fn static_params() -> AbstractParameters {
        AbstractParameters {
            max_seqno: 0xffff_ffff_ffff_ffff as nat,
            max_delegations: 0x7FFF_FFFF_FFFF_FFFF as nat,
        }
    }
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

#[verifier::ext_equal]
pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: Seq<SingleMessage<MT>>,
}

impl AckState<int> {
    pub open spec fn new() -> Self {
        AckState { num_packets_acked: 0, un_acked: seq![] }
    }
}

pub type TombstoneTable = Map<AbstractEndPoint, nat>;
pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;

pub open spec(checked) fn ack_state_lookup<MT>(src: AbstractEndPoint, send_state: SendState<MT>) -> AckState<MT> {
    if send_state.contains_key(src)
        { send_state[src] }
    else
        { AckState { num_packets_acked: 0, un_acked: Seq::empty() } }
}

#[verifier::ext_equal]
pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

impl<MT> SingleDelivery<MT> {
    pub open spec(checked) fn send_single_message(
        pre: Self, post: Self, m: MT, dst: AbstractEndPoint,
        sm: Option<SingleMessage<MT>>, params: AbstractParameters
    ) -> bool {
        let old_ack_state = ack_state_lookup(dst, pre.send_state);
        let new_seqno = old_ack_state.num_packets_acked + old_ack_state.un_acked.len() + 1;
        if new_seqno > params.max_seqno {
            &&& post == pre
            &&& sm is None
        } else {
            &&& sm == Some(SingleMessage::<MT>::Message {
                    seqno: new_seqno,
                    m: m,
                    dst: dst,
                })
            &&& post == SingleDelivery {
                send_state: pre.send_state.insert(dst,
                    AckState {
                        un_acked: old_ack_state.un_acked.push(sm.unwrap()),
                        ..old_ack_state }),
                ..pre }
        }
    }
}


// ========== LOGICAL TESTS ==========
// These tests assert properties NOT explicitly guaranteed by the spec.
// They probe for over-entailment: does the spec accidentally allow unintended reasoning?


// Test 1: Claim that sending to two different destinations produces the same seqno
// AND the same output message. Even if both are fresh, the dst differs, so the
// returned messages differ (different dst field).
// SHOULD FAIL
proof fn test_logical_different_dst_same_message(
    pre: SingleDelivery<int>,
    post1: SingleDelivery<int>,
    post2: SingleDelivery<int>,
    m: int,
    sm1: Option<SingleMessage<int>>,
    sm2: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst1 = AbstractEndPoint { id: seq![1u8] };
        let dst2 = AbstractEndPoint { id: seq![2u8] };
        let params = AbstractParameters::static_params();
        &&& !pre.send_state.contains_key(dst1)
        &&& !pre.send_state.contains_key(dst2)
        &&& SingleDelivery::send_single_message(pre, post1, m, dst1, sm1, params)
        &&& SingleDelivery::send_single_message(pre, post2, m, dst2, sm2, params)
    }),
{
    // Both seqnos should be 1 (same), but dst fields differ.
    // Asserting sm1 == sm2 should fail because dst differs.
    assert(sm1 == sm2);  // SHOULD FAIL
}


// Test 2: Claim that receive_state changes after send_single_message.
// The spec only updates send_state, never receive_state.
// post.receive_state should equal pre.receive_state.
// Asserting they differ should fail.
// SHOULD FAIL
proof fn test_logical_receive_state_changes(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let params = AbstractParameters::static_params();
        &&& !pre.send_state.contains_key(dst)
        &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, params)
    }),
{
    assert(post.receive_state !== pre.receive_state);  // SHOULD FAIL
}


// Test 3: Claim that the seqno is strictly monotonically increasing across sends.
// Specifically, after two sequential sends to the same dst, the second seqno
// should be first_seqno + 1. But asserting second_seqno == first_seqno is wrong.
// SHOULD FAIL
proof fn test_logical_seqno_not_monotonic(
    pre: SingleDelivery<int>,
    mid: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m1: int,
    m2: int,
    sm1: Option<SingleMessage<int>>,
    sm2: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let params = AbstractParameters::static_params();
        &&& !pre.send_state.contains_key(dst)
        &&& SingleDelivery::send_single_message(pre, mid, m1, dst, sm1, params)
        &&& SingleDelivery::send_single_message(mid, post, m2, dst, sm2, params)
    }),
{
    // First send: seqno = 1. Second send: seqno = 2.
    // Claiming they have the same seqno should fail.
    assert(sm1 is Some && sm2 is Some);
    let msg1 = sm1.unwrap();
    let msg2 = sm2.unwrap();
    assert(msg1->Message_seqno == msg2->Message_seqno);  // SHOULD FAIL: 1 != 2
}


// Test 4: Claim the send_state for OTHER endpoints is modified.
// The spec only inserts/updates the entry for `dst`.
// Other endpoints' ack states should remain unchanged.
// SHOULD FAIL
proof fn test_logical_other_endpoint_modified(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let other = AbstractEndPoint { id: seq![2u8] };
        let params = AbstractParameters::static_params();
        let other_ack = AckState::<int> { num_packets_acked: 5, un_acked: seq![] };
        &&& !pre.send_state.contains_key(dst)
        &&& pre.send_state.contains_key(other)
        &&& pre.send_state[other] == other_ack
        &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, params)
    }),
{
    let other = AbstractEndPoint { id: seq![2u8] };
    let wrong_ack = AckState::<int> { num_packets_acked: 999, un_acked: seq![] };
    // The spec preserves other endpoints' state via map insert semantics.
    // Asserting the other endpoint now has a different ack state should fail.
    assert(post.send_state[other] == wrong_ack);  // SHOULD FAIL
}


// Test 5: Claim the returned SingleMessage is an Ack instead of a Message.
// The spec explicitly constructs SingleMessage::Message for the success case.
// Asserting it's an Ack should fail.
// SHOULD FAIL
proof fn test_logical_result_is_ack(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let params = AbstractParameters::static_params();
        &&& !pre.send_state.contains_key(dst)
        &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, params)
    }),
{
    assert(sm is Some);
    let msg = sm.unwrap();
    assert(msg is Ack);  // SHOULD FAIL: it's a Message, not an Ack
}

}
