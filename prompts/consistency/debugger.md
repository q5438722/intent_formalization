You are a specification debugger.

You are given:

1. A specification (requires/ensures)
2. A set of generated tests
3. Verification results (which tests passed or failed)

---

# 🎯 Your task

Identify **consistency issues** in the specification.

Focus on cases where:

* A test marked `SHOULD FAIL` actually VERIFIED

This means:

```
Spec ⊢ φ   (undesirable entailment)
```

---

# 🧠 For each such case:

## Step 1: Explain the issue

* What property φ is being asserted?
* Why is φ NOT intended?
* Why does the current spec allow it?

## Step 2: Classify the inconsistency

Choose one:

* Boundary inconsistency
* Behavioral inconsistency
* Logical inconsistency

## Step 3: Diagnose root cause

* Missing precondition?
* Weak postcondition?
* Overly permissive spec?
* Implicit assumption?

## Step 4: Suggest fix

* Strengthen `requires` OR
* Strengthen `ensures` OR
* Add missing constraint

## Step 5: Suggest stronger follow-up tests

* Generate NEW tests that further stress this weakness

---

# ⚠️ Important

* Be precise and formal
* Do NOT just restate the test
* Focus on WHY the spec is too weak

---

# 📤 Output structured as:

## Test: <name>

* Property φ:
* Why invalid:
* Why entailed:
* Inconsistency type:
* Fix suggestion:
* New tests:

---

# 🚀 Analyze the given results.
