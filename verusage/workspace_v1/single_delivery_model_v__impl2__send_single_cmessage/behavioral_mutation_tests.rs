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


// ========== BEHAVIORAL MUTATION TESTS ==========
// These tests assume valid inputs and spec satisfaction, but assert WRONG outputs/relations.


// Test 1: Mutate the seqno — claim the returned message has seqno = new_seqno + 1 (off by one).
// The spec says seqno = num_packets_acked + un_acked.len() + 1.
// For fresh dst: seqno = 0 + 0 + 1 = 1. Asserting seqno == 2 should fail.
// SHOULD FAIL
proof fn test_mutation_wrong_seqno(
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
    let dst = AbstractEndPoint { id: seq![1u8] };
    // The spec says sm == Some(Message{seqno: 1, ...}). Assert seqno is 2.
    assert(sm is Some);
    let msg = sm.unwrap();
    assert(msg is Message);
    assert(msg->Message_seqno == 2nat);  // SHOULD FAIL: actual seqno is 1
}


// Test 2: Mutate the destination — claim the returned message has a DIFFERENT dst.
// The spec says sm.dst == dst. Asserting a different dst should fail.
// SHOULD FAIL
proof fn test_mutation_wrong_dst(
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
    let wrong_dst = AbstractEndPoint { id: seq![2u8] };
    assert(sm is Some);
    let msg = sm.unwrap();
    assert(msg->Message_dst == wrong_dst);  // SHOULD FAIL: dst should be seq![1u8]
}


// Test 3: Mutate the message payload — claim returned sm contains different message m.
// The spec says sm.m == m. Asserting a different m should fail.
// SHOULD FAIL
proof fn test_mutation_wrong_message_payload(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let m_actual = 42int;
        let params = AbstractParameters::static_params();
        &&& !pre.send_state.contains_key(dst)
        &&& SingleDelivery::send_single_message(pre, post, m_actual, dst, sm, params)
    }),
{
    assert(sm is Some);
    let msg = sm.unwrap();
    assert(msg->Message_m == 99int);  // SHOULD FAIL: actual m is 42
}


// Test 4: Mutate the result — claim None is returned when there IS room for seqnos.
// For a fresh dst, new_seqno = 1 <= max_seqno, so Some should be returned.
// SHOULD FAIL
proof fn test_mutation_none_when_should_be_some(
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
    assert(sm is None);  // SHOULD FAIL: should be Some
}


// Test 5: Mutate state — claim post.send_state equals pre.send_state after successful send.
// When a message is sent (not overflow), the send_state is updated with the new message.
// SHOULD FAIL
proof fn test_mutation_state_unchanged_after_send(
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
    let dst = AbstractEndPoint { id: seq![1u8] };
    // After successful send, send_state should have dst mapped. Claiming it doesn't should fail.
    assert(!post.send_state.contains_key(dst));  // SHOULD FAIL
}

}
