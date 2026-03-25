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
// BEHAVIORAL MUTATION TESTS - Correct inputs, wrong outputs
// These should all FAIL verification
// ============================================================

// Test M1: After truncating [1,2,3] at seqno_acked=2, result length should be 1, not 3
// SHOULD FAIL
proof fn test_m1_wrong_result_length() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let msg3 = make_msg(3);
    let list = seq![msg1, msg2, msg3];
    let result = truncate_un_ack_list(list, 2);
    // seqno 1 <= 2: removed. seqno 2 <= 2: removed. seqno 3 > 2: stays.
    // Correct result length is 1.
    assert(result.len() == 3); // SHOULD FAIL: mutated output, actual is 1
}

// Test M2: After truncating [1,2,3] at seqno_acked=1, first element seqno should be 2, not 1
// SHOULD FAIL
proof fn test_m2_wrong_first_element_seqno() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let msg3 = make_msg(3);
    let list = seq![msg1, msg2, msg3];
    let result = truncate_un_ack_list(list, 1);
    // seqno 1 <= 1: removed. First remaining is seqno 2.
    assert(result[0].arrow_Message_seqno() == 1); // SHOULD FAIL: should be 2
}

// Test M3: After truncating [1,5] at seqno_acked=3, result should have 1 element, not 2
// SHOULD FAIL
proof fn test_m3_wrongly_preserves_removed_element() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(5);
    let list = seq![msg1, msg2];
    let result = truncate_un_ack_list(list, 3);
    // seqno 1 <= 3: removed. seqno 5 > 3: stays.
    assert(result.len() == 2); // SHOULD FAIL: should be 1
}

// Test M4: Truncating [1,2,3] at seqno_acked=2 should yield [msg(3)], not [msg(2),msg(3)]
// SHOULD FAIL
proof fn test_m4_wrong_result_content() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let msg3 = make_msg(3);
    let list = seq![msg1, msg2, msg3];
    let result = truncate_un_ack_list(list, 2);
    // Correct: result = [msg(3)], length 1
    // We wrongly claim result length is 2
    assert(result.len() == 2); // SHOULD FAIL
}

// Test M5: Truncating [1,2,3] at seqno_acked=10 should yield empty, not non-empty
// SHOULD FAIL
proof fn test_m5_wrongly_nonempty_after_full_truncation() {
    let msg1 = make_msg(1);
    let msg2 = make_msg(2);
    let msg3 = make_msg(3);
    let list = seq![msg1, msg2, msg3];
    let result = truncate_un_ack_list(list, 10);
    // All seqnos <= 10: all removed.
    assert(result.len() > 0); // SHOULD FAIL: result is empty
}

}
