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


// ========== BEHAVIORAL MUTATION TESTS ==========
// Valid inputs but assert wrong outputs/relations. Each should be rejected.


// Test 1: Call lemma with valid empty dests, then assert a packet is in the result.
// The postcondition establishes result == Set::empty(), so no packet should be in it.
// SHOULD FAIL
proof fn test_mutation_negate_empty_result(sd: SingleDelivery<int>, src: AbstractEndPoint) {
    let dests = Set::<AbstractEndPoint>::empty();
    sd.lemma_un_acked_messages_for_dests_empty(src, dests);
    // The lemma says: sd.un_acked_messages_for_dests(src, dests) == Set::empty()
    // Asserting a specific element is in the result contradicts the postcondition.
    let pkt = Packet {
        src: src,
        dst: AbstractEndPoint { id: seq![1u8] },
        msg: SingleMessage::<int>::Ack { ack_seqno: 0 },
    };
    assert(sd.un_acked_messages_for_dests(src, dests).contains(pkt));  // SHOULD FAIL
}


// Test 2: Assert a packet with WRONG src is in un_acked_messages_for_dest.
// The spec requires p.src == src for membership. A packet with different src is excluded.
// SHOULD FAIL
proof fn test_mutation_wrong_src_in_result() {
    let dst = AbstractEndPoint { id: seq![1u8] };
    let src = AbstractEndPoint { id: seq![2u8] };
    let wrong_src = AbstractEndPoint { id: seq![3u8] };
    let msg = SingleMessage::<int>::Message { seqno: 1, dst: dst, m: 42 };
    let ack = AckState::<int> { num_packets_acked: 0, un_acked: seq![msg] };
    let sd = SingleDelivery::<int> {
        receive_state: Map::<AbstractEndPoint, nat>::empty(),
        send_state: Map::<AbstractEndPoint, AckState<int>>::empty().insert(dst, ack),
    };
    // Packet with wrong_src should NOT be in the result (spec requires p.src == src)
    let wrong_pkt = Packet { src: wrong_src, dst: dst, msg: msg };
    assert(sd.un_acked_messages_for_dest(src, dst).contains(wrong_pkt));  // SHOULD FAIL
}


// Test 3: Assert that count=0 gives a non-empty set.
// With count=0, the range 0 <= i < 0 is empty, so no index satisfies the existential.
// The resulting set must be empty.
// SHOULD FAIL
proof fn test_mutation_count_zero_non_empty() {
    let dst = AbstractEndPoint { id: seq![1u8] };
    let src = AbstractEndPoint { id: seq![2u8] };
    let msg = SingleMessage::<int>::Message { seqno: 1, dst: dst, m: 42 };
    let ack = AckState::<int> { num_packets_acked: 0, un_acked: seq![msg] };
    let sd = SingleDelivery::<int> {
        receive_state: Map::<AbstractEndPoint, nat>::empty(),
        send_state: Map::<AbstractEndPoint, AckState<int>>::empty().insert(dst, ack),
    };
    let pkt = Packet { src: src, dst: dst, msg: msg };
    // count=0 means the set is empty, asserting a packet is in it should fail
    assert(sd.un_acked_messages_for_dest_up_to(src, dst, 0).contains(pkt));  // SHOULD FAIL
}

}
