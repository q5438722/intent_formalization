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
proof fn phi_5_return_type_not_constrained_beyond_length(n: usize, m: usize)
    requires
        n == m,
    ensures
        init_vec_u8(n)@.len() == init_vec_u8(m)@.len(),
{
}

}
