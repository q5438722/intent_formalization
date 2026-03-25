use vstd::prelude::*;

fn main() {}

verus!{

// ===== Key/Ordering types =====

pub trait KeyTrait : Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;
}

#[derive(Structural, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Ordering {
    pub open spec fn lt(self) -> bool {
        self == Ordering::Less
    }
}

pub trait VerusClone {}

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
        || (!self.k.is_None() && !other.k.is_None()
            && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }

    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

impl<K: KeyTrait + VerusClone> KeyRange<K> {
    pub open spec fn contains(self, k: K) -> bool {
        KeyIterator::<K>::between(self.lo, KeyIterator::<K>::new_spec(k), self.hi)
    }

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
        if self.ukey < other.ukey { Ordering::Less }
        else if self.ukey == other.ukey { Ordering::Equal }
        else { Ordering::Greater }
    }
}

impl VerusClone for SHTKey {}

pub type AbstractKey = SHTKey;
pub type Hashtable = Map<AbstractKey, AbstractValue>;
pub type AbstractValue = Seq<u8>;

// ===== Endpoint =====

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

// ===== Parameters =====

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

// ===== IO types =====

pub struct LPacket<IdType, MessageType> {
    pub dst: IdType,
    pub src: IdType,
    pub msg: MessageType,
}

pub enum LIoOp<IdType, MessageType> {
    Send{s: LPacket<IdType, MessageType>},
    Receive{r: LPacket<IdType, MessageType>},
    TimeoutReceive{},
    ReadClock{t: int},
}

// ===== Messages =====

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
    pub msg: PMsg,
}

pub type PMsg = SingleMessage<Message>;
pub type LSHTPacket = LPacket<AbstractEndPoint, SingleMessage<Message>>;
pub type LSHTIo = LIoOp<AbstractEndPoint, SingleMessage<Message>>;
pub type AbstractIos = Seq<LSHTIo>;
pub type AckList<MT> = Seq<SingleMessage<MT>>;
pub type SendState<MT> = Map<AbstractEndPoint, AckState<MT>>;
pub type TombstoneTable = Map<AbstractEndPoint, nat>;

// ===== Service types =====

pub enum AppRequest {
    AppGetRequest{seqno: nat, key: AbstractKey},
    AppSetRequest{seqno: nat, key: AbstractKey, ov: Option<AbstractValue>},
}

// ===== SingleDelivery =====

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

#[verifier::ext_equal]
pub struct AckState<MT> {
    pub num_packets_acked: nat,
    pub un_acked: AckList<MT>,
}

pub open spec(checked) fn ack_state_lookup<MT>(src: AbstractEndPoint, send_state: SendState<MT>) -> AckState<MT> {
    if send_state.contains_key(src) { send_state[src] }
    else { AckState{num_packets_acked: 0, un_acked: Seq::empty()} }
}

#[verifier::ext_equal]
pub struct SingleDelivery<MT> {
    pub receive_state: TombstoneTable,
    pub send_state: SendState<MT>,
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
                let new_ack_state = AckState{
                    num_packets_acked: pkt.msg.arrow_Ack_ack_seqno(),
                    un_acked: truncate_un_ack_list(old_ack_state.un_acked, pkt.msg.arrow_Ack_ack_seqno()),
                    ..old_ack_state
                };
                post =~= Self{ send_state: pre.send_state.insert(pkt.src, new_ack_state), ..post }
            } else {
                post == pre
            }
        }
    }

    pub open spec(checked) fn receive_real_packet(self, post: Self, pkt: Packet) -> bool {
        if self.new_single_message(pkt) {
            let last_seqno = tombstone_table_lookup(pkt.src, self.receive_state);
            post == Self{ receive_state: self.receive_state.insert(pkt.src, (last_seqno + 1) as nat), ..self }
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
            SingleMessage::Ack{ack_seqno: _} => Self::receive_ack(pre, post, pkt, acks),
            SingleMessage::Message{seqno, dst: _, m} => {
                &&& Self::receive_real_packet(pre, post, pkt)
                &&& Self::maybe_ack_packet(post, pkt, ack, acks)
            }
            SingleMessage::InvalidMessage{} => {
                &&& post === pre
                &&& acks === Set::empty()
            }
        }
    }

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

    pub open spec(checked) fn receive_no_message(pre: Self, post: Self) -> bool {
        post.receive_state == pre.receive_state
    }

    pub open spec(checked) fn send_no_message(pre: Self, post: Self) -> bool {
        post.send_state == pre.send_state
    }
}

