use vstd::prelude::*;

fn main() {}

verus!{

// File: src/utils.rs
pub exec fn init_vec_u8(n: usize) -> (res: Vec<u8>)
    ensures
        res@.len() == n,
{
    let mut i: usize = 0;
    let mut ret: Vec<u8> = Vec::new();
    while i < n
        invariant
            0 <= i <= n,
            ret@.len() == i,
        decreases n - i,
    {
        ret.push(0);
        assert(ret@[i as int] == 0);
        i = i + 1
    }
    ret
}




// === Entailment query ===
exec fn phi_2_init_vec_missing_zero_guarantee()
{
    let v = init_vec_u8(3);
    assert(v@.len() == 3);
    // This should hold since we push 0, but the ensures doesn't promise it
    // assert(v@[0] == 0);  // Cannot prove from ensures alone!
}

}
