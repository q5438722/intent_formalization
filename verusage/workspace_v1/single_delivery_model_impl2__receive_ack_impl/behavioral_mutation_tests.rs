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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Wrong num_packets_acked after ack update.
// The spec sets num_packets_acked = ack_seqno. Asserting it equals ack_seqno + 1 is wrong.
// SHOULD FAIL
proof fn test_mutation_wrong_num_packets_acked(
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
    // The spec says new num_packets_acked = ack_seqno.
    // Asserting ack_seqno + 1 (a mutated/wrong value) should fail.
    assert(ack_state_lookup(pkt.src, post.send_state).num_packets_acked == ack_seqno + 1);
}

// Test 2: State changes when no update is expected.
// When ack_seqno <= num_packets_acked, spec says post == pre.
// Asserting the send_state lost a key contradicts post == pre.
// SHOULD FAIL
proof fn test_mutation_state_changed_when_noop(
    pre: SingleDelivery<Message>,
    post: SingleDelivery<Message>,
    pkt: Packet,
)
    requires
        pkt.msg is Ack,
        SingleDelivery::<Message>::receive_ack(pre, post, pkt, Set::empty()),
        pkt.msg.arrow_Ack_ack_seqno() <= ack_state_lookup(pkt.src, pre.send_state).num_packets_acked,
        pre.send_state.contains_key(pkt.src),
{
    // Spec says post == pre when ack_seqno <= num_packets_acked.
    // Since pre has pkt.src in send_state, post must too.
    // Asserting it doesn't should fail.
    assert(!post.send_state.contains_key(pkt.src));
}

// Test 3: Assert acks set produced by receive_ack is non-empty.
// The spec always requires acks.is_empty(), so this should fail.
// SHOULD FAIL
proof fn test_mutation_acks_nonempty(
    pre: SingleDelivery<Message>,
    post: SingleDelivery<Message>,
    pkt: Packet,
    acks: Set<Packet>,
)
    requires
        pkt.msg is Ack,
        SingleDelivery::<Message>::receive_ack(pre, post, pkt, acks),
{
    // receive_ack requires acks.is_empty() as its first conjunct.
    // Asserting the opposite should fail.
    assert(!acks.is_empty());
}

}