impl SingleDelivery<Message> {
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
                &&& p.dst == p.msg.arrow_Message_dst()
            }
        })
    }

    pub open spec(checked) fn un_acked_messages_for_dest(self, src: AbstractEndPoint, dst: AbstractEndPoint) -> Set<Packet>
        recommends self.send_state.contains_key(dst)
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
}

pub open spec fn flatten_sets<A>(sets: Set<Set<A>>) -> Set<A> {
    Set::new(|a: A| (exists |s: Set<A>| sets.contains(s) && s.contains(a)))
}

// ===== Delegation Map =====

#[verifier::ext_equal]
pub struct AbstractDelegationMap(pub Map<AbstractKey, AbstractEndPoint>);

impl AbstractDelegationMap {
    #[verifier(inline)]
    pub open spec fn view(self) -> Map<AbstractKey, AbstractEndPoint> { self.0 }

    #[verifier(inline)]
    pub open spec fn spec_index(self, key: AbstractKey) -> AbstractEndPoint
        recommends self.0.dom().contains(key)
    { self@.index(key) }

    pub open spec fn is_complete(self) -> bool { self@.dom().is_full() }

    pub open spec fn update(self, newkr: KeyRange<AbstractKey>, host: AbstractEndPoint) -> Self
        recommends self.is_complete()
    {
        AbstractDelegationMap(self@.union_prefer_right(Map::new(|k| newkr.contains(k), |k| host)))
    }

    pub open spec fn delegate_for_key_range_is_host(self, kr: KeyRange<AbstractKey>, id: AbstractEndPoint) -> bool
        recommends self.is_complete()
    {
        forall |k: AbstractKey| #[trigger] kr.contains(k) ==> self[k] == id
    }
}

// ===== Host State =====

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

impl AbstractHostState {
    pub open spec(checked) fn wf(self) -> bool {
        self.delegation_map.is_complete()
    }
}

// ===== App interface =====

pub open spec fn max_val_len() -> int { 1024 }

pub open spec fn valid_key(key: AbstractKey) -> bool { true }

pub open spec fn valid_value(value: AbstractValue) -> bool { value.len() < max_val_len() }

pub open spec fn max_hashtable_size() -> int { 62 }

