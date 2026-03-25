use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

pub fn vec_erase<A>(v: &mut Vec<A>, start: usize, end: usize)
    requires
        start <= end <= old(v).len(),
    ensures
        true,
        v@ == old(v)@.subrange(0, start as int) + old(v)@.subrange(end as int, old(v)@.len() as int),
{
    let mut deleted = 0;
    proof {
        assert_seqs_equal!(v@,
                           old(v)@.subrange(0, start as int) +
                           old(v)@.subrange(start as int + deleted as int,
                                               old(v)@.len() as int));
    }
    while deleted < end - start
        invariant
            start <= end <= old(v)@.len(),
            v@.len() == old(v)@.len() - deleted,
            0 <= deleted <= end - start,
            v@ == old(v)@.subrange(0, start as int) + old(v)@.subrange(start as int + deleted as int, old(v)@.len() as int),
        decreases
            end - start - deleted
    {
        v.remove(start);
        deleted = deleted + 1;
        proof {
            assert_seqs_equal!(v@,
                               old(v)@.subrange(0, start as int) +
                               old(v)@.subrange(start as int + deleted as int,
                                                   old(v)@.len() as int));
        }
    }
}



// === Entailment query ===
proof fn phi_4_erased_element_still_present(old_v: Seq<int>, v: Seq<int>, start: int, end: int)
    requires
        0 <= start < end <= old_v.len(),
        end < old_v.len(),
        v == old_v.subrange(0, start) + old_v.subrange(end, old_v.len()),
    ensures
        v[start] == old_v[start],
{
}

}
