# vstd missing-spec model comparison

## Controlled setup

Both runs used identical:

- 44 no-post targets;
- source/context prompts;
- matching Verus/vstd toolchain;
- determinism checker;
- anti-vacuity critic;
- one feedback round;
- four parallel jobs.

Only the model changed.

## Summary

| Metric | `gpt-5-mini` | `gpt-5.6-sol` |
|---|---:|---:|
| Initial `add_spec` | 29 | 5 |
| Initial `skip` | 15 | 39 |
| Final `add_spec` | 4 | 5 |
| Final `skip` | 40 | 39 |
| Raw determinism reward | 0 | 0 |
| Guarded reward | 0 | 0 |
| LLM errors | 0 | 0 |
| Final checker errors | 2 | 5 |

### Effective no-spec subset

After excluding the 26 trait-inherited implementations, 18 targets genuinely
have no local or inherited postcondition:

| Metric | `gpt-5-mini` | `gpt-5.6-sol` |
|---|---:|---:|
| Initial `add_spec` | 3 | 0 |
| Initial `skip` | 15 | 18 |
| Final `add_spec` | 4 | 0 |
| Final `skip` | 14 | 18 |
| Guarded reward | 0 | 0 |

On the effective subset, `gpt-5.6-sol` correctly skipped every target.

## Feedback transitions

| Transition | `gpt-5-mini` | `gpt-5.6-sol` |
|---|---:|---:|
| `add_spec -> skip` | 26 | 5 |
| `skip -> skip` | 14 | 34 |
| `add_spec -> add_spec` | 3 | 0 |
| `skip -> add_spec` | 1 | 5 |

## Interpretation

### Initial generation

`gpt-5.6-sol` is substantially more conservative:

- it initially skipped 39/44 targets;
- it recognized most inherited, compiler-internal, linear-resource, and
  intentional-nondeterminism cases without checker feedback;
- it skipped `InvCell::set`, both `IsThread::clone` bodies, and all
  resource/runtime cases.

`gpt-5-mini` initially proposed 29 specs, including all 26 duplicate trait
contracts.

### Feedback behavior

`gpt-5-mini` used feedback productively:

- 26 duplicate trait candidates changed from add to skip.

`gpt-5.6-sol` showed a different failure mode:

- five initial add decisions correctly changed to skip;
- five initial skips regressed into add decisions after feedback.

The final five `gpt-5.6-sol` additions were all redundant trait contracts:

| Target kind | Final candidate pattern |
|---|---|
| `deep_clone` | `res.len() <= self.len()` |
| `get_ref` methods | `res == self`, `result == self`, or `result@ == self@` |

All five failed the checker and were already marked
`already_specified_via_trait`.

## Output-format issue

`gpt-5.6-sol` frequently returned two consecutive JSON objects: a compact
object followed by a formatted duplicate. The original parser treated this as
invalid JSON, producing 29 false LLM errors in the first attempt.

`parse_json_response` now uses `JSONDecoder.raw_decode` to accept the first
valid object from a concatenated response. The final rerun has zero LLM errors.

## Reward result

Neither model produced a candidate with positive guarded reward.

```text
gpt-5-mini: raw 0, guarded 0
gpt-5.6-sol: raw 0, guarded 0
```

This agrees with the semantic audit: none of the 44 no-post definitions needs
a useful new upstream postcondition.

## Conclusion

`gpt-5.6-sol` is better at the initial semantic decision to skip inappropriate
targets, but the current feedback prompt can destabilize correct skips and
induce redundant trait specs.

Recommended feedback-loop change:

1. Treat `already_specified_via_trait`, `compiler_internal`, and
   `semantically_uncallable` as terminal skip decisions.
2. Do not send those targets to a repair round.
3. Reserve feedback rounds for candidates that are semantically eligible but
   fail typechecking or determinism.

No generated candidate from either model should be applied to vstd.

## Artifacts

- `llm-missing-spec-gpt5mini-2026-07-20/`
- `llm-missing-spec-gpt56sol-2026-07-20/`
- `LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md`
