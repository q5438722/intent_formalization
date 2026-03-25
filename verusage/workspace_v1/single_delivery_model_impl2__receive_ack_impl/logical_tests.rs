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

// ========== LOGICAL TESTS ==========

// Test 1: Soundness — derive false from receive_ack.
// If the spec is consistent, valid inputs should never let us derive false.
// SHOULD FAIL
proof fn test_logical_derive_false(
    pre: SingleDelivery<Message>,
    post: SingleDelivery<Message>,
    pkt: Packet,
)
    requires
        pkt.msg is Ack,
        SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()),
{
    assert(false);
}

// Test 2: Receive state preservation — NOT guaranteed by spec.
// The spec uses `..post` for the receive_state field, making it tautological:
//   post.receive_state =~= post.receive_state (always true, no constraint).
// So receive_state is NOT tied to pre.receive_state in the > branch.
// This tests whether the spec is too weak (missing constraint).
// SHOULD FAIL
proof fn test_logical_receive_state_preserved(
    pre: SingleDelivery<Message>,
    post: SingleDelivery<Message>,
    pkt: Packet,
)
    requires
        pkt.msg is Ack,
        SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()),
        pkt.msg.arrow_Ack_ack_seqno() > ack_state_lookup(pkt.src, pre.send_state).num_packets_acked,
{
    // The implementation preserves receive_state, but the SPEC doesn't guarantee it.
    // The spec's `..post` is self-referential and does not constrain receive_state.
    assert(post.receive_state =~= pre.receive_state);
}

// Test 3: Stronger inequality — num_packets_acked strictly greater than ack_seqno.
// The spec sets num_packets_acked = ack_seqno (equal, not strictly greater).
// Asserting a stronger bound should fail.
// SHOULD FAIL
proof fn test_logical_stronger_inequality(
    pre: SingleDelivery<Message>,
    post: SingleDelivery<Message>,
    pkt: Packet,
)
    requires
        pkt.msg is Ack,
        SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()),
        pkt.msg.arrow_Ack_ack_seqno() > ack_state_lookup(pkt.src, pre.send_state).num_packets_acked,
{
    let ack_seqno = pkt.msg.arrow_Ack_ack_seqno();
    // Spec says new num_packets_acked == ack_seqno, NOT > ack_seqno.
    assert(ack_state_lookup(pkt.src, post.send_state).num_packets_acked > ack_seqno);
}

}
