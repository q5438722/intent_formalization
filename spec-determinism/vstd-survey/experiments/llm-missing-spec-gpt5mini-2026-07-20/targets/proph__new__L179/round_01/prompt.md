Revise a proposed Verus postcondition after determinism-checking feedback.

Target: proph:new@179

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "new() allocates an opaque prophecy whose spec view has no observable constraints until resolve(). Any nontrivial postcondition would either be tautological (equivalent to true) or would require saying the view is an arbitrary value (prohibited). The module already exposes resolve(...) with a strong ensures. Given available public vstd vocabulary, no sound, non\u2011vacuous postcondition can be stated for new().",
  "risks": [
    "No invariant is recorded at allocation time, so callers cannot rely on additional guarantees beyond ownership of an opaque prophecy value.",
    "If future APIs expose observable allocation properties, a spec must be added then to avoid mismatch."
  ]
}
```

Checker result:
```json
{
  "status": "not_run"
}
```

Anti-vacuity/semantic issues:
```json
[
  "no_candidate_postcondition",
  "semantic_disposition:intentional_nondeterminism"
]
```

Relevant source:
```rust
 120:         *final(pair) == (*final(fst), old(pair).1),
 121: {
 122:     &mut pair.0
 123: }
 124: ```
 125: 
 126: The prophetic value `*final(pair)` is resolved to a value dependent on prophetic value `*final(fst)`.
 127: 
 128: 3. The value `*final(x)` is subject to prophecy-dependence tracking:
 129: 
 130: ```
 131: fn test(x: &mut u64) {
 132:     let xfinal = Ghost(*final(x));
 133: }
 134: ```
 135: 
 136: ```
 137: error: prophetic value not allowed for 'Ghost' wrapper
 138:  --> mut_ref_test.rs:6:24
 139:   |
 140: 6 |     let xfinal = Ghost(*final(x));
 141:   |                        ^^^^^^^^^
 142:   |                        |
 143:   |                        the `final` builtin is prophetic
 144:   |                        operand of this wrapper is expected to be non-prophetic
 145: ```
 146: */
 147: #![allow(unused_variables)]
 148: 
 149: use super::modes::*;
 150: use super::prelude::*;
 151: 
 152: verus! {
 153: 
 154: /** A general-purpose prophecy variable.
 155: 
 156: A prophecy variable is allocated via [`Prophecy::new`](Self::new)
 157: and resolved to a definitive value via [`Prophecy::resolve`](Self::resolve).
 158: 
 159: In contrast to the similar [`ProphecyGhost`], this does not require us to mark the
 160: [`value()`](Self::view) as [`verifier::prophetic`](https://verus-lang.github.io/verus/guide/reference-attributes.html#verifierprophetic), which can make it easier to use.
 161: However, it does not allow dependent resolutions, and it can only resolve to values
 162: which are computed purely in exec-mode (as enforced by the `T: Structural` bound).
 163: See [the module level documentation](self) for a detailed comparison with the rationale
 164: for these trade-offs.
 165: */
 166: #[verifier::external_body]
 167: #[verifier::accept_recursive_types(T)]
 168: pub struct Prophecy<T> {
 169:     v: Ghost<T>,
 170: }
 171: 
 172: impl<T> Prophecy<T> where T: Structural {
 173:     /// The prophecized value.
 174:     pub uninterp spec fn view(self) -> T;
 175: 
 176:     /// Allocate a new prophecy variable.
 177:     #[inline(always)]
 178:     #[verifier::external_body]
 179:     pub exec fn new() -> (proph_var: Self) {
 180:         Prophecy::<T> { v: Ghost::assume_new() }
 181:     }
 182: 
 183:     /// Resolve the prophecy variable to a concrete value.
 184:     /// This consumes `self`, so it can only be called once.
 185:     #[inline(always)]
 186:     #[verifier::external_body]
 187:     pub exec fn resolve(self, v: &T)
 188:         ensures
 189:             self@ == v,
 190:     {
 191:     }
 192: }
 193: 
 194: /** A general-purpose prophecy variable.
 195: 
 196: A prophecy variable is allocated via [`ProphecyGhost::new`](Self::new)
 197: and resolved to a definitive value via [`ProphecyGhost::resolve`](Self::resolve).
 198: 
 199: In contrast to the similar [`Prophecy`], this allows resolving on ghost computations
 200: and permits dependent resolutions. However, the [`value()`](Self::value)
 201: is marked [`verifier::prophetic`](https://verus-lang.github.io/verus/guide/reference-attributes.html#verifierrprophetic), which entails certain restrictions in the way it can be used.
 202: See [the module level documentation](self) for a detailed comparison with the rationale
 203: for these trade-offs.
 204: 
 205: See [`ProphecySeq`] for an example of a library verified using dependent resolutions.
 206: */
 207: 
 208: #[verifier::external_body]
 209: #[verifier::accept_recursive_types(T)]
 210: pub tracked struct ProphecyGhost<T> {
 211:     _t: Ghost<T>,
 212: }
 213: 
 214: impl<T> ProphecyGhost<T> {
 215:     #[verifier::prophetic]
 216:     pub uninterp spec fn value(&self) -> T;
 217: 
 218:     pub axiom fn new() -> (tracked proph_var: Self);
 219: 
 220:     pub axiom fn resolve(tracked self, value: T)
 221:         ensures
 222:             self.value() == value,
 223:     ;
 224: 
 225:     pub proof fn resolve_dependent<U>(
 226:         tracked self,
 227:         tracked u: &ProphecyGhost<U>,
 228:         f: spec_fn(U) -> T,
 229:     )
 230:         ensures
 231:             self.value() == f(u.value()),
 232:     {
 233:         self.resolve_dependent_2(u, &ProphecyGhost::new(), |u: U, v: ()| f(u));
 234:     }
 235: 
 236:     pub axiom fn resolve_dependent_2<U, V>(
 237:         tracked self,
 238:         tracked u: &ProphecyGhost<U>,
 239:         tracked v: &ProphecyGhost<V>,
 240:         f: spec_fn(U, V) -> T,
 241:     )
 242:         ensures
 243:             self.value() == f(u.value(), v.value()),
 244:     ;
 245: 
 246:     pub proof fn resolve_dependent_3<U, V, W>(
 247:         tracked self,
 248:         tracked u: &ProphecyGhost<U>,
 249:         tracked v: &ProphecyGhost<V>,
 250:         tracked w: &ProphecyGhost<W>,
 251:         f: spec_fn(U, V, W) -> T,
 252:     )
 253:         ensures
 254:             self.value() == f(u.value(), v.value(), w.value()),
 255:     {
 256:         let tracked vw = ProphecyGhost::<(V, W)>::new();
 257:         self.resolve_dependent_2(u, &vw, |u: U, vw: (V, W)| f(u, vw.0, vw.1));
 258:         vw.resolve_dependent_2(v, w, |v: V, w: W| (v, w));
 259:     }
 260: }
 261: 
 262: /// A prophetic sequence, which permits prophesying one element at a time.
 263: pub tracked struct ProphecySeq<T> {
 264:     tracked var: ProphecyGhost<Seq<T>>,
 265: }
 266: 
 267: impl<T> ProphecySeq<T> {
 268:     #[verifier::prophetic]
 269:     pub closed spec fn seq(&self) -> Seq<T> {
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
