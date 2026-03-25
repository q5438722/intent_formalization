use vstd::prelude::*;
use vstd::raw_ptr::*;


fn main() {}

verus! {

pub const INTPTR_SHIFT: u64 = 3;

pub const INTPTR_SIZE: u64 = 8;

pub const SLICE_SHIFT: u64 = 13 + INTPTR_SHIFT;

pub const SLICE_SIZE: u64 = 65536; //(1 << SLICE_SHIFT);

pub const SEGMENT_SHIFT: u64 = 9 + SLICE_SHIFT;

pub const SEGMENT_SIZE: u64 = (1 << SEGMENT_SHIFT);

pub const SLICES_PER_SEGMENT: u64 = (SEGMENT_SIZE / SLICE_SIZE);

pub closed spec fn segment_start(segment_id: SegmentId) -> int {
    segment_id.id * (SEGMENT_SIZE as int)
}


pub proof fn segment_start_mult8(segment_id: SegmentId)
    ensures segment_start(segment_id) % 8 == 0,
{
        assert(SEGMENT_SIZE == 33554432) by (compute);
}

pub ghost struct SegmentId {
    pub id: nat,
    pub provenance: Provenance,
    pub uniq: int,
}




// === Entailment query ===
proof fn phi_3_segment_start_no_overlap_guarantee(s1: SegmentId, s2: SegmentId)
    requires
        s1.id != s2.id,
    ensures
        segment_start(s1) != segment_start(s2),
{
    assert(SEGMENT_SIZE == 33554432) by (compute);
    assert(SEGMENT_SIZE > 0);
}

}
