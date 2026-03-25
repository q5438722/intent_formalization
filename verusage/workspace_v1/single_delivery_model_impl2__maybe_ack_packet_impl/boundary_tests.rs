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

pub open spec fn tombstone_table_lookup(src: AbstractEndPoint, t: TombstoneTable) -> nat {
    if t.dom().contains(src) { t[src] } else { 0 }
}

pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: Seq<SingleMessage<MT>>,
}

pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;

pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

impl<MT> SingleDelivery<MT> {
    pub open spec fn init() -> Self {
        SingleDelivery { receive_state: Map::empty(), send_state: Map::empty() }
    }

    pub open spec(checked) fn maybe_ack_packet(pre: Self, pkt: Packet, ack: Packet, acks: Set<Packet>) -> bool {
        if pre.should_ack_single_message(pkt) {
            pre.send_ack(pkt, ack, acks)
        } else {
            acks.is_empty()
        }
    }

    pub open spec(checked) fn should_ack_single_message(self, pkt: Packet) -> bool {
        &&& pkt.msg is Message
        &&& {
            let last_seqno = tombstone_table_lookup(pkt.src, self.receive_state);
            pkt.msg.arrow_Message_seqno() <= last_seqno
        }
    }

    pub open spec(checked) fn send_ack(self, pkt: Packet, ack: Packet, acks: Set<Packet>) -> bool
        recommends self.should_ack_single_message(pkt),
    {
        &&& ack.msg is Ack
        &&& ack.msg.arrow_Ack_ack_seqno() == pkt.msg.arrow_Message_seqno()
        &&& ack.src == pkt.dst
        &&& ack.dst == pkt.src
        &&& acks == set![ack]
    }
}

// ========== BOUNDARY TESTS ==========

// Test 1: pkt.msg is Ack (not Message) — violates the exec precondition pkt.msg is Message.
// should_ack is false (pkt.msg is not Message), so maybe_ack_packet requires acks.is_empty().
// We provide non-empty acks, so this should be rejected.
// SHOULD FAIL
proof fn test_boundary_ack_not_message_with_nonempty_acks() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let pkt = Packet { src, dst, msg: SingleMessage::Ack { ack_seqno: 5 as nat } };
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Ack { ack_seqno: 5 as nat } };
    let acks: Set<Packet> = set![ack];
    // pkt.msg is Ack, not Message => should_ack false => requires acks.is_empty()
    // But acks is non-empty => maybe_ack_packet is false
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

// Test 2: seqno just above tombstone value (edge case at the boundary).
// receive_state maps src -> 5, pkt.seqno = 6. should_ack is false (6 > 5).
// maybe_ack_packet requires acks.is_empty(), but we provide non-empty acks.
// SHOULD FAIL
proof fn test_boundary_seqno_exceeds_tombstone() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, 5 as nat),
        send_state: Map::empty(),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 6 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Ack { ack_seqno: 6 as nat } };
    let acks: Set<Packet> = set![ack];
    // seqno(6) > tombstone(5) => should_ack false => requires acks.is_empty()
    // But acks is non-empty => maybe_ack_packet is false
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

// Test 3: seqno=0 with empty tombstone — should_ack IS true (0 <= 0).
// But we provide empty acks. send_ack requires acks == set![ack], not empty.
// SHOULD FAIL
proof fn test_boundary_empty_acks_when_should_ack() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 0 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Ack { ack_seqno: 0 as nat } };
    let acks: Set<Packet> = Set::empty();
    // should_ack true (0 <= 0) => send_ack required => acks must equal set![ack]
    // But acks is empty => send_ack is false => maybe_ack_packet is false
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

}
