很好，这一步我们把两个思路**真正融合成一个“论文级 Framework”**，你基本可以直接放进 paper 的 **Method / Approach / Overview** 里。

我会给你一个**结构完整 + 术语统一 + 可画图**的版本👇

---

# 🧠 统一框架：Consistency Checking via Entailment-Guided Querying

---

## 1️⃣ 总体思想（Overview）

我们提出一个统一框架，用于评估形式化规范的完整性：

> **通过系统性地构造对抗性查询（queries），并检查规范是否错误地蕴含（entail）这些查询，从而检测规范的一致性。**

这个框架融合了两个核心视角：

* **Entailment 视角（理论）**：
  检查 `Spec ⊢ φ` 是否成立
* **Testing 视角（实现）**：
  生成 query 并观察是否被 spec 拒绝

---

## 2️⃣ 核心形式化定义

### 2.1 规范

设：

```text
Spec = (Pre, Post)
```

表示：

* Pre：前置条件（输入约束）
* Post：后置条件（输出约束）

---

### 2.2 查询（Query）

我们定义：

```text
Q : Spec → φ
```

其中：

* φ 是一个逻辑命题，可以是：

  * 输入是否合法
  * 输入输出关系
  * 任意可推导性质（logical claim）

---

### 2.3 判定函数（Check）

定义：

```text
Check(Spec, φ) =
    accept   if Spec ⊢ φ
    reject   otherwise
```

---

## 3️⃣ Consistency 的统一定义

我们将 consistency 定义为：

> 对所有“不符合意图”的命题 φ：

```text
Spec ⊬ φ
```

等价于：

```text
∀ φ ∈ BadQueries:
    Check(Spec, φ) = reject
```

---

## 4️⃣ 三类 Consistency 的统一刻画

我们将不同类型的 inconsistency，统一为：

> **构造不同类型的 φ（query）**

---

### 4.1 Behavioral Consistency

#### Query 形式：

```text
φ = (input → output)
```

#### 两类情况：

* 正确行为：

  ```text
  Spec ⊢ φ   （应成立）
  ```

* 错误行为（mutation）：

  ```text
  Spec ⊬ φ   （应拒绝）
  ```

👉 本质：

> mutation testing = 构造错误 φ

---

### 4.2 Boundary Consistency（R1）

#### Query 形式：

```text
φ = "input 是合法的"
```

例如：

```text
φ = valid(a = 0)
```

#### 检查：

```text
Spec ⊬ φ
```

👉 本质：

> 测试 Spec 是否错误接受非法输入

---

### 4.3 Logical Consistency（R5）

#### Query 形式：

```text
φ = 任意逻辑推论
```

例如：

* determinism：

  ```text
  φ = (b1 == b2)
  ```

* stronger property：

  ```text
  φ = (a > (1 << b))
  ```

#### 检查：

```text
Spec ⊬ φ
```

👉 本质：

> 测试 Spec 是否允许错误推理

---

## 5️⃣ 与 Mutation Testing 的关系（关键贡献点）

我们可以形式化说明：

---

### 传统 mutation testing：

```text
φ = (input → mutated_output)
```

👉 只在：

> **行为空间（behavioral space）**

---

### 我们的方法：

```text
φ ∈ 任意逻辑命题空间
```

👉 扩展到：

> **语义空间（semantic space）**

---

## 🔥 可以直接写进 paper 的一句话

> 我们将 mutation testing 从行为层推广到语义层，使其能够检测规范是否错误地蕴含不符合意图的逻辑结论。

---

## 6️⃣ Structural Coverage 的位置（降级为辅助）

在这个框架中：

> Structural coverage 不再是 consistency 的一部分

而是：

```text
Coverage(Spec, Proof/Test) → 哪些约束被使用
```

用于：

* 发现未被使用的 spec constraint
* 辅助生成 query（但不参与 correctness 定义）

---

## 7️⃣ Framework Pipeline（可以画图）

你可以画一个三层 pipeline（非常推荐）：

---

### Step 1️⃣ Query Generation

```text
Spec → Q(Spec) → φ
```

三类 generator：

* behavioral generator（mutation）
* boundary generator（R1）
* logical generator（R5）

---

### Step 2️⃣ Entailment Checking

```text
Check: Spec ⊢ φ ?
```

---

### Step 3️⃣ Consistency Decision

```text
if Spec ⊢ φ and φ is invalid:
    inconsistency detected
```

---

## 8️⃣ 一句话总结整个框架

> **Consistency checking = 系统性地生成“不应该成立的命题”，并检测规范是否错误地蕴含它们。**

---

# 🎯 你这一步的本质提升

你现在已经从：

❌ “我们设计了几类 test（R1 / R5）”

升级为：

✅ **“我们提出了一个统一的、可形式化的 consistency checking 框架，其核心是 entailment-guided query generation。”**

---

# 🚀 下一步我可以帮你做什么

如果你想冲顶会（PLDI / ICSE / FSE），下一步最关键的是：

### 👉 我可以帮你画一个“论文级 Figure 1”：

包含：

* Spec
* Query generators（三类）
* Entailment checker
* Inconsistency detection

👉 这个图会极大提升论文说服力（reviewer 很吃这一套）

只要你说一句：**“帮我画图（中文/英文）”**，我可以直接给你可用版本 👍
