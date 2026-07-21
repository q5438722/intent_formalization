Revise a proposed Verus postcondition after determinism-checking feedback.

Target: thread:clone@183

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "IsThread is tracked/compiler-erased, while Clone::clone is an exec-mode trait method with no inherited postcondition; verified callers cannot invoke it, so even result@ == self@ would be unusable.",
  "risks": []
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
  "tracked_value_exec_clone_is_uncallable"
]
```

Relevant source:
```rust
 124:         ),
 125:     );
 126:     match res {
 127:         Ok(res) => res,
 128:         Err(_) => {
 129:             println!("panic on spawn");
 130:             std::process::abort();
 131:         },
 132:     }
 133: }
 134: 
 135: /// Wrapper around Rust's
 136: /// [`ThreadId`](https://doc.rust-lang.org/std/thread/struct.ThreadId.html)
 137: /// object. This is an opaque type.
 138: // Note: Rust defines ThreadId as an opaque type. Rust guarantees that ThreadIds
 139: // will never be reused. There's also an `as_u64()` method, but it's unstable,
 140: // and right now it's not clear if it's going to have the same guarantee.
 141: // Regardless, it seems best to stick with Rust's opaque type here.
 142: #[verifier::external_body]
 143: pub struct ThreadId {
 144:     thread_id: std::thread::ThreadId,
 145: }
 146: 
 147: /// Proof object that guarantees the owning thread has the given ThreadId.
 148: #[cfg(verus_keep_ghost)]
 149: #[verifier::external_body]
 150: pub tracked struct IsThread {}
 151: 
 152: #[cfg(verus_keep_ghost)]
 153: impl !Sync for IsThread {
 154: 
 155: }
 156: 
 157: #[cfg(verus_keep_ghost)]
 158: impl !Send for IsThread {
 159: 
 160: }
 161: 
 162: // TODO: remove this when !Sync, !Send are supported by stable Rust
 163: #[cfg(not(verus_keep_ghost))]
 164: #[verifier::external_body]
 165: pub tracked struct IsThread {
 166:     _no_send_sync: core::marker::PhantomData<*const ()>,
 167: }
 168: 
 169: impl IsThread {
 170:     pub uninterp spec fn view(&self) -> ThreadId;
 171: 
 172:     /// Guarantees that any two `IsThread` objects on the same thread
 173:     /// will have the same ID.
 174:     pub axiom fn agrees(tracked self, tracked other: IsThread)
 175:         ensures
 176:             self@ == other@,
 177:     ;
 178: }
 179: 
 180: #[verifier::external]
 181: impl Clone for IsThread {
 182:     #[cfg(verus_keep_ghost)]
 183:     fn clone(&self) -> Self {
 184:         IsThread {  }
 185:     }
 186: 
 187:     #[cfg(not(verus_keep_ghost))]
 188:     fn clone(&self) -> Self {
 189:         IsThread { _no_send_sync: Default::default() }
 190:     }
 191: }
 192: 
 193: impl Copy for IsThread {
 194: 
 195: }
 196: 
 197: /// Gets the current thread ID using Rust's [`Thread::id()`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.id)
 198: /// under the hood. Also returns a ghost object representing proof of being on this thread.
 199: #[verifier::external_body]
 200: pub fn thread_id() -> (res: (ThreadId, Tracked<IsThread>))
 201:     ensures
 202:         res.1@@ == res.0,
 203: {
 204:     let id: std::thread::ThreadId = std::thread::current().id();
 205:     let id = ThreadId { thread_id: id };
 206:     (id, Tracked::assume_new())
 207: }
 208: 
 209: /// Returns _just_ the ghost object, without physically obtaining the thread ID.
 210: pub axiom fn ghost_thread_id() -> (tracked res: IsThread);
 211: 
 212: /// Tracked object that makes any tracked object `Send` or `Sync`.
 213: /// Requires the client to prove that they are the correct thread in order to
 214: /// access the underlying object.
 215: #[verifier::external_body]
 216: #[verifier::accept_recursive_types(V)]
 217: tracked struct ThreadShareable<V> {
 218:     phantom: marker::PhantomData<V>,
 219: }
 220: 
 221: #[verifier::external]
 222: unsafe impl<V> Sync for ThreadShareable<V> {
 223: 
 224: }
 225: 
 226: #[verifier::external]
 227: unsafe impl<V> Send for ThreadShareable<V> {
 228: 
 229: }
 230: 
 231: impl<V> ThreadShareable<V> {
 232:     pub uninterp spec fn view(&self) -> V;
 233: 
 234:     pub uninterp spec fn id(&self) -> ThreadId;
 235: 
 236:     /// Recover the inner value provide we are on the same thread.
 237:     pub axiom fn into(tracked self, tracked is_thread: IsThread) -> (tracked res: V)
 238:         requires
 239:             self.id() == is_thread@,
 240:         ensures
 241:             res == self@,
 242:     ;
 243: 
 244:     /// Borrow the inner value provide we are on the same thread.
 245:     pub axiom fn borrow(tracked &self, tracked is_thread: IsThread) -> (tracked res: &V)
 246:         requires
 247:             self.id() == is_thread@,
 248:         ensures
 249:             *res == self@,
 250:     ;
 251: }
 252: 
 253: impl<V: Send> ThreadShareable<V> {
 254:     /// Recover the inner value.
 255:     /// Unlike `into`, this has no thread requirement, but it does
 256:     /// require the inner type to be `Send`.
 257:     pub axiom fn send_into(tracked self) -> (tracked res: V)
 258:         ensures
 259:             res == self@,
 260:     ;
 261: }
 262: 
 263: impl<V: Sync> ThreadShareable<V> {
 264:     /// Borrow the inner value.
 265:     /// Unlike `borrow`, this has no thread requirement, but it does
 266:     /// require the inner type to be `Sync`.
 267:     pub axiom fn sync_borrow(tracked &self) -> (tracked res: &V)
 268:         ensures
 269:             *res == self@,
 270:     ;
 271: }
 272: 
 273: } // verus!
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
