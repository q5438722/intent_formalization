use vstd::prelude::*;

fn main() {}

verus! {

// ─── Definitions (from target) ───

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
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires
        forall|i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures
        ex1 == ex2,
{
    unimplemented!()
}

// ─── Logical Tests ───

// SHOULD FAIL: Execution uniqueness — two executions satisfying always(p) need not be equal
proof fn test_logical_execution_uniqueness<T>(
    ex1: Execution<T>,
    ex2: Execution<T>,
    p: TempPred<T>,
)
    requires
        always(p).satisfied_by(ex1),
        always(p).satisfied_by(ex2),
    ensures
        ex1 == ex2,
{
}

// SHOULD FAIL: Suffix is not identity — ex.suffix(1) is generally not equal to ex
proof fn test_logical_suffix_not_identity<T>(ex: Execution<T>)
    ensures
        ex.suffix(1) == ex,
{
}

// SHOULD FAIL: Predicate uniqueness — two predicates both always-true on same execution need not be equal
proof fn test_logical_predicate_uniqueness<T>(
    ex: Execution<T>,
    p: TempPred<T>,
    q: TempPred<T>,
)
    requires
        always(p).satisfied_by(ex),
        always(q).satisfied_by(ex),
    ensures
        p == q,
{
}

}