pub open spec fn valid_hashtable(h: Hashtable) -> bool {
    &&& h.dom().len() < max_hashtable_size()
    &&& (forall |k| h.dom().contains(k) ==> valid_key(k) && #[trigger] valid_value(h[k]))
}

pub open spec(checked) fn hashtable_lookup(h: Hashtable, k: AbstractKey) -> Option<AbstractValue> {
    if h.dom().contains(k) { Some(h[k]) } else { None }
}

pub open spec(checked) fn bulk_update_domain(h: Hashtable, kr: KeyRange<AbstractKey>, u: Hashtable) -> Set<AbstractKey> {
    Set::<AbstractKey>::new(|k| (h.dom().contains(k) || u.dom().contains(k))
                                && (kr.contains(k) ==> u.dom().contains(k)))
}

pub open spec fn bulk_update_hashtable(h: Hashtable, kr: KeyRange<AbstractKey>, u: Hashtable) -> Hashtable {
    Map::<AbstractKey, AbstractValue>::new(
        |k: AbstractKey| bulk_update_domain(h, kr, u).contains(k),
        |k: AbstractKey| if u.dom().contains(k) { u[k] } else { h[k] }
    )
}

pub open spec fn bulk_remove_hashtable(h: Hashtable, kr: KeyRange<AbstractKey>) -> Hashtable {
    Map::<AbstractKey, AbstractValue>::new(
        |k: AbstractKey| h.dom().contains(k) && !kr.contains(k),
        |k: AbstractKey| h[k]
    )
}

pub open spec(checked) fn valid_optional_value(ov: Option<AbstractValue>) -> bool {
    match ov {
        None => true,
        Some(value) => valid_value(value),
    }
}

pub open spec fn extract_range(h: Hashtable, kr: KeyRange<AbstractKey>) -> Hashtable {
    Map::<AbstractKey, AbstractValue>::new(
        |k: AbstractKey| h.dom().contains(k) && kr.contains(k),
        |k: AbstractKey| h[k]
    )
}

// ===== Packet extraction =====

pub open spec fn extract_sent_packets_from_ios(ios: Seq<LSHTIo>) -> Seq<LSHTPacket> {
    ios.filter(|io: LSHTIo| io is Send).map_values(|io: LSHTIo| io.arrow_Send_s())
}

pub open spec fn extract_packet_from_lsht_packet(lp: LSHTPacket) -> Packet {
    Packet { dst: lp.dst, src: lp.src, msg: lp.msg }
}

pub open spec fn extract_packets_from_lsht_packets(seq_packets: Seq<LSHTPacket>) -> Set<Packet> {
    seq_packets.map_values(|lp: LSHTPacket| extract_packet_from_lsht_packet(lp)).to_set()
}

pub open spec fn extract_packets_from_abstract_ios(ios: AbstractIos) -> Set<Packet> {
    extract_packets_from_lsht_packets(extract_sent_packets_from_ios(ios))
}

// ===== Transition spec functions =====

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
           &&& post == AbstractHostState {sd: post.sd, received_packet: post.received_packet, ..post}
       }
    ||| {
           &&& pre.received_packet is Some || okay_to_ignore_packets()
           &&& post == pre
           &&& out == Set::<Packet>::empty()
       }
}

pub open spec(checked) fn receive_packet_wrapper(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, sent_packets: Set<Packet>) -> bool {
    exists |ack| receive_packet(pre, post, pkt, sent_packets, ack)
}

