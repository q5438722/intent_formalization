use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type definitions (from target spec) =====

pub struct SHTKey { pub ukey: u64 }
pub type AbstractKey = SHTKey;
pub type AbstractValue = Seq<u8>;

pub struct AbstractEndPoint { pub id: Seq<u8> }
impl AbstractEndPoint {
    pub open spec fn valid_physical_address(self) -> bool { self.id.len() < 0x100000 }
    pub open spec fn abstractable(self) -> bool { self.valid_physical_address() }
}

pub trait KeyTrait : Sized {}
pub trait VerusClone : Sized {}
pub struct KeyRange<K: KeyTrait + VerusClone> { pub lo: KeyIterator<K>, pub hi: KeyIterator<K> }
pub struct KeyIterator<K: KeyTrait + VerusClone> { pub k: Option<K> }
impl<K: VerusClone + KeyTrait> VerusClone for KeyIterator<K> {}
impl<K: VerusClone + KeyTrait> VerusClone for KeyRange<K> {}
impl KeyTrait for SHTKey {}
impl VerusClone for SHTKey {}

pub type Hashtable = Map<AbstractKey, AbstractValue>;

pub enum Message {
    GetRequest { key: AbstractKey },
    SetRequest { key: AbstractKey, value: Option<AbstractValue> },
    Reply { key: AbstractKey, value: Option<AbstractValue> },
    Redirect { key: AbstractKey, id: AbstractEndPoint },
    Shard { range: KeyRange<AbstractKey>, recipient: AbstractEndPoint },
    Delegate { range: KeyRange<AbstractKey>, h: Hashtable },
}

pub enum SingleMessage<MT> {
    Message { seqno: nat, dst: AbstractEndPoint, m: MT },
    Ack { ack_seqno: nat },
    InvalidMessage {},
}

pub struct Packet {
    pub dst: AbstractEndPoint,
    pub src: AbstractEndPoint,
    pub msg: SingleMessage<Message>,
}

pub type TombstoneTable = Map<AbstractEndPoint, nat>;
pub type AckList<MT> = Seq<SingleMessage<MT>>;

pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: AckList<MT>,
}

pub open spec(checked) fn truncate_un_ack_list<MT>(un_acked: AckList<MT>, seqno_acked: nat) -> Seq<SingleMessage<MT>>
    decreases un_acked.len()
{
    if un_acked.len() > 0 && un_acked[0] is Message && un_acked[0].arrow_Message_seqno() <= seqno_acked {
        truncate_un_ack_list(un_acked.skip(1), seqno_acked)
    } else {
        un_acked
    }
}

pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;

pub open spec(checked) fn ack_state_lookup<MT>(src: AbstractEndPoint, send_state: SendState<MT>) -> AckState<MT> {
    if send_state.contains_key(src) { send_state[src] }
    else { AckState { num_packets_acked: 0, un_acked: Seq::empty() } }
}

pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

impl<MT> SingleDelivery<MT> {
    pub open spec fn init() -> Self {
        SingleDelivery { receive_state: Map::empty(), send_state: Map::empty() }
    }

    pub open spec(checked) fn receive_ack(pre: Self, post: Self, pkt: Packet, acks: Set<Packet>) -> bool
        recommends pkt.msg is Ack,
    {
        &&& acks.is_empty()
        &&& {
            let old_ack_state = ack_state_lookup(pkt.src, pre.send_state);
            if pkt.msg.arrow_Ack_ack_seqno() > old_ack_state.num_packets_acked {
                let new_ack_state = AckState {
                    num_packets_acked: pkt.msg.arrow_Ack_ack_seqno(),
                    un_acked: truncate_un_ack_list(old_ack_state.un_acked, pkt.msg.arrow_Ack_ack_seqno()),
                    .. old_ack_state
                };
                post =~= Self { send_state: pre.send_state.insert(pkt.src, new_ack_state), ..post }
            } else {
                post == pre
            }
        }
    }
}

// ========== BOUNDARY TESTS ==========

// Test 1: Non-empty acks set — receive_ack requires acks.is_empty().
// Passing a non-empty acks set violates the first conjunct.
// SHOULD FAIL
proof fn test_boundary_nonempty_acks() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let post = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let pkt = Packet { src, dst, msg: SingleMessage::Ack { ack_seqno: 0 as nat } };
    let ack_pkt = Packet {
        src: AbstractEndPoint { id: seq![2u8] },
        dst: AbstractEndPoint { id: seq![1u8] },
        msg: SingleMessage::Ack { ack_seqno: 0 as nat },
    };
    let acks: Set<Packet> = set![ack_pkt];
    // acks is non-empty, so receive_ack should be false.
    assert(SingleDelivery::<Message>::receive_ack(pre, post, pkt, acks));
}

// Test 2: pkt.msg is InvalidMessage — violates the recommends (pkt.msg is Ack).
// The spec body uses arrow_Ack_ack_seqno() which is undefined for InvalidMessage.
// SHOULD FAIL
proof fn test_boundary_invalid_message_type() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let post = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let pkt = Packet { src, dst, msg: SingleMessage::InvalidMessage {} };
    assert(SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()));
}

// Test 3: post has different send_state at the equality boundary.
// When ack_seqno (0) == num_packets_acked (0), the else branch requires post == pre.
// But post has a different send_state, so this should fail.
// SHOULD FAIL
proof fn test_boundary_post_differs_at_equality_edge() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let extra_ack = AckState::<Message> { num_packets_acked: 42, un_acked: Seq::empty() };
    let post = SingleDelivery::<Message> {
        receive_state: Map::empty(),
        send_state: Map::<AbstractEndPoint, AckState<Message>>::empty().insert(src, extra_ack),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Ack { ack_seqno: 0 as nat } };
    // ack_seqno (0) is NOT > num_packets_acked (0), so post == pre is required.
    // But post.send_state != pre.send_state, so this should fail.
    assert(SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()));
}

}
