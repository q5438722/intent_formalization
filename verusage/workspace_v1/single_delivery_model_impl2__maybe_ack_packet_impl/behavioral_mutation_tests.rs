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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutated ack_seqno — ack has wrong seqno (99 instead of 5).
// send_ack requires ack.msg.ack_seqno == pkt.msg.seqno.
// SHOULD FAIL
proof fn test_mutation_wrong_ack_seqno() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, 10 as nat),
        send_state: Map::empty(),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 5 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    // Mutated: ack_seqno is 99, not 5
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Ack { ack_seqno: 99 as nat } };
    let acks: Set<Packet> = set![ack];
    // should_ack true (5 <= 10), but send_ack fails: 99 != 5
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

// Test 2: Mutated ack source — ack.src = pkt.src instead of pkt.dst.
// send_ack requires ack.src == pkt.dst, not pkt.src.
// SHOULD FAIL
proof fn test_mutation_wrong_ack_source() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, 10 as nat),
        send_state: Map::empty(),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 5 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    // Mutated: ack.src = pkt.src (should be pkt.dst)
    let ack = Packet { src: src, dst: dst, msg: SingleMessage::Ack { ack_seqno: 5 as nat } };
    let acks: Set<Packet> = set![ack];
    // should_ack true, but send_ack fails: ack.src (src) != pkt.dst (dst)
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

// Test 3: Mutated ack message type — ack.msg is Message instead of Ack.
// send_ack requires ack.msg is Ack.
// SHOULD FAIL
proof fn test_mutation_ack_is_message_type() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, 10 as nat),
        send_state: Map::empty(),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 5 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    // Mutated: ack.msg is Message (should be Ack)
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Message {
        seqno: 5 as nat, dst: src, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    let acks: Set<Packet> = set![ack];
    // should_ack true, but send_ack fails: ack.msg is Message, not Ack
    assert(SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks));
}

}
