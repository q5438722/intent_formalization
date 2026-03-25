use vstd::prelude::*;

fn main() {}
pub type ReqId = nat;

verus!{

// File: exec/utils.rs
pub open spec fn rids_match(
    bools: Seq<Option<ReqId>>,
    rids: Seq<ReqId>,
    bools_start: nat,
    bools_end: nat,
    rids_start: nat,
    rids_end: nat,
) -> bool
    decreases bools_end - bools_start,
    when 0 <= bools_start <= bools_end <= bools.len() && 0 <= rids_start <= rids_end <= rids.len()
{
    if bools_end == bools_start {
        rids_end == rids_start
    } else {
        if bools[bools_end - 1].is_Some() {
            &&& rids_end > rids_start
            &&& rids[rids_end - 1] == bools[bools_end - 1].get_Some_0()
            &&& rids_match(
                bools,
                rids,
                bools_start,
                (bools_end - 1) as nat,
                rids_start,
                (rids_end - 1) as nat,
            )
        } else {
            rids_match(bools, rids, bools_start, (bools_end - 1) as nat, rids_start, rids_end)
        }
    }
}

pub proof fn rids_match_add_rid(
    bools: Seq<Option<ReqId>>,
    rids: Seq<ReqId>,
    bools_start: nat,
    bools_end: nat,
    rids_start: nat,
    rids_end: nat,
    rid: ReqId,
)
    requires
        0 <= bools_start <= bools_end <= bools.len(),
        0 <= rids_start <= rids_end <= rids.len(),
        rids_match(bools, rids, bools_start, bools_end, rids_start, rids_end),
    ensures
        rids_match(
            bools.push(Option::Some(rid)),
            rids.push(rid),
            bools_start,
            bools_end,
            rids_start,
            rids_end,
        ),
    decreases bools_end - bools_start,
{
    let bools_new = bools.push(Option::Some(rid));
    let rids_new = rids.push(rid);
    if bools_end == bools_start {
        assert(rids_match(bools_new, rids_new, bools_start, bools_end, rids_start, rids_end));
    } else {
        if bools[bools_end - 1].is_Some() {
            rids_match_add_rid(
                bools,
                rids,
                bools_start,
                (bools_end - 1) as nat,
                rids_start,
                (rids_end - 1) as nat,
                rid,
            );
        } else {
            rids_match_add_rid(
                bools,
                rids,
                bools_start,
                (bools_end - 1) as nat,
                rids_start,
                rids_end,
                rid,
            );
        }
    }
}




// === Entailment query ===
proof fn phi_4_rids_match_duplicate_rids_allowed(rid: ReqId)
    ensures
        rids_match(
            seq![Option::Some(rid), Option::Some(rid)],
            seq![rid, rid],
            0,
            2,
            0,
            2,
        ),
{
    let bools0 = Seq::<Option<ReqId>>::empty();
    let rids0 = Seq::<ReqId>::empty();
    rids_match_add_rid(bools0, rids0, 0, 0, 0, 0, rid);
    let bools1 = bools0.push(Option::Some(rid));
    let rids1 = rids0.push(rid);
    rids_match_add_rid(bools1, rids1, 0, 1, 0, 1, rid);
}

}