pub open spec(checked) fn receive_packet_without_reading_clock(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool
    recommends
        ios.len() >= 1,
        ios[0] is Receive,
        pre.delegation_map.is_complete(),
{
    let r = ios[0].arrow_Receive_r();
    let pkt = Packet{dst: r.dst, src: r.src, msg: r.msg};
    let sent_packets = extract_packets_from_abstract_ios(ios);
    receive_packet_wrapper(pre, post, pkt, sent_packets)
}

pub open spec(checked) fn receive_packet_next(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& ios.len() >= 1
    &&& if ios[0] is TimeoutReceive {
            &&& post == pre
            &&& ios.len() == 1
        } else {
            &&& pre.delegation_map.is_complete()
            &&& ios[0] is Receive
            &&& forall |i| 1 <= i < ios.len() ==> ios[i] is Send
            &&& receive_packet_without_reading_clock(pre, post, ios)
        }
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

pub open spec(checked) fn next_set_request_complete(
    pre: AbstractHostState, post: AbstractHostState,
    src: AbstractEndPoint, seqno: nat, reqm: Message,
    sm: SingleMessage<Message>, replym: Message,
    out: Set<Packet>, should_send: bool
) -> bool
    recommends pre.delegation_map.is_complete(), reqm is SetRequest
{
    let k = reqm.arrow_SetRequest_key();
    let ov = reqm.arrow_SetRequest_value();
    let owner = pre.delegation_map[k];
    if should_send && valid_key(k) && valid_optional_value(ov) {
        &&& if owner == pre.constants.me {
               &&& post.h == match ov { None => pre.h.remove(k), Some(v) => pre.h.insert(k, v) }
               &&& replym == Message::Reply { key: k, value: ov }
               &&& post.received_requests == pre.received_requests.push(AppRequest::AppSetRequest { seqno: seqno, key: k, ov: ov })
           }
           else {
               &&& post.h == pre.h
               &&& replym == Message::Redirect { key: k, id: owner }
               &&& post.received_requests == pre.received_requests
           }
        &&& SingleDelivery::send_single_message(pre.sd, post.sd, replym, src, Some(sm), pre.constants.params)
        &&& sm.arrow_Message_dst() == src
        &&& out == set![Packet{dst: src, src: pre.constants.me, msg: sm}]
    }
    else {
        &&& post == AbstractHostState { received_packet: post.received_packet, ..pre }
        &&& out == Set::<Packet>::empty()
    }
}

pub open spec(checked) fn next_set_request(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends pkt.msg is Message, pre.delegation_map.is_complete()
{
    &&& pkt.msg.arrow_Message_m() is SetRequest
    &&& exists |sm: SingleMessage<Message>, replym: Message, should_send: bool|
            next_set_request_complete(pre, post, pkt.src, pkt.msg.arrow_Message_seqno(), pkt.msg.arrow_Message_m(), sm, replym, out, should_send)
    &&& post.delegation_map == pre.delegation_map
    &&& post.num_delegations == pre.num_delegations
}

pub open spec(checked) fn next_delegate(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends pkt.msg is Message, pre.delegation_map.is_complete()
{
    &&& pkt.msg.arrow_Message_m() is Delegate
    &&& if pre.constants.host_ids.contains(pkt.src) {
            let m = pkt.msg.arrow_Message_m();
            &&& post.delegation_map == pre.delegation_map.update(m.arrow_Delegate_range(), pre.constants.me)
            &&& post.h == bulk_update_hashtable(pre.h, m.arrow_Delegate_range(), m.arrow_Delegate_h())
            &&& post.num_delegations == pre.num_delegations + 1
        }
        else {
            &&& post.delegation_map == pre.delegation_map
            &&& post.h == pre.h
            &&& post.num_delegations == pre.num_delegations
        }
    &&& SingleDelivery::<Message>::send_no_message(pre.sd, post.sd)
    &&& SingleDelivery::<Message>::receive_no_message(pre.sd, post.sd)
    &&& out == Set::<Packet>::empty()
    &&& post.received_requests == pre.received_requests
}

pub open spec(checked) fn next_shard(
    pre: AbstractHostState, post: AbstractHostState, out: Set<Packet>,
    kr: KeyRange<AbstractKey>, recipient: AbstractEndPoint,
    sm: SingleMessage<Message>, should_send: bool
) -> bool
    recommends pre.delegation_map.is_complete()
{
    &&& recipient != pre.constants.me
    &&& pre.constants.host_ids.contains(recipient)
    &&& pre.delegation_map.delegate_for_key_range_is_host(kr, pre.constants.me)
    &&& SingleDelivery::send_single_message(pre.sd, post.sd, Message::Delegate{range: kr, h: extract_range(pre.h, kr)}, recipient, if should_send { Some(sm) } else { None }, pre.constants.params)
    &&& should_send ==> recipient == sm.arrow_Message_dst()
    &&& pre.constants == post.constants
    &&& post.num_delegations == pre.num_delegations + 1
    &&& post.received_requests == pre.received_requests
    &&& if should_send {
            &&& out == set![Packet{dst: recipient, src: pre.constants.me, msg: sm}]
            &&& post.delegation_map == pre.delegation_map.update(kr, recipient)
            &&& post.h == bulk_remove_hashtable(pre.h, kr)
        }
        else {
            &&& out == Set::<Packet>::empty()
            &&& post.delegation_map == pre.delegation_map
            &&& post.h == pre.h
        }
}

pub open spec fn next_shard_wrapper_must_reject(pre: AbstractHostState, m: Message) -> bool {
    let recipient = m.arrow_Shard_recipient();
    let kr = m.arrow_Shard_range();
    ||| recipient == pre.constants.me
    ||| !recipient.valid_physical_address()
    ||| kr.is_empty()
    ||| !pre.constants.host_ids.contains(recipient)
    ||| !pre.delegation_map.delegate_for_key_range_is_host(kr, pre.constants.me)
    ||| extract_range(pre.h, kr).dom().len() >= max_hashtable_size()
}

pub open spec(checked) fn next_shard_wrapper(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends pkt.msg is Message, pre.delegation_map.is_complete()
{
    let m: Message = pkt.msg.arrow_Message_m();
    let recipient = m.arrow_Shard_recipient();
    let kr = m.arrow_Shard_range();
    &&& m is Shard
    &&& if next_shard_wrapper_must_reject(pre, m) {
            &&& post == AbstractHostState { received_packet: post.received_packet, ..pre }
            &&& out == Set::<Packet>::empty()
        } else {
            exists |sm: SingleMessage<Message>, b: bool| next_shard(pre, post, out, kr, recipient, sm, b)
        }
}

pub open spec(checked) fn next_reply(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends pkt.msg is Message, pre.delegation_map.is_complete()
{
    &&& pkt.msg.arrow_Message_m() is Reply
    &&& out == Set::<Packet>::empty()
    &&& post == AbstractHostState { received_packet: post.received_packet, ..pre }
}

pub open spec(checked) fn next_redirect(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends pkt.msg is Message, pre.delegation_map.is_complete()
{
    &&& pkt.msg.arrow_Message_m() is Redirect
    &&& out == Set::<Packet>::empty()
    &&& post == AbstractHostState { received_packet: post.received_packet, ..pre }
}

pub open spec(checked) fn should_process_received_message(pre: AbstractHostState) -> bool {
    &&& pre.received_packet.is_some()
    &&& pre.received_packet.arrow_Some_0().msg is Message
    &&& {
        ||| pre.received_packet.arrow_Some_0().msg.arrow_Message_m() is Delegate
        ||| pre.received_packet.arrow_Some_0().msg.arrow_Message_m() is Shard
        } ==> pre.num_delegations < pre.constants.params.max_delegations - 2
}

pub open spec(checked) fn process_message(pre: AbstractHostState, post: AbstractHostState, out: Set<Packet>) -> bool
    recommends pre.delegation_map.is_complete()
{
    if should_process_received_message(pre) {
        let packet = pre.received_packet.arrow_Some_0();
        &&& {
            ||| next_get_request(pre, post, packet, out)
            ||| next_set_request(pre, post, packet, out)
            ||| next_delegate(pre, post, packet, out)
            ||| next_shard_wrapper(pre, post, packet, out)
            ||| next_reply(pre, post, packet, out)
            ||| next_redirect(pre, post, packet, out)
        }
        &&& post.received_packet is None
    }
    else {
        &&& post == pre
        &&& out == Set::<Packet>::empty()
    }
}

pub open spec(checked) fn process_received_packet(pre: AbstractHostState, post: AbstractHostState, out: Set<Packet>) -> bool
    recommends pre.delegation_map.is_complete()
{
    match pre.received_packet {
        Some(_) => process_message(pre, post, out),
        None => {
            &&& post == pre
            &&& out == Set::<Packet>::empty()
        }
    }
}

pub open spec(checked) fn process_received_packet_next(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& pre.delegation_map.is_complete()
    &&& forall |i| 0 <= i < ios.len() ==> ios[i] is Send
    &&& process_received_packet(pre, post, extract_packets_from_abstract_ios(ios))
}

pub open spec(checked) fn spontaneously_retransmit(pre: AbstractHostState, post: AbstractHostState, out: Set<Packet>) -> bool {
    &&& out == SingleDelivery::un_acked_messages(pre.sd, pre.constants.me)
    &&& post == pre
}

pub open spec(checked) fn spontaneously_retransmit_next(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& pre.delegation_map.is_complete()
    &&& {
        ||| {
            &&& forall |i| 0 <= i < ios.len() ==> ios[i] is Send
            &&& spontaneously_retransmit(pre, post, extract_packets_from_abstract_ios(ios))
        }
        ||| {
            &&& post == pre
            &&& ios =~= Seq::<LSHTIo>::empty()
        }
    }
}

pub open spec(checked) fn ignore_unparseable_packet(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& ios.len() == 1
    &&& ios[0] is Receive
    &&& ios[0].arrow_Receive_r().msg is InvalidMessage
    &&& pre == post
}

pub open spec(checked) fn ignore_nonsensical_delegation_packet(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& ios.len() == 0
    &&& pre.received_packet.is_some()
    &&& pre.received_packet.arrow_Some_0().msg is Message
    &&& match pre.received_packet.arrow_Some_0().msg.arrow_Message_m() {
        Message::Delegate{range: range, h: h} => !({
            &&& valid_hashtable(h)
            &&& !range.is_empty()
            &&& pre.received_packet.arrow_Some_0().msg.arrow_Message_dst().valid_physical_address()
        }),
        _ => false,
      }
    &&& if should_process_received_message(pre) {
          post == AbstractHostState{received_packet: None, ..pre}
      } else {
          post == pre
      }
}

pub enum Step {
    ReceivePacket,
    ProcessReceivedPacket,
    SpontaneouslyRetransmit,
    Stutter,
    IgnoreUnparseablePacket,
    IgnoreNonsensicalDelegationPacket,
}

pub open spec(checked) fn next_step(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos, step: Step) -> bool {
    &&& pre.delegation_map.is_complete()
    &&& match step {
        Step::ReceivePacket => receive_packet_next(pre, post, ios),
        Step::ProcessReceivedPacket => process_received_packet_next(pre, post, ios),
        Step::SpontaneouslyRetransmit => spontaneously_retransmit_next(pre, post, ios),
        Step::Stutter => pre == post && ios.len() == 0,
        Step::IgnoreUnparseablePacket => ignore_unparseable_packet(pre, post, ios),
        Step::IgnoreNonsensicalDelegationPacket => ignore_nonsensical_delegation_packet(pre, post, ios),
    }
}

pub open spec(checked) fn no_invalid_sends(ios: AbstractIos) -> bool {
    forall |i| #![auto] 0 <= i < ios.len() && ios[i] is Send ==> !(ios[i].arrow_Send_s().msg is InvalidMessage)
}

pub open spec(checked) fn next(pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos) -> bool {
    &&& pre.wf()
    &&& pre.constants == post.constants
    &&& exists |step| next_step(pre, post, ios, step)
    &&& no_invalid_sends(ios)
}


// ========================================================================
// BEHAVIORAL MUTATION TESTS — mutate expected outputs or relations
// ========================================================================

// Test 1: spontaneously_retransmit requires post == pre.
// Claiming that h changes after retransmit should fail.
// SHOULD FAIL
proof fn test_mutation_retransmit_changes_h(
    pre: AbstractHostState, post: AbstractHostState, out: Set<Packet>
)
    requires
        spontaneously_retransmit(pre, post, out),
{
    assert(post.h !== pre.h);
}

// Test 2: Stutter step requires pre == post and ios.len() == 0.
// Claiming stutter holds with non-empty ios should fail.
// SHOULD FAIL
proof fn test_mutation_stutter_with_nonempty_ios(
    pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos
)
    requires
        pre.delegation_map.is_complete(),
        ios.len() == 1,
{
    assert(next_step(pre, post, ios, Step::Stutter));
}

// Test 3: next_get_request preserves delegation_map.
// Claiming delegation_map changed after get_request should fail.
// SHOULD FAIL
proof fn test_mutation_get_request_changes_delegation_map(
    pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>
)
    requires
        pre.delegation_map.is_complete(),
        pkt.msg is Message,
        next_get_request(pre, post, pkt, out),
{
    assert(post.delegation_map !== pre.delegation_map);
}

// Test 4: next_reply produces empty output set.
// Claiming non-empty output after reply should fail.
// SHOULD FAIL
proof fn test_mutation_reply_produces_output(
    pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>
)
    requires
        pre.delegation_map.is_complete(),
        pkt.msg is Message,
        next_reply(pre, post, pkt, out),
{
    assert(!out.is_empty());
}

// Test 5: process_received_packet_next requires all ios to be Send.
// Claiming it holds with a Receive io should fail.
// SHOULD FAIL
proof fn test_mutation_process_with_receive_io(
    pre: AbstractHostState, post: AbstractHostState, ios: AbstractIos
)
    requires
        pre.delegation_map.is_complete(),
        ios.len() == 1,
        ios[0] is Receive,
{
    assert(process_received_packet_next(pre, post, ios));
}

}
