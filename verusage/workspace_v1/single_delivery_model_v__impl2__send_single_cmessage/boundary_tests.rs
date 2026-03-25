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


// ========== BOUNDARY TESTS ==========
// These tests violate preconditions or use edge-case inputs that should be rejected.


// Test 1: Destination endpoint with an ID that is too long (>= 0x100000 bytes)
// violates dst@.valid_physical_address()
// SHOULD FAIL
proof fn test_boundary_invalid_dst_too_long(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
        // Intentionally use an invalid destination
        ({
            let dst = AbstractEndPoint { id: Seq::new(0x100000nat, |i: int| 0u8) };
            &&& !dst.valid_physical_address()
            &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, AbstractParameters::static_params())
        }),
    ensures
        post.send_state =~= pre.send_state,  // claim state didn't change
{
}


// Test 2: Sequence number overflow — num_packets_acked at max and un_acked is non-empty
// The spec says when new_seqno > max_seqno, post == pre and sm is None.
// This test tries to assert sm is Some (contradicting the overflow path).
// SHOULD FAIL
proof fn test_boundary_seqno_overflow_returns_some(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let params = AbstractParameters::static_params();
        let ack = AckState::<int> {
            num_packets_acked: params.max_seqno,
            un_acked: seq![],
        };
        &&& pre.send_state.contains_key(dst)
        &&& pre.send_state[dst] == ack
        &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, params)
    }),
{
    let dst = AbstractEndPoint { id: seq![1u8] };
    // When num_packets_acked == max_seqno, new_seqno = max_seqno + 0 + 1 > max_seqno
    // so sm must be None. Asserting Some should fail.
    assert(sm is Some);  // SHOULD FAIL
}


// Test 3: Empty send_state (no prior ack state for dst). Violate by assuming
// the resulting seqno is 0 (but spec dictates seqno = 0 + 0 + 1 = 1).
// SHOULD FAIL
proof fn test_boundary_fresh_dst_seqno_zero()
{
    let dst = AbstractEndPoint { id: seq![1u8] };
    let pre = SingleDelivery::<int> {
        receive_state: Map::empty(),
        send_state: Map::empty(),
    };
    let m = 42int;
    let params = AbstractParameters::static_params();
    let old_ack = ack_state_lookup(dst, pre.send_state);
    // old_ack should be the default (num_packets_acked=0, un_acked=empty)
    // new_seqno = 0 + 0 + 1 = 1
    // Claim seqno is 0 — should fail
    let sm_wrong = SingleMessage::<int>::Message { seqno: 0nat, m: m, dst: dst };
    let post = SingleDelivery::<int> {
        send_state: pre.send_state.insert(dst,
            AckState {
                un_acked: old_ack.un_acked.push(sm_wrong),
                ..old_ack }),
        ..pre
    };
    assert(SingleDelivery::send_single_message(pre, post, m, dst, Some(sm_wrong), params));  // SHOULD FAIL
}


// Test 4: num_packets_acked is u64::MAX - 1 and un_acked has 1 element.
// new_seqno = (u64::MAX-1) + 1 + 1 = u64::MAX + 1 > max_seqno, so sm must be None.
// Try to assert that state changes (it shouldn't when overflow).
// SHOULD FAIL
proof fn test_boundary_almost_overflow_state_changes(
    pre: SingleDelivery<int>,
    post: SingleDelivery<int>,
    m: int,
    sm: Option<SingleMessage<int>>,
)
    requires
    ({
        let dst = AbstractEndPoint { id: seq![1u8] };
        let params = AbstractParameters::static_params();
        let dummy_msg = SingleMessage::<int>::Message { seqno: (0xffff_ffff_ffff_ffffu64 as nat), m: 0int, dst: dst };
        let ack = AckState::<int> {
            num_packets_acked: (0xffff_ffff_ffff_fffeu64 as nat),
            un_acked: seq![dummy_msg],
        };
        &&& pre.send_state.contains_key(dst)
        &&& pre.send_state[dst] == ack
        &&& SingleDelivery::send_single_message(pre, post, m, dst, sm, params)
    }),
{
    // In overflow case, post == pre. Asserting they differ should fail.
    assert(post.send_state !== pre.send_state);  // SHOULD FAIL
}


// Test 5: Call with max_seqno = 0 in params (edge case for parameters).
// With empty ack state, new_seqno = 1 > 0, so must return None.
// Asserting Some should fail.
// SHOULD FAIL
proof fn test_boundary_zero_max_seqno()
{
    let dst = AbstractEndPoint { id: seq![1u8] };
    let pre = SingleDelivery::<int> {
        receive_state: Map::empty(),
        send_state: Map::empty(),
    };
    let m = 42int;
    let params = AbstractParameters { max_seqno: 0, max_delegations: 0 };
    let old_ack = ack_state_lookup(dst, pre.send_state);
    // new_seqno = 0 + 0 + 1 = 1 > 0 = max_seqno => overflow path
    // post == pre, sm is None
    // Try to assert sm is Some
    let sm_wrong = SingleMessage::<int>::Message { seqno: 1, m: m, dst: dst };
    let post_wrong = SingleDelivery::<int> {
        send_state: pre.send_state.insert(dst,
            AckState {
                un_acked: old_ack.un_acked.push(sm_wrong),
                ..old_ack }),
        ..pre
    };
    assert(SingleDelivery::send_single_message(pre, post_wrong, m, dst, Some(sm_wrong), params));  // SHOULD FAIL
}

}
