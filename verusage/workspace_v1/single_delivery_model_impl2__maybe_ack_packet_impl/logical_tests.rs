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

// ========== LOGICAL TESTS ==========

// Test 1: should_ack does NOT imply seqno > 0.
// With seqno=0 and empty tombstone, tombstone_table_lookup returns 0, so 0 <= 0 is true.
// The property "should_ack implies seqno > 0" is NOT entailed.
// SHOULD FAIL
proof fn test_should_ack_implies_nonzero_seqno() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> { receive_state: Map::empty(), send_state: Map::empty() };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 0 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    // should_ack is true (0 <= 0), but seqno is 0 (not > 0)
    // This implication is false: true ==> false
    assert(pre.should_ack_single_message(pkt) ==> pkt.msg.arrow_Message_seqno() > 0);
}

// Test 2: should_ack is NOT upward-monotone in seqno.
// If tombstone is 5, should_ack holds for seqno=5 but NOT for seqno=6.
// The property "should_ack(seqno=5) ==> should_ack(seqno=6)" is NOT entailed.
// SHOULD FAIL
proof fn test_should_ack_upward_monotonicity() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, 5 as nat),
        send_state: Map::empty(),
    };
    let pkt_lo = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 5 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    let pkt_hi = Packet { src, dst, msg: SingleMessage::Message {
        seqno: 6 as nat, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    // should_ack(pkt_lo) is true (5 <= 5), should_ack(pkt_hi) is false (6 > 5)
    // Implication is false: true ==> false
    assert(pre.should_ack_single_message(pkt_lo) ==> pre.should_ack_single_message(pkt_hi));
}

// Test 3: The ack_seqno is NOT guaranteed to be strictly less than max_seqno.
// When pkt.seqno = max_seqno and tombstone = max_seqno, maybe_ack_packet holds
// with ack_seqno = max_seqno. The property "ack_seqno < max_seqno" is NOT entailed.
// SHOULD FAIL
proof fn test_ack_seqno_bounded_by_max() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };
    let max_seqno: nat = 0xffff_ffff_ffff_ffff as nat;
    let pre = SingleDelivery::<Message> {
        receive_state: Map::<AbstractEndPoint, nat>::empty().insert(src, max_seqno),
        send_state: Map::empty(),
    };
    let pkt = Packet { src, dst, msg: SingleMessage::Message {
        seqno: max_seqno, dst: dst, m: Message::GetRequest { key: SHTKey { ukey: 0 } }
    }};
    let ack = Packet { src: dst, dst: src, msg: SingleMessage::Ack { ack_seqno: max_seqno } };
    let acks: Set<Packet> = set![ack];
    // maybe_ack_packet holds (seqno=max <= tombstone=max, send_ack all conditions met)
    // But ack_seqno equals max_seqno, not strictly less
    assert(
        SingleDelivery::<Message>::maybe_ack_packet(pre, pkt, ack, acks) ==>
        ack.msg.arrow_Ack_ack_seqno() < AbstractParameters::static_params().max_seqno
    );
}

}
