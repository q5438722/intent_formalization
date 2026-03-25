use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions from target file ==========

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn suffix(self, pos: nat) -> Self {
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
}

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {
    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred { pred: pred }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

proof fn not_eventually_by_always_not<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures not(eventually(p)).satisfied_by(ex),
{
    always_unfold::<T>(ex, not(p));
}

// ========== Logical Tests ==========

// Test 1: Invalid duality — not(always(p)) does NOT imply always(not(p))
// ¬□p means p fails sometimes, but □¬p means p fails always — strictly stronger
// SHOULD FAIL
proof fn logical_invalid_duality<T>(ex: Execution<T>, p: TempPred<T>)
    requires not(always(p)).satisfied_by(ex),
    ensures always(not(p)).satisfied_by(ex),
{
}

// Test 2: Stronger property — always(not(p)) implies always(not(eventually(p)))
// This requires applying not_eventually_by_always_not to every suffix,
// which requires showing always(not(p)) is preserved under suffix — not provided by spec
// SHOULD FAIL
proof fn logical_stronger_always_not_eventually<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures always(not(eventually(p))).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 3: Cross-function misuse — always_unfold on eventually(p) without always
// Try to derive universal quantification from an existential claim
// SHOULD FAIL
proof fn logical_unfold_eventually_as_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold(ex, p);
}

// Test 4: Double negation — not(not(p)) behaves like p
// Spec uses not() but doesn't guarantee double negation elimination
// SHOULD FAIL
proof fn logical_double_negation<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(not(p))).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
}

// Test 5: Commutativity of always and not — always(not(p)) implies not(always(p))
// This is semantically true but not explicitly in the spec; tests if spec leaks this
// SHOULD FAIL
proof fn logical_always_not_implies_not_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures not(always(p)).satisfied_by(ex),
{
}

}
