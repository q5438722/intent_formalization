use vstd::prelude::*;

fn main() {}

verus! {

// ===== Minimal type definitions for spec-level reasoning =====

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl AbstractEndPoint {
    pub open spec fn valid_physical_address(self) -> bool {
        self.id.len() < 0x100000
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

pub type TombstoneTable = Map<AbstractEndPoint, nat>;
pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;

pub struct Packet {
    pub dst: AbstractEndPoint,
    pub src: AbstractEndPoint,
    pub msg: SingleMessage<int>,
}

#[verifier::ext_equal]
pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

pub open spec fn flatten_sets<A>(sets: Set<Set<A>>) -> Set<A> {
    Set::new(|a: A| (exists |s: Set<A>| sets.contains(s) && s.contains(a)))
}

impl SingleDelivery<int> {

    pub open spec(checked) fn un_acked_messages_for_dest_up_to(self, src: AbstractEndPoint, dst: AbstractEndPoint, count: nat) -> Set<Packet>
    recommends
        self.send_state.contains_key(dst),
        count <= self.send_state[dst].un_acked.len()
    {
        Set::new(|p: Packet| {
            &&& p.src == src
            &&& exists |i: int| {
                &&& 0 <= i < count
                &&& self.send_state[dst].un_acked[i] is Message
                &&& p.msg == self.send_state[dst].un_acked[i]
                &&& p.dst == p.msg->Message_dst
            }
        })
    }

    pub open spec(checked) fn un_acked_messages_for_dest(self, src: AbstractEndPoint, dst: AbstractEndPoint) -> Set<Packet>
    recommends
        self.send_state.contains_key(dst)
    {
        self.un_acked_messages_for_dest_up_to(src, dst, self.send_state[dst].un_acked.len())
    }

    pub open spec fn un_acked_messages_for_dests(self, src: AbstractEndPoint, dsts: Set<AbstractEndPoint>) -> Set<Packet>
        recommends dsts.subset_of(self.send_state.dom())
    {
        flatten_sets(
            dsts.map(|dst: AbstractEndPoint| self.un_acked_messages_for_dest(src, dst))
        )
    }

    pub open spec fn un_acked_messages(self, src: AbstractEndPoint) -> Set<Packet> {
        self.un_acked_messages_for_dests(src, self.send_state.dom())
    }

    #[verifier::external_body]
    pub proof fn lemma_un_acked_messages_for_dests_empty(&self, src: AbstractEndPoint, dests: Set<AbstractEndPoint>)
        requires dests == Set::<AbstractEndPoint>::empty()
        ensures self.un_acked_messages_for_dests(src, dests) == Set::<Packet>::empty()
    {
        unimplemented!()
    }

}


// ========== BOUNDARY TESTS ==========
// These tests violate preconditions of the lemma. Each should be rejected.


// Test 1: Call lemma with a singleton dest set — violates requires: dests == Set::empty()
// The precondition demands empty dests, but we pass a singleton.
// SHOULD FAIL
proof fn test_boundary_singleton_dests() {
    let dst = AbstractEndPoint { id: seq![1u8] };
    let src = AbstractEndPoint { id: seq![2u8] };
    let sd = SingleDelivery::<int> {
        receive_state: Map::<AbstractEndPoint, nat>::empty(),
        send_state: Map::<AbstractEndPoint, AckState<int>>::empty(),
    };
    sd.lemma_un_acked_messages_for_dests_empty(src, set![dst]);
}


// Test 2: Call lemma with two-element dests — violates requires: dests == Set::empty()
// Even more clearly non-empty than test 1.
// SHOULD FAIL
proof fn test_boundary_two_element_dests() {
    let dst1 = AbstractEndPoint { id: seq![1u8] };
    let dst2 = AbstractEndPoint { id: seq![2u8] };
    let src = AbstractEndPoint { id: seq![3u8] };
    let sd = SingleDelivery::<int> {
        receive_state: Map::<AbstractEndPoint, nat>::empty(),
        send_state: Map::<AbstractEndPoint, AckState<int>>::empty(),
    };
    sd.lemma_un_acked_messages_for_dests_empty(src, set![dst1, dst2]);
}


// Test 3: Call lemma with arbitrary non-empty dests (universally quantified)
// Any set containing at least one element violates requires: dests == Set::empty()
// SHOULD FAIL
proof fn test_boundary_arbitrary_non_empty_dests(
    sd: SingleDelivery<int>,
    src: AbstractEndPoint,
    dests: Set<AbstractEndPoint>,
    ep: AbstractEndPoint,
)
    requires
        dests.contains(ep),
{
    sd.lemma_un_acked_messages_for_dests_empty(src, dests);
}

}
