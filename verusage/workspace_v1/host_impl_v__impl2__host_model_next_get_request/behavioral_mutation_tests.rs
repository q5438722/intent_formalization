use vstd::prelude::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Type definitions (from target file) =====

pub trait VerusClone {}
impl VerusClone for SHTKey {}

pub enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Ordering {
    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }
}

pub trait KeyTrait : Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;
}

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

impl<K: KeyTrait + VerusClone> KeyRange<K> {
    pub open spec fn is_empty(self) -> bool {
        self.lo.geq_spec(self.hi)
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

impl KeyTrait for SHTKey {
    open spec fn cmp_spec(self, other: Self) -> Ordering {
        if self.ukey < other.ukey {
            Ordering::Less
        } else if self.ukey == other.ukey {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

pub type AbstractKey = SHTKey;
pub type AbstractValue = Seq<u8>;
pub type Hashtable = Map<AbstractKey, AbstractValue>;

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

#[verifier::ext_equal]
pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: Seq<SingleMessage<MT>>,
}

pub open spec(checked) fn ack_state_lookup<MT>(src: AbstractEndPoint, send_state: SendState<MT>) -> AckState<MT> {
    if send_state.contains_key(src)
        { send_state[src] }
    else
        { AckState{num_packets_acked: 0, un_acked: Seq::empty()} }
}

pub enum SingleMessage<MT> {
    Message { seqno: nat, dst: AbstractEndPoint, m: MT },
    Ack { ack_seqno: nat },
    InvalidMessage {},
}

pub type PMsg = SingleMessage<Message>;

pub struct Packet {
    pub dst: AbstractEndPoint,
    pub src: AbstractEndPoint,
    pub msg: PMsg,
}

pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;
pub type TombstoneTable = Map<AbstractEndPoint, nat>;

#[verifier::ext_equal]
pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
}

impl<MT> SingleDelivery<MT> {
    pub open spec(checked) fn send_single_message(pre: Self, post: Self, m: MT, dst: AbstractEndPoint, sm: Option<SingleMessage<MT>>, params: AbstractParameters) -> bool {
        let old_ack_state = ack_state_lookup(dst, pre.send_state);
        let new_seqno = old_ack_state.num_packets_acked + old_ack_state.un_acked.len() + 1;
        if new_seqno > params.max_seqno {
            &&& post == pre
            &&& sm is None
        } else {
            &&& sm == Some(SingleMessage::<MT>::Message{
                    seqno: new_seqno,
                    m: m,
                    dst: dst,
                })
            &&& post == SingleDelivery {
                send_state: pre.send_state.insert(dst,
                    AckState{
                        un_acked: old_ack_state.un_acked.push(sm.unwrap()),
                        ..old_ack_state }),
                ..pre }
        }
    }
}

pub enum AppRequest {
    AppGetRequest { seqno: nat, key: AbstractKey },
    AppSetRequest { seqno: nat, key: AbstractKey, ov: Option<AbstractValue> },
}

#[verifier::ext_equal]
pub struct AbstractDelegationMap(pub Map<AbstractKey, AbstractEndPoint>);

impl AbstractDelegationMap {
    #[verifier(inline)]
    pub open spec fn view(self) -> Map<AbstractKey, AbstractEndPoint> { self.0 }

    #[verifier(inline)]
    pub open spec fn spec_index(self, key: AbstractKey) -> AbstractEndPoint
        recommends self.0.dom().contains(key)
    {
        self@.index(key)
    }

    pub open spec fn is_complete(self) -> bool { self@.dom().is_full() }
}

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

pub open spec fn max_val_len() -> int { 1024 }
pub open spec fn valid_key(key: AbstractKey) -> bool { true }
pub open spec fn valid_value(value: AbstractValue) -> bool { value.len() < max_val_len() }

pub open spec(checked) fn hashtable_lookup(h: Hashtable, k: AbstractKey) -> Option<AbstractValue> {
    if h.dom().contains(k) { Some(h[k]) } else { None }
}

pub open spec(checked) fn next_get_request_reply(pre: AbstractHostState, post: AbstractHostState, src: AbstractEndPoint, seqno: nat, k: AbstractKey, sm: SingleMessage<Message>, m: Message, out: Set<Packet>, should_send: bool) -> bool
    recommends pre.delegation_map.is_complete()
{
    let owner = pre.delegation_map[k];
    if should_send && valid_key(k) {
        &&& if owner == pre.constants.me {
                &&& m == Message::Reply{key: k, value: hashtable_lookup(pre.h, k)}
                &&& post.received_requests == pre.received_requests.push(AppRequest::AppGetRequest{seqno, key: k})
            } else {
                &&& m == Message::Redirect{key: k, id: owner}
                &&& post.received_requests == pre.received_requests
            }
        &&& SingleDelivery::send_single_message(pre.sd, post.sd, m, src, Some(sm), pre.constants.params)
        &&& sm.arrow_Message_dst() == src
        &&& out == set![ Packet{dst: src, src: pre.constants.me, msg: sm} ]
    } else {
        &&& post == AbstractHostState { received_packet: post.received_packet, ..pre }
        &&& out == Set::<Packet>::empty()
    }
}

pub open spec(checked) fn next_get_request(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
{
    &&& pkt.msg.arrow_Message_m() is GetRequest
    &&& post.delegation_map == pre.delegation_map
    &&& post.h == pre.h
    &&& post.num_delegations == pre.num_delegations
    &&& (exists |sm, m, b| next_get_request_reply(pre, post, pkt.src, pkt.msg.arrow_Message_seqno(), pkt.msg.arrow_Message_m().arrow_GetRequest_key(), sm, m, out, b))
}


// ========== BEHAVIORAL MUTATION TESTS ==========

// M1: After a valid next_get_request, assert delegation_map changed (mutated expectation).
// The spec requires post.delegation_map == pre.delegation_map, so this should fail.
// SHOULD FAIL
proof fn test_mutation_delegation_map_changes(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_get_request(pre, post, pkt, out),
{
    assert(post.delegation_map != pre.delegation_map);
}

// M2: After a valid next_get_request, assert hashtable changed (mutated expectation).
// The spec requires post.h == pre.h, so this should fail.
// SHOULD FAIL
proof fn test_mutation_hashtable_changes(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_get_request(pre, post, pkt, out),
{
    assert(post.h != pre.h);
}

// M3: After a valid next_get_request, assert num_delegations changed (mutated expectation).
// The spec requires post.num_delegations == pre.num_delegations, so this should fail.
// SHOULD FAIL
proof fn test_mutation_num_delegations_changes(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_get_request(pre, post, pkt, out),
{
    assert(post.num_delegations != pre.num_delegations);
}

// M4: When owner == me, the reply message must contain hashtable_lookup(pre.h, k).
// Here we claim reply contains a DIFFERENT value (Some of an arbitrary seq).
// SHOULD FAIL
proof fn test_mutation_wrong_reply_value(
    pre: AbstractHostState, post: AbstractHostState,
    src: AbstractEndPoint, sm: SingleMessage<Message>,
    wrong_val: AbstractValue,
)
    requires
        pre.delegation_map.is_complete(),
        pre.delegation_map[SHTKey{ukey: 7}] == pre.constants.me,
        hashtable_lookup(pre.h, SHTKey{ukey: 7}) != Some(wrong_val),
{
    let k = SHTKey { ukey: 7 };
    let bad_m = Message::Reply { key: k, value: Some(wrong_val) };
    assert(next_get_request_reply(pre, post, src, 1, k, sm, bad_m, set![ Packet{dst: src, src: pre.constants.me, msg: sm} ], true));
}

// M5: When owner != me, the spec says the message is Redirect.
// Here we claim the message is Reply instead.
// SHOULD FAIL
proof fn test_mutation_reply_when_should_redirect(
    pre: AbstractHostState, post: AbstractHostState,
    src: AbstractEndPoint, sm: SingleMessage<Message>,
    other_ep: AbstractEndPoint,
)
    requires
        pre.delegation_map.is_complete(),
        pre.delegation_map[SHTKey{ukey: 42}] != pre.constants.me,
{
    let k = SHTKey { ukey: 42 };
    let bad_m = Message::Reply { key: k, value: None };
    assert(next_get_request_reply(pre, post, src, 1, k, sm, bad_m, set![ Packet{dst: src, src: pre.constants.me, msg: sm} ], true));
}

}
