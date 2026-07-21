# vstd missing-spec determinism-reward experiment

- No-post definitions: 44
- Dispositions: `{'diagnostic_candidate': 5, 'already_specified_via_trait': 26, 'intentional_opaque_token': 1, 'linear_resource_effect': 4, 'compiler_internal': 3, 'runtime_effect_unmodeled': 2, 'diverges': 1, 'intentional_nondeterminism': 1, 'linear_resource_hidden_state': 1}`
- Diagnostic candidates checked: 5
- Raw determinism reward total: 3
- Guarded reward total: 0

## Result

No target received a guarded reward. The raw determinism checker gives
positive reward to redundant or vacuous candidates, demonstrating that
determinism alone is not a sufficient reward for unit-returning,
false-precondition, hidden-state, or mode-incompatible functions.

| Target | Candidate | Status | R0 | Raw reward | Guarded reward | Reason |
|---|---|---|---|---:|---:|---|
| `cell::invcell:set@116` | `self.inv(val)` | ok | unsat | 1 | 0 | Sound but redundant: identical to the existing requires clause and does not expose the hidden current cell value. |
| `pervasive:unreached@190` | `false` | ok | unsat | 1 | 0 | Negative control: requires false makes any postcondition vacuous. |
| `pervasive:runtime_assert@204` | `b` | ok | unsat | 1 | 0 | Sound but redundant: b is already required and the function returns unit. |
| `thread:clone@183` | `result@ == self@` | ok | unknown | 0 | 0 | Semantic view equality is expressible, but IsThread is tracked and core::clone::Clone::clone is exec-mode, so verified callers cannot invoke this method. |
| `thread:clone@188` | `result@ == self@` | ok | unknown | 0 | 0 | cfg-alternate body of the same tracked/exec-incompatible Clone implementation. |

## Disposition counts

| Disposition | Count |
|---|---:|
| `already_specified_via_trait` | 26 |
| `compiler_internal` | 3 |
| `diagnostic_candidate` | 5 |
| `diverges` | 1 |
| `intentional_nondeterminism` | 1 |
| `intentional_opaque_token` | 1 |
| `linear_resource_effect` | 4 |
| `linear_resource_hidden_state` | 1 |
| `runtime_effect_unmodeled` | 2 |
