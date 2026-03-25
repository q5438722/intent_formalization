use vstd::prelude::*;

fn main() {}

verus! {

// ============================================================
// Minimal type definitions needed for testing truncate_un_ack_list
// ============================================================

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
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

pub type AckList<MT> = Seq<SingleMessage<MT>>;

pub open spec(checked) fn truncate_un_ack_list<MT>(un_acked: AckList<MT>, seqno_acked: nat) -> Seq<SingleMessage<MT>>
    decreases un_acked.len()
{
    if un_acked.len() > 0 && un_acked[0] is Message && un_acked[0].arrow_Message_seqno() <= seqno_acked {
        truncate_un_ack_list(un_acked.skip(1), seqno_acked)
    } else {
        un_acked
    }
}

// Helper spec functions
spec fn empty_dst() -> AbstractEndPoint {
    AbstractEndPoint { id: Seq::empty() }
}

spec fn make_msg(seqno: nat) -> SingleMessage<u64> {
    SingleMessage::Message {
        seqno: seqno,
        dst: empty_dst(),
        m: 0u64,
    }
}

// ============================================================
// LOGICAL TESTS - Properties NOT explicitly guaranteed
// These should all FAIL verification
// ============================================================

// Test L1: Truncation is NOT always a no-op (identity)
// SHOULD FAIL
proof fn test_l1_truncation_is_not_identity() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let list = seq![msg1, msg2];
    let result = truncate_un_ack_list(list, 5);
    // Both seqnos <= 5, so all removed. Result is empty.
    assert(result =~= list); // SHOULD FAIL: result is empty, not [msg1, msg2]
}

// Test L2: Truncation result's first element does NOT always have seqno == seqno_acked + 1
// The exec postcondition guarantees this for valid sequential lists, but for arbitrary lists it's NOT guaranteed
// SHOULD FAIL
proof fn test_l2_first_element_seqno_not_guaranteed() {
    let msg = make_msg(10);
    let list = seq![msg];
    let result = truncate_un_ack_list(list, 3);
    // seqno 10 > 3, so msg stays. First element has seqno 10, not 3+1=4.
    assert(result[0].arrow_Message_seqno() == 4); // SHOULD FAIL: it's 10, not 4
}

// Test L3: Truncation does NOT always make the list strictly shorter
// When no elements qualify for removal, the length is unchanged
// SHOULD FAIL
proof fn test_l3_always_shorter_is_wrong() {
    let msg = make_msg(10);
    let list = seq![msg];
    let result = truncate_un_ack_list(list, 5);
    // seqno 10 > 5: nothing removed. len stays 1.
    assert(result.len() < list.len()); // SHOULD FAIL: 1 < 1 is false
}

// Test L4: Truncation is NOT order-independent
// It only removes from the FRONT; reordering the list gives different results
// SHOULD FAIL
proof fn test_l4_order_independent_is_wrong() {
    let msg_hi = make_msg(5);
    let msg_lo = make_msg(1);
    // List 1: high seqno first, low seqno second
    let list1 = seq![msg_hi, msg_lo];
    let result1 = truncate_un_ack_list(list1, 3);
    // msg_hi seqno=5 > 3: stop. result1 = [msg_hi, msg_lo], len=2
    // List 2: low seqno first, high seqno second
    let list2 = seq![msg_lo, msg_hi];
    let result2 = truncate_un_ack_list(list2, 3);
    // msg_lo seqno=1 <= 3: removed. msg_hi seqno=5 > 3: stop. result2 = [msg_hi], len=1
    assert(result1.len() == result2.len()); // SHOULD FAIL: 2 != 1
}

// Test L5: The number of removed elements does NOT always equal seqno_acked
// SHOULD FAIL
proof fn test_l5_removed_count_not_seqno_acked() {
    let msg = make_msg(10);
    let list = seq![msg];
    let result = truncate_un_ack_list(list, 2);
    // No elements removed (10 > 2). list.len() - result.len() = 0, not 2.
    assert(list.len() - result.len() == 2); // SHOULD FAIL: 1 - 1 = 0 != 2
}

}
