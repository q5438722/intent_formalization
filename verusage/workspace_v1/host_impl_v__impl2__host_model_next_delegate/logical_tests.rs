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
    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }

    pub open spec fn get_spec(&self) -> &K
        recommends self.k.is_some(),
    {
        &self.k.get_Some_0()
    }

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
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
pub type CKey = SHTKey;
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
    pub open spec(checked) fn receive_no_message(pre: Self, post: Self) -> bool {
        post.receive_state == pre.receive_state
    }

    pub open spec(checked) fn send_no_message(pre: Self, post: Self) -> bool {
        post.send_state == pre.send_state
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

    pub open spec fn is_complete(self) -> bool { self@.dom().is_full() }

    pub open spec fn update(self, newkr: KeyRange<AbstractKey>, host: AbstractEndPoint) -> Self
        recommends self.is_complete(),
    {
        AbstractDelegationMap(self@.union_prefer_right(Map::new(|k| newkr.contains(k), |k| host)))
    }
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
pub open spec fn max_hashtable_size() -> int { 62 }

pub open spec fn valid_hashtable(h: Hashtable) -> bool {
    &&& h.dom().len() < max_hashtable_size()
    &&& (forall |k| h.dom().contains(k) ==> valid_key(k) && #[trigger] valid_value(h[k]))
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

pub open spec(checked) fn next_delegate(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, out: Set<Packet>) -> bool
    recommends
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
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

// ========== LOGICAL TESTS ==========

// L1: next_delegate should NOT entail false (soundness check).
// If next_delegate is satisfiable, we should not be able to derive false.
// SHOULD FAIL
proof fn test_logical_next_delegate_not_unsound(pre: AbstractHostState, post: AbstractHostState, pkt: Packet)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_delegate(pre, post, pkt, Set::<Packet>::empty()),
{
    assert(false);
}

// L2: next_delegate should NOT allow received_requests to change.
// Asserting received_requests changed should fail since the spec preserves them.
// SHOULD FAIL
proof fn test_logical_received_requests_can_change(pre: AbstractHostState, post: AbstractHostState, pkt: Packet, extra: AppRequest)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_delegate(pre, post, pkt, Set::<Packet>::empty()),
{
    assert(post.received_requests != pre.received_requests);
}

// L3: next_delegate should NOT allow constants to change.
// The spec does not explicitly constrain post.constants, so this tests if
// the spec is weak enough to allow post.constants != pre.constants.
// NOTE: This may PASS (spec weakness) - the spec doesn't constrain constants.
// SHOULD FAIL
proof fn test_logical_constants_preserved(pre: AbstractHostState, post: AbstractHostState, pkt: Packet)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_delegate(pre, post, pkt, Set::<Packet>::empty()),
{
    // next_delegate does NOT explicitly require post.constants == pre.constants
    // but refers to pre.constants.me for delegation_map update.
    // Try to assert constants differ - if this PASSES, spec is too weak.
    assert(post.constants !== pre.constants);
}

// L4: next_delegate should be deterministic given the same inputs.
// Two post-states from the same (pre, pkt, out) should have the same num_delegations.
// SHOULD FAIL
proof fn test_logical_determinism_num_delegations(
    pre: AbstractHostState,
    post1: AbstractHostState,
    post2: AbstractHostState,
    pkt: Packet,
)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_delegate(pre, post1, pkt, Set::<Packet>::empty()),
        next_delegate(pre, post2, pkt, Set::<Packet>::empty()),
{
    assert(post1.num_delegations != post2.num_delegations);
}

// L5: next_delegate should NOT allow received_packet to remain Some.
// The spec doesn't constrain post.received_packet, so this tests spec weakness.
// NOTE: This may PASS (spec weakness) - next_delegate doesn't constrain received_packet.
// SHOULD FAIL
proof fn test_logical_received_packet_unconstrained(pre: AbstractHostState, post: AbstractHostState, pkt: Packet)
    requires
        pkt.msg is Message,
        pre.delegation_map.is_complete(),
        next_delegate(pre, post, pkt, Set::<Packet>::empty()),
        pre.received_packet is Some,
{
    // next_delegate doesn't explicitly set post.received_packet to None.
    // If the spec allows post.received_packet to remain Some, this assertion
    // could pass, revealing spec weakness.
    assert(post.received_packet is None);
}

}
