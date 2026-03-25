use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
macro_rules! bitmask_inc {
    ($low:expr,$high:expr) => {
        (!(!0usize << (($high+1usize)-$low))) << $low
    }
}

#[verifier(external_body)]
pub const MAX_PHYADDR_WIDTH: usize = 52;

pub axiom fn axiom_max_phyaddr_width_facts()
    ensures
        32 <= MAX_PHYADDR_WIDTH <= 52,
;


// File: spec_t/mmu/translation.rs
pub spec const MASK_L2_PG_ADDR_SPEC: usize = bitmask_inc!(21usize, MAX_PHYADDR_WIDTH - 1);

pub fn MASK_L2_PG_ADDR() -> (ret: usize)
    ensures ret == MASK_L2_PG_ADDR_SPEC 
{
    proof {
        axiom_max_phyaddr_width_facts();
    }
    let r = bitmask_inc!(21usize, MAX_PHYADDR_WIDTH - 1);
    r
}



// === Entailment query ===
proof fn phi_1_mask_l2_pg_addr_low_21_bits_zero()
    ensures
        MASK_L2_PG_ADDR_SPEC & 0x1F_FFFF == 0,
{
    axiom_max_phyaddr_width_facts();
    assert(forall|low: usize, high: usize| low <= high && high < 64 ==> bitmask_inc!(low, high) & (!0usize >> (64 - low)) == 0) by (bit_vector);
}

}
