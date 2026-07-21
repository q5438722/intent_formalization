Revise a proposed Verus postcondition after determinism-checking feedback.

Target: contrib::exec_spec::map:exec_eq@72

Previous proposal:
```json
{
  "decision": "add_spec",
  "ensures": [
    "res == (this.deep_view() =~~= other.deep_view())"
  ],
  "useful": true,
  "rationale": "Matches the ExecSpecEq trait contract: the boolean result must reflect deep-view equality of the two maps. This is the strongest concise specification expressible with existing vstd vocabulary and directly ties the impl to the semantic map equality used throughout the spec layer.",
  "risks": [
    "The implementation uses HashMap::get (Hash + Eq) to find matching keys; if the ExecSpecEq semantics for K are not consistent with the runtime Hash/Eq used by HashMap, the body could be unsound relative to the spec.",
    "Hidden nondeterminism from HashMap iteration order is tolerated because deep_view equality (=~~=) is order-independent, but callers must ensure DeepView and ExecSpecEq instances are pure and deterministic.",
    "Function is marked external_body; verifier will rely on this postcondition rather than checking the body, so the postcondition must accurately capture intended behavior."
  ]
}
```

Checker result:
```json
{
  "status": "verus_error",
  "requires": [],
  "ensures": [
    "res == (this.deep_view() =~~= other.deep_view())"
  ],
  "equal_fn_trivial": false,
  "stderr_tail": " K>,\n   |                  ^^^^^^^^^^ not found in this scope\n   |\nhelp: consider importing this trait\n   |\n 3 + use vstd::contrib::exec_spec::ExecSpecEq;\n   |\n\nerror[E0405]: cannot find trait `ExecSpecEq` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:18:12\n   |\n18 |     &'a V: ExecSpecEq<'a, Other = &'a V>,\n   |            ^^^^^^^^^^ not found in this scope\n   |\nhelp: consider importing this trait\n   |\n 3 + use vstd::contrib::exec_spec::ExecSpecEq;\n   |\n\nerror[E0425]: cannot find type `HashMap` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:16:146\n   |\n16 | ..._r2_is_false: bool, g_neq_tuple: bool, this: &'a HashMap<K, V>, other: &'a HashMap<K, V>::Other, r1: bool, r2: bool)\n   |                                                     ^^^^^^^ not found in this scope\n   |\nhelp: consider importing this struct\n   |\n 3 + use std::collections::HashMap;\n   |\n\nerror[E0433]: cannot find type `HashMap` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:16:172\n   |\n16 | ...: bool, this: &'a HashMap<K, V>, other: &'a HashMap<K, V>::Other, r1: bool, r2: bool)\n   |                                                ^^^^^^^ use of undeclared type `HashMap`\n   |\nhelp: consider importing this struct\n   |\n 3 + use std::collections::HashMap;\n   |\n\nerror[E0425]: cannot find value `res` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:21:18\n   |\n21 |             &&& (res == (this.deep_view() =~~= other.deep_view()))\n   |                  ^^^ not found in this scope\n\nerror[E0425]: cannot find value `res` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:22:18\n   |\n22 |             &&& (res == (this.deep_view() =~~= other.deep_view()))\n   |                  ^^^ not found in this scope\n\nerror[E0603]: module `map` is private\n --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__map__exec_eq__L72/round_00/artifacts/contrib__exec_spec__map__exec_eq__L72/harness.rs:4:31\n  |\n4 | use vstd::contrib::exec_spec::map::*;\n  |                               ^^^ private module\n  |\nnote: the module `map` is defined here\n --> vstd/contrib/exec_spec/mod.rs:13:0\n\nerror: aborting due to 9 previous errors\n\nSome errors have detailed explanations: E0405, E0425, E0433, E0603.\nFor more information about an error, try `rustc --explain E0405`.\n"
}
```

Anti-vacuity/semantic issues:
```json
[
  "already_specified_via_trait",
  "checker_status:verus_error"
]
```

