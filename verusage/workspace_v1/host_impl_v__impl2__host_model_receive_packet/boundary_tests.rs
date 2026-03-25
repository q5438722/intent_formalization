use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type definitions =====

pub trait KeyTrait {}
pub trait VerusClone {}
impl VerusClone for SHTKey {}
impl KeyTrait for SHTKey {}

#[derive(Eq, PartialEq, Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

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
    Message { seqno: nat, dst: AbstractEndPoint, m: MT },
    Ack { ack_seqno: nat },
    InvalidMessage {},
}

pub type PMsg = SingleMessage<Message>;
pub type AckList<MT> = Seq<SingleMessage<MT>>;
pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;
pub type TombstoneTable = Map<AbstractEndPoint, nat>;

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

pub enum AppRequest {
    AppGetRequest { seqno: nat, key: AbstractKey },
    AppSetRequest { seqno: nat, key: AbstractKey, ov: Option<AbstractValue> },
}

#[verifier::ext_equal]
pub struct AbstractDelegationMap(pub Map<AbstractKey, AbstractEndPoint>);

pub struct AbstractConstants {
    pub root_identity: AbstractEndPoint,
    pub host_ids: Seq<AbstractEndPoint>,
    pub params: AbstractParameters,
    pub me: AbstractEndPoint,
}

pub struct AbstractHostState {
    pub constants: AbstractConstants,
    pub delegation_map: AbstractDelegationMap,
    pub h: Hashtable,
    pub sd: SingleDelivery<Message>,
    pub received_packet: Option<Packet>,
    pub num_delegations: int,
    pub received_requests: Seq<AppRequest>,
}

// ===== Spec functions =====

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
    if send_state.contains_key(src) { send_state[src] }
    else { AckState { num_packets_acked: 0, un_acked: Seq::empty() } }
}

impl<MT> SingleDelivery<MT> {

    pub open spec(checked) fn new_single_message(self, pkt: Packet) -> bool {
        let last_seqno = tombstone_table_lookup(pkt.src, self.receive_state);
        &&& pkt.msg is Message
        &&& pkt.msg.arrow_Message_seqno() == last_seqno + 1
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
                    ..old_ack_state
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
        recommends self.should_ack_single_message(pkt),
    {
        &&& ack.msg is Ack
        &&& ack.msg.arrow_Ack_ack_seqno() == pkt.msg.arrow_Message_seqno()
        &&& ack.src == pkt.dst
        &&& ack.dst == pkt.src
        &&& acks == set![ack]
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

#[verifier::opaque]
pub open spec fn okay_to_ignore_packets() -> bool {
    true
}

pub open spec(checked) fn receive_packet(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>, ack: Packet) -> bool {
    ||| {
        &&& pre.received_packet is None
        &&& SingleDelivery::receive(pre.sd, post.sd, pkt, ack, out)
        &&& if SingleDelivery::new_single_message(pre.sd, pkt) {
                post.received_packet == Some(pkt)
            } else {
                post.received_packet is None
            }
        &&& post == AbstractHostState { sd: post.sd, received_packet: post.received_packet, ..post }
    }
    ||| {
        &&& pre.received_packet is Some || okay_to_ignore_packets()
        &&& post == pre
        &&& out == Set::<Packet>::empty()
    }
}


// ========== BOUNDARY TESTS ==========

// B1: InvalidMessage should never cause buffering.
// An InvalidMessage packet cannot result in post.received_packet being Some.
// SHOULD FAIL
proof fn test_boundary_invalid_msg_buffered(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, ack: Packet, out: Set<Packet>)
    requires
        pkt.msg is InvalidMessage,
        pre.received_packet is None,
        receive_packet(pre, post, pkt, out, ack),
{
    // InvalidMessage → new_single_message is false → received_packet is None in both disjuncts
    assert(post.received_packet is Some);
}

// B2: When buffer is full (received_packet is Some), receive_packet cannot
// produce a state that differs from pre (second disjunct forces post == pre).
// SHOULD FAIL
proof fn test_boundary_full_buffer_state_change(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, ack: Packet, out: Set<Packet>)
    requires
        pre.received_packet is Some,
        post.received_packet is None,
{
    // Second disjunct: post == pre → post.received_packet == Some (contradicts requires)
    // First disjunct: pre.received_packet is None → contradicts requires
    assert(receive_packet(pre, post, pkt, out, ack));
}

// B3: okay_to_ignore_packets is opaque; the verifier cannot prove it's true.
// SHOULD FAIL
proof fn test_boundary_okay_to_ignore_opaque()
{
    assert(okay_to_ignore_packets());
}

// B4: A fresh Message packet with empty buffer must be buffered as exactly that packet.
// Asserting it becomes a DIFFERENT packet should fail.
// SHOULD FAIL
proof fn test_boundary_receive_wrong_packet(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, ack: Packet, out: Set<Packet>)
    requires
        pre.received_packet is None,
        pkt.msg is Message,
        pkt.msg.arrow_Message_seqno() == tombstone_table_lookup(pkt.src, pre.sd.receive_state) + 1,
        receive_packet(pre, post, pkt, out, ack),
        post.received_packet is Some,
{
    // First disjunct: new_single_message ⇒ post.received_packet == Some(pkt)
    // (second disjunct gives None, ruled out by post.received_packet is Some)
    assert(post.received_packet != Some(pkt));
}

// B5: AbstractParameters::static_params() has specific fixed values.
// A different set of parameters should not equal static_params().
// SHOULD FAIL
proof fn test_boundary_params_mismatch()
{
    let p = AbstractParameters { max_seqno: 0xffff_ffff_ffff_ffff as nat, max_delegations: 3 };
    assert(p == AbstractParameters::static_params());
}

}
