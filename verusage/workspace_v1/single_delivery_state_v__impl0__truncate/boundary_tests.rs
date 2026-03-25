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
// BOUNDARY TESTS - Precondition/edge-case violations
// These should all FAIL verification
// ============================================================

// Test B1: Empty sequence should return empty, not non-empty
// SHOULD FAIL
proof fn test_b1_empty_returns_nonempty() {
    let empty: Seq<SingleMessage<u64>> = Seq::empty();
    let result = truncate_un_ack_list(empty, 5);
    assert(result.len() > 0); // SHOULD FAIL: empty input => empty output
}

// Test B2: Ack messages should NOT be removed by truncation
// truncate_un_ack_list only removes leading Message variants with seqno <= seqno_acked
// SHOULD FAIL
proof fn test_b2_truncate_removes_ack() {
    let ack: SingleMessage<u64> = SingleMessage::Ack { ack_seqno: 1 };
    let s = seq![ack];
    let result = truncate_un_ack_list(s, 10);
    assert(result.len() == 0); // SHOULD FAIL: Ack is not Message, so it stays
}

// Test B3: seqno_acked=0 should NOT remove a message with seqno=1 (1 > 0)
// SHOULD FAIL
proof fn test_b3_seqno_zero_does_not_remove_seqno_one() {
    let msg = make_msg(1);
    let s = seq![msg];
    let result = truncate_un_ack_list(s, 0);
    assert(result.len() == 0); // SHOULD FAIL: seqno 1 > 0, message stays
}

// Test B4: When seqno_acked < all message seqnos, no messages removed
// SHOULD FAIL
proof fn test_b4_no_removal_when_seqno_too_small() {
    let msg = make_msg(100);
    let s = seq![msg];
    let result = truncate_un_ack_list(s, 50);
    assert(result.len() == 0); // SHOULD FAIL: seqno 100 > 50, nothing removed
}

// Test B5: InvalidMessage variant should NOT be removed by truncation
// SHOULD FAIL
proof fn test_b5_truncate_removes_invalid_message() {
    let inv: SingleMessage<u64> = SingleMessage::InvalidMessage {};
    let s = seq![inv];
    let result = truncate_un_ack_list(s, 1000);
    assert(result.len() == 0); // SHOULD FAIL: InvalidMessage is not Message, stays
}

}
