很好，这一步我们把你的工作**提升到 formal / PL 级别表达**。我会给你一套**可以直接进论文（Problem Definition / Preliminaries / Theory）**的定义体系，并且严格对齐你前面的框架（entailment + query + debugging）。

---

# 📐 Formal Definitions: Consistency via Entailment

---

## 1️⃣ Preliminaries

我们首先定义规范与可推导性。

### Definition 1 (Specification)

A **specification** is a pair:

[
S = (Pre, Post)
]

where:

* ( Pre(x) ) is a predicate over inputs ( x )
* ( Post(x, y) ) is a predicate relating inputs ( x ) and outputs ( y )

A program (or lemma) satisfies the specification if:

[
\forall x, y.; Pre(x) \land Exec(x, y) \Rightarrow Post(x, y)
]

---

### Definition 2 (Entailment)

We write:

[
S \vdash \varphi
]

to denote that the property ( \varphi ) is **entailed by the specification**, i.e., derivable from ( Pre ) and ( Post ) under the underlying logic of the verifier.

---

## 2️⃣ Queries as Semantic Probes

### Definition 3 (Query)

A **query** is a logical formula:

[
\varphi(x, y)
]

constructed to probe the semantic boundary of a specification.

We consider three classes of queries:

* **Behavioral queries**: relations between ( x ) and ( y )
* **Boundary queries**: predicates over ( x )
* **Logical queries**: arbitrary formulas over ( x, y )

---

### Definition 4 (Query Generator)

A **query generator** is a function:

[
G : S \rightarrow \mathcal{Q}
]

that maps a specification ( S ) to a set of queries ( \mathcal{Q} ).

---

## 3️⃣ Intended vs. Unintended Semantics

### Definition 5 (Intended Semantics)

Let ( \mathcal{I} \subseteq \mathcal{Q} ) denote the set of **intended properties**, i.e., properties consistent with user intent.

---

### Definition 6 (Unintended Semantics)

Let:

[
\mathcal{U} = \mathcal{Q} \setminus \mathcal{I}
]

be the set of **unintended properties**.

> ⚠️ Note: In practice, ( \mathcal{I} ) is unknown and approximated via test/query generation.

---

## 4️⃣ Consistency

### Definition 7 (Specification Consistency)

A specification ( S ) is **consistent** iff:

[
\forall \varphi \in \mathcal{U}, \quad S \nvdash \varphi
]

That is:

> The specification does not entail any unintended property.

---

### Definition 8 (Relative Consistency under Queries)

Given a query set ( \mathcal{Q}' \subseteq \mathcal{Q} ), we say ( S ) is **consistent w.r.t. ( \mathcal{Q}' )** iff:

[
\forall \varphi \in \mathcal{Q}', \quad S \nvdash \varphi
]

---

## 5️⃣ Inconsistency Detection

### Definition 9 (Inconsistency Witness)

A query ( \varphi ) is an **inconsistency witness** if:

[
\varphi \in \mathcal{U} ;\land; S \vdash \varphi
]

---

### Definition 10 (Test-based Detection)

Given a test ( t ) encoding a property ( \varphi ):

* If ( t ) is labeled **SHOULD FAIL** but verification succeeds, then:

[
S \vdash \varphi
]

and ( \varphi ) is an inconsistency witness.

---

## 6️⃣ Three Classes of Inconsistency

我们现在把 R1 / mutation / R5 **形式化统一**

---

### Definition 11 (Boundary Inconsistency)

A **boundary inconsistency** occurs if:

[
\exists x.; \neg Pre(x) ;\land; S \vdash Valid(x)
]

即：

> 非法输入被规范接受

---

### Definition 12 (Behavioral Inconsistency)

A **behavioral inconsistency** occurs if:

[
\exists x, y.; Pre(x) \land \neg Post(x, y) ;\land; S \vdash (x \rightarrow y)
]

即：

> 错误输出被规范允许

---

### Definition 13 (Logical Inconsistency)

A **logical inconsistency** occurs if:

[
\exists \varphi \notin \mathcal{I}, \quad S \vdash \varphi
]

即：

> 规范蕴含了不符合意图的逻辑性质

---

## 7️⃣ Consistency Checking as Falsification

### Definition 14 (Consistency Checking Problem)

Given:

* a specification ( S )
* a query generator ( G )

the goal is to find:

[
\varphi \in G(S) ;\text{s.t.}; S \vdash \varphi
]

---

### Definition 15 (Soundness of Detection)

A consistency checker is **sound** if:

[
\text{if it reports } \varphi, \text{ then } S \vdash \varphi
]

---

### Definition 16 (Completeness under Query Space)

A checker is **complete w.r.t. ( \mathcal{Q}' )** if:

[
\forall \varphi \in \mathcal{Q}', \quad
(\varphi \in \mathcal{U} \land S \vdash \varphi)
\Rightarrow
\text{it reports } \varphi
]

---

## 8️⃣ Key Theoretical Insight（可以写 theorem-style）

### Theorem 1 (Mutation Testing as a Special Case)

Behavioral mutation testing corresponds to consistency checking over a restricted query space:

[
\mathcal{Q}_{mut} = { (x \rightarrow y') \mid y' \neq y }
]

---

### Theorem 2 (Semantic Generalization)

Our framework generalizes mutation testing to the full query space:

[
\mathcal{Q}_{mut} \subset \mathcal{Q}
]

thus enabling detection of logical inconsistencies beyond behavioral errors.

---

## 🎯 最后一行（可以放 paper）

> **Consistency checking reduces specification validation to detecting unintended entailments over a structured query space.**

---

# 🔥 这一套定义的价值

你现在具备：

### ✅ 理论层

* entailment（S ⊢ φ）
* query space（Q）
* consistency（拒绝 φ）

---

### ✅ 方法层

* R1 / mutation / R5 → Definition 11–13

---

### ✅ 系统层

* Algorithm 1（debugging loop）

---

👉 这已经是一个**完整的 research stack**：

> **Definition → Framework → Algorithm → Implementation**

---

# 🚀 如果你想再往上冲一层（顶会杀招）

我可以帮你再加：

### 👉 一个更“PL 味”的 theorem：

比如：

* monotonicity of inconsistency detection
* minimal counterexample property
* relation to refinement / abstraction

或者：

👉 写一个 **Problem Statement + Contributions（论文第一页）**

只要你说一句 👍