Relevant source:
```rust
  13:     crate::std_specs::hash::group_hash_axioms,
  14:     crate::std_specs::hash::lemma_hashmap_deepview_dom,
  15: };
  16: 
  17: /// Impls for shared traits
  18: impl<
  19:     'a,
  20:     K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
  21:     V: DeepView + DeepViewClone,
  22: > ToRef<&'a HashMap<K, V>> for &'a HashMap<K, V> {
  23:     #[inline(always)]
  24:     fn get_ref(self) -> &'a HashMap<K, V> {
  25:         &self
  26:     }
  27: }
  28: 
  29: impl<
  30:     'a,
  31:     K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
  32:     V: DeepView + DeepViewClone,
  33: > ToOwned<HashMap<K, V>> for &'a HashMap<K, V> {
  34:     #[verifier::external_body]
  35:     #[inline(always)]
  36:     fn get_owned(self) -> HashMap<K, V> {
  37:         let mut new_map = HashMap::new();
  38:         for (k, v) in self.iter() {
  39:             new_map.insert(k.deep_clone(), v.deep_clone());
  40:         }
  41:         new_map
  42:     }
  43: }
  44: 
  45: impl<
  46:     K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
  47:     V: DeepView + DeepViewClone,
  48: > DeepViewClone for HashMap<K, V> {
  49:     #[verifier::external_body]
  50:     #[inline(always)]
  51:     fn deep_clone(&self) -> Self {
  52:         let mut new_map = HashMap::new();
  53:         for (k, v) in self.iter() {
  54:             new_map.insert(k.deep_clone(), v.deep_clone());
  55:         }
  56:         new_map
  57:     }
  58: }
  59: 
  60: impl<
  61:     'a,
  62:     K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
  63:     V: DeepView + DeepViewClone,
  64: > ExecSpecEq<'a> for &'a HashMap<K, V> where
  65:     &'a K: ExecSpecEq<'a, Other = &'a K>,
  66:     &'a V: ExecSpecEq<'a, Other = &'a V>,
  67:  {
  68:     type Other = &'a HashMap<K, V>;
  69: 
  70:     #[verifier::external_body]
  71:     #[inline(always)]
  72:     fn exec_eq(this: Self, other: Self::Other) -> bool {
  73:         if this.len() != other.len() {
  74:             return false;
  75:         }
  76:         for (k, v) in this.iter() {
  77:             match other.get(k) {
  78:                 Some(ov) => {
  79:                     if !<&'a V>::exec_eq(v, ov) {
  80:                         return false;
  81:                     }
  82:                 },
  83:                 None => return false,
  84:             }
  85:         }
  86:         true
  87:     }
  88: }
  89: 
  90: impl<
  91:     'a,
  92:     K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
  93:     V: DeepView + DeepViewClone,
  94: > ExecSpecLen for &'a HashMap<K, V> {
  95:     #[inline(always)]
  96:     #[verifier::external_body]
  97:     fn exec_len(self) -> (res: usize)
  98:         ensures
  99:             res == self.deep_view().len(),
 100:     {
 101:         self.len()
 102:     }
 103: }
 104: 
 105: /// Traits for Map methods
 106: /// Spec for executable version of [`Map::empty`].
 107: pub trait ExecSpecMapEmpty: Sized {
 108:     fn exec_empty() -> Self;
 109: }
 110: 
 111: // todo: this only works for primtive key types right now
 112: /// Spec for executable version of [`Map`] indexing.
 113: pub trait ExecSpecMapIndex<'a>: Sized + DeepView<
 114:     V = Map<<Self::Key as DeepView>::V, <Self::Value as DeepView>::V>,
 115: > {
 116:     type Key: DeepView;
 117: 
 118:     type Value: DeepView;
 119: 
 120:     fn exec_index(self, key: Self::Key) -> Self::Value
 121:         requires
 122:             self.deep_view().dom().contains(key.deep_view()),
 123:     ;
 124: }
 125: 
 126: /// Spec for executable version of [`Map::insert`].
 127: pub trait ExecSpecMapInsert<'a, Out: Sized + DeepView>: Sized + DeepView + ToOwned<Out> {
 128:     type Key: DeepView + DeepViewClone;
 129: 
 130:     type Value: DeepView + DeepViewClone;
 131: 
 132:     fn exec_insert(self, key: Self::Key, value: Self::Value) -> Out;
 133: }
 134: 
 135: /// Spec for executable version of [`Map::remove`].
 136: pub trait ExecSpecMapRemove<'a, Out: Sized + DeepView>: Sized + DeepView + ToOwned<Out> {
 137:     type Key: DeepView + DeepViewClone;
 138: 
 139:     fn exec_remove(self, key: Self::Key) -> Out;
 140: }
 141: 
 142: /// Spec for executable version of [`Map::dom`].
 143: pub trait ExecSpecMapDom<'a>: Sized + DeepView {
 144:     type Key: DeepView + DeepViewClone;
 145: 
 146:     fn exec_dom(self) -> HashSet<Self::Key>;
 147: }
 148: 
 149: /// Spec for executable version of [`Map::get`].
 150: pub trait ExecSpecMapGet<'a>: Sized + DeepView {
 151:     type Key: DeepView + DeepViewClone;
 152: 
 153:     type Value: DeepView + DeepViewClone;
 154: 
 155:     fn exec_get(self, k: Self::Key) -> Option<Self::Value>;
 156: }
 157: 
 158: /// Impls for executable versions of Map methods
 159: impl<K: DeepView + std::hash::Hash + std::cmp::Eq, V: DeepView> ExecSpecMapEmpty for HashMap<K, V> {
 160:     #[inline(always)]
 161:     fn exec_empty() -> (res: Self)
 162:         ensures

// Shared trait declarations:
T001: //! This module provides runtime utilities for the compiled
T002: //! executable code of [`verus_builtin_macros::exec_spec_verified`]
T003: //! and [`verus_builtin_macros::exec_spec_unverified`].
T004: #![cfg(all(feature = "alloc", feature = "std"))]
T005: 
T006: use crate::multiset::*;
T007: use crate::prelude::*;
T008: use std::collections::HashMap;
T009: use std::collections::HashSet;
T010: pub use verus_builtin_macros::exec_spec_unverified;
T011: pub use verus_builtin_macros::exec_spec_verified;
T012: 
T013: mod map;
T014: pub use map::*;
T015: mod multiset;
T016: pub use multiset::*;
T017: mod option;
T018: pub use option::*;
T019: mod seq;
T020: pub use seq::*;
T021: mod set;
T022: pub use set::*;
T023: mod string;
T024: pub use string::*;
T025: 
T026: verus! {
T027: 
T028: /// [`ToRef`] and [`ToOwned`] are almost the same trait
T029: /// but separated to avoid type inference ambiguities.
T030: pub trait ToRef<T: Sized + DeepView>: Sized + DeepView<V = T::V> {
T031:     fn get_ref(self) -> (res: T)
T032:         ensures
T033:             res.deep_view() == self.deep_view(),
T034:     ;
T035: }
T036: 
T037: pub trait ToOwned<T: Sized + DeepView>: Sized + DeepView<V = T::V> {
T038:     fn get_owned(self) -> (res: T)
T039:         ensures
T040:             res.deep_view() == self.deep_view(),
T041:     ;
T042: }
T043: 
T044: /// Cloned objects have the same deep view
T045: pub trait DeepViewClone: Sized + DeepView {
T046:     fn deep_clone(&self) -> (res: Self)
T047:         ensures
T048:             res.deep_view() == self.deep_view(),
T049:     ;
T050: }
T051: 
T052: /// Any spec types used in [`exec_spec_verified`] or [`exec_spec_unverified`] macros
T053: /// must implement this trait to indicate
T054: /// the corresponding exec type (owned and borrowed versions).
T055: pub trait ExecSpecType where
T056:     for <'a>&'a Self::ExecOwnedType: ToRef<Self::ExecRefType<'a>>,
T057:     for <'a>Self::ExecRefType<'a>: ToOwned<Self::ExecOwnedType>,
T058:  {
T059:     /// Owned version of the exec type.
T060:     type ExecOwnedType: DeepView<V = Self>;
T061: 
T062:     /// Reference version of the exec type.
T063:     type ExecRefType<'a>: DeepView<V = Self>;
T064: }
T065: 
T066: /// Spec for the executable version of equality.
T067: pub trait ExecSpecEq<'a>: DeepView + Sized {
T068:     type Other: DeepView<V = Self::V>;
T069: 
T070:     fn exec_eq(this: Self, other: Self::Other) -> (res: bool)
T071:         ensures
T072:             res == (this.deep_view() =~~= other.deep_view()),
T073:     ;
T074: }
T075: 
T076: /// Spec for executable version of [`Seq`] and [`str`] indexing.
T077: pub trait ExecSpecIndex<'a>: Sized + DeepView<V = Seq<<Self::Elem as DeepView>::V>> {
T078:     type Elem: DeepView;
T079: 
T080:     fn exec_index(self, index: usize) -> Self::Elem
T081:         requires
T082:             0 <= index < self.deep_view().len(),
T083:     ;
T084: }
T085: 
T086: /// Spec for executable version of `len`.
T087: pub trait ExecSpecLen {
T088:     fn exec_len(self) -> usize;
T089: }
T090: 
T091: /// A macro to implement various traits for primitive arithmetic types.
T092: macro_rules! impl_primitives {
T093:     ($(,)?) => {};
T094:     ($t:ty $(,$rest:ty)* $(,)?) => {
T095:         verus! {
T096:             impl ExecSpecType for $t {
T097:                 type ExecOwnedType = $t;
T098:                 type ExecRefType<'a> = $t;
T099:             }
T100: 
```

Return JSON only with the same schema:
{
  "decision": "add_spec" | "skip",
  "ensures": ["..."],
  "useful": true | false,
  "rationale": "...",
  "risks": ["..."]
}

Do not optimize for `R0 = unsat` by returning a redundant, vacuous, false-domain,
unit-output, or semantically unusable specification. Choose `skip` if no useful
postcondition can be expressed.
