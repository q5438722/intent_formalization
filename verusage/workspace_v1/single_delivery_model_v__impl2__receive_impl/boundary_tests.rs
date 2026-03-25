extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;

fn main() {}

verus! {

// === Minimal Type Definitions (spec-level only) ===

pub trait KeyTrait {}
pub trait VerusClone {}

#[derive(Eq, PartialEq, Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

impl VerusClone for SHTKey {}
impl KeyTrait for SHTKey {}

pub type AbstractKey = SHTKey;
pub type AbstractValue = Seq<u8>;
pub type Hashtable = Map<AbstractKey, AbstractValue>;

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

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

pub enum Message {
    GetRequest { key: AbstractKey },
    SetRequest { key: AbstractKey, value: Option<AbstractValue> },
    Reply { key: AbstractKey, value: Option<AbstractValue> },
    Redirect { key: AbstractKey, id: AbstractEndPoint },
    Shard { range: KeyRange<AbstractKey>, recipient: AbstractEndPoint },
    Delegate { range: KeyRange<AbstractKey>, h: Hashtable },
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

pub type PMsg = SingleMessage<Message>;
pub type AckList<MT> = Seq<SingleMessage<MT>>;
pub type TombstoneTable = Map<AbstractEndPoint, nat>;
pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;

pub struct Packet {
    pub dst: AbstractEndPoint,
    pub src: AbstractEndPoint,
    pub msg: PMsg,
}

#[verifier::ext_equal]
pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: AckList<MT>,
}

#[verifier::ext_equal]
pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

// === Spec Functions ===

pub open spec fn tombstone_table_lookup(src: AbstractEndPoint, t: TombstoneTable) -> nat {
    if t.dom().contains(src) { t[src] } else { 0 }
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

pub open spec(checked) fn ack_state_lookup<MT>(src: AbstractEndPoint, send_state: SendState<MT>) -> AckState<MT> {
    if send_state.contains_key(src)
        { send_state[src] }
    else
        { AckState { num_packets_acked: 0, un_acked: Seq::empty() } }
}

impl<MT> SingleDelivery<MT> {

    pub open spec(checked) fn new_single_message(self, pkt: Packet) -> bool {
        let last_seqno = tombstone_table_lookup(pkt.src, self.receive_state);
        &&& pkt.msg is Message
        &&& pkt.msg.arrow_Message_seqno() == last_seqno + 1
    }

    pub open spec(checked) fn receive_ack(pre: Self, post: Self, pkt: Packet, acks: Set<Packet>) -> bool
        recommends pkt.msg is Ack
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

    pub open spec(checked) fn receive_real_packet(self, post: Self, pkt: Packet) -> bool {
        if self.new_single_message(pkt) {
            let last_seqno = tombstone_table_lookup(pkt.src, self.receive_state);
            post == Self { receive_state: self.receive_state.insert(pkt.src, (last_seqno + 1) as nat), ..self }
        } else {
            post == self
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
        recommends self.should_ack_single_message(pkt)
    {
        &&& ack.msg is Ack
        &&& ack.msg.arrow_Ack_ack_seqno() == pkt.msg.arrow_Message_seqno()
        &&& ack.src == pkt.dst
        &&& ack.dst == pkt.src
        &&& acks == set![ ack ]
    }

    pub open spec(checked) fn maybe_ack_packet(pre: Self, pkt: Packet, ack: Packet, acks: Set<Packet>) -> bool {
        if pre.should_ack_single_message(pkt) {
            pre.send_ack(pkt, ack, acks)
        } else {
            acks.is_empty()
        }
    }

    pub open spec(checked) fn receive(pre: Self, post: Self, pkt: Packet, ack: Packet, acks: Set<Packet>) -> bool {
        match pkt.msg {
            SingleMessage::Ack { ack_seqno: _ } => Self::receive_ack(pre, post, pkt, acks),
            SingleMessage::Message { seqno, dst: _, m } => {
                &&& Self::receive_real_packet(pre, post, pkt)
                &&& Self::maybe_ack_packet(post, pkt, ack, acks)
            }
            SingleMessage::InvalidMessage {} => {
                &&& post === pre
                &&& acks === Set::empty()
            }
        }
    }

}


// ============================================================
// BOUNDARY TESTS: Violate preconditions and edge cases
// ============================================================

// B1: InvalidMessage should produce empty acks — asserting non-empty should fail
// SHOULD FAIL
proof fn test_boundary_invalid_msg_nonempty_acks() {
    let pre: SingleDelivery<Message> = arbitrary();
    let post: SingleDelivery<Message> = arbitrary();
    let pkt: Packet = arbitrary();
    let ack: Packet = arbitrary();
    let acks: Set<Packet> = arbitrary();

    assume(pkt.msg is InvalidMessage);
    assume(SingleDelivery::receive(pre, post, pkt, ack, acks));

    // receive with InvalidMessage ensures acks === Set::empty()
    // Asserting acks is non-empty should FAIL
    assert(!acks.is_empty());
}

// B2: Ack packets must produce empty ack set — asserting non-empty should fail
// SHOULD FAIL
proof fn test_boundary_ack_produces_nonempty_acks() {
    let pre: SingleDelivery<Message> = arbitrary();
    let post: SingleDelivery<Message> = arbitrary();
    let pkt: Packet = arbitrary();
    let ack: Packet = arbitrary();
    let acks: Set<Packet> = arbitrary();

    assume(pkt.msg is Ack);
    assume(SingleDelivery::receive(pre, post, pkt, ack, acks));

    // receive_ack enforces acks.is_empty()
    // Asserting acks is non-empty should FAIL
    assert(!acks.is_empty());
}

// B3: new_single_message with wrong seqno (5 instead of 1) on empty receive_state
// SHOULD FAIL
proof fn test_boundary_wrong_seqno_is_new() {
    let sd: SingleDelivery<Message> = arbitrary();
    let msg_content: Message = arbitrary();
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };

    assume(tombstone_table_lookup(src, sd.receive_state) == 0);

    // With tombstone 0, new_single_message requires seqno == 1
    let pkt = Packet {
        src: src, dst: dst,
        msg: SingleMessage::Message { seqno: 5 as nat, dst: dst, m: msg_content }
    };

    // seqno 5 != 0 + 1 = 1, so this should NOT be a new message
    assert(sd.new_single_message(pkt));
}

// B4: should_ack_single_message should be false when seqno > last_seqno
// SHOULD FAIL
proof fn test_boundary_future_seqno_should_ack() {
    let sd: SingleDelivery<Message> = arbitrary();
    let msg_content: Message = arbitrary();
    let src = AbstractEndPoint { id: seq![1u8] };
    let dst = AbstractEndPoint { id: seq![2u8] };

    assume(tombstone_table_lookup(src, sd.receive_state) == 0);

    // seqno 1 > last_seqno 0, so should_ack should be false
    let pkt = Packet {
        src: src, dst: dst,
        msg: SingleMessage::Message { seqno: 1 as nat, dst: dst, m: msg_content }
    };

    assert(sd.should_ack_single_message(pkt));
}

// B5: tombstone_table_lookup on absent key returns 0, not > 0
// SHOULD FAIL
proof fn test_boundary_tombstone_absent_nonzero() {
    let src = AbstractEndPoint { id: seq![1u8] };
    let t: TombstoneTable = Map::empty();

    // Empty map lookup returns 0
    assert(tombstone_table_lookup(src, t) > 0);
}


} // verus!
