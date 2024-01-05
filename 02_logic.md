# 2. Logic #
Two systems of logic are commonly used in mathematics: propositional logic and predicate logic.

1. Propositional Logic

- A proposition is a statement which is true or false (but never both).
- It also can’t contain variables, e.g. ($x \geq 5$ isn't proposition )
- Sentence fragments without verbs (e.g. “bright blue flowers”)
- arithmetic expressions (e.g. $5 + 17$ ), aren’t propositions

Because they don’t state a claim.

2. Predicate logic:  is an upgrade that adds variables. e.g. ($x \geq 5$)

    Propositions are a helpful beginning but too rigid to represent most of the interesting bits of mathematics.

    To do this, we need predicate logic, which allows variables and predicates that take variables as input. A predicate is a statement that becomes true or false if you substitute in values for its variables.
#

### 1.1 Complex Proposition ###
Proposition statements can be joined together to make more complex statements. For example: "Justin is Male" and "Yangon is second major city of Myanmar".

To express complex sequences of statements without making everything long, represent each simple statement by a variable.

E.g, 'p' is "Pyi Thein Kyaw is Male", 'q' is "Yangon is second major city of Myanmar"
Then the whole long statement would be “p and q”. Or, using $shorthand$ notation $p ∧ q$.

Following are the Math symbol and their defination.

| Shorthand Notation | Defination | Description |
|---|---|---|
|$\land$| AND | The statement $p \land q$ is true when both p and q are true. If some of them are `false`, output is `false`.|
| $\lor$| OR (“inclusive")| $p \lor q$ is the shorthand for “p or q”, which is true when either p or q is `true`. Notice that it is also true when both p and q are true.
|$\oplus$| XOR ("exclusive") | Exclusive OR means something can only be one of two options, not both. Imagine a light switch: it's either on OR off, never both at the same time.
|$\neg$| NOT ("negating") | $\neg p$ is shorthand for "not p". $\neg p$ is true exactly when p is false. For Example, if 'p' is "Justin is Female", $\neg p$ is `true`. |
|$\rightarrow$ or $\implies$| Implication | Two propositions p and q can also be joined into the conditional statement. “if p, then q.”
|$\leftrightarrow$ or $\iff$| Biconditional Implication| Both are Implication.

#

### 1.2 Implication ###

Two propositions p and q can also be joined into the conditional statement. “if p, then q.”

- The proposition after the “if” (p in this case) is called the
**“hypothesis”** and 
- the proposition after “then” (q in this example) is called the **“conclusion.”** 

As in normal English, there are a number of alternative
ways to phrase the statement “if p, then q”, e.g. “p implies q” or “q follows
from p”.

The shorthand for this conditional is $p \implies q$ and its truth table is -

|p|q|$p\implies q$|
|---|---|---|
|T|T|T|
|T|F|F|
|F|T|T|
|F|F|T|

For example, “If Obama is president, then Obama lives in the White House” is `true` (at least in 2010). But “If Obama is president, then 2 > 4” is `false`. All the examples tend to be a bit artificial, because we don’t have variables yet.

In normal English, we tend not to use conditionals in which the “if” part is false. E.g. “If Japan is town, then I am Gay.” In mathematical English, such statements occur more often. And, worse, they are always considered `true`, no matter whether the “then” part is `true` or `false`.

For example, this statement is `true`: “If I am Gay, then 2 > 4.”

***The easiest way to remember the right output values for this operation is to remember that the value is `false` in exactly one case: when p is true and q is false. Other caes are `true`.***

In normal English if/then statements, there is frequently a flow of time
involved. Unless we make a special effort to build a model of time, propositional logic is timeless.  It’s not a big problem in mathematics, because mathematical proofs normally discuss a world that is static.

#

### 1.3 Implies and Conversly ###
The phrase ***“p implies q, and conversely”*** means that change the role of p and q. The converse of $p \implies q$ is $q \implies p$. But those two statements are not entirely equivalent. For Example,
- If I study hard, I will get good grades. 
- And conversely, if I get good grades, I must have studied hard.

This is not true biconditionally. It's possible to get good grades without studying hard (e.g., through luck). Converse is the logical inversion of the consequent, not always equivalent to the original statement.

|p|q|$q \implies p$|
|---|---|---|
|T|T|T|
|T|F|T|
|F|T|F|
|F|F|T|

#

### 1.4 Biconditional (“p if and only if q") ###
The shorthand for this is the biconditional operator $p \iff q$. Another common way to phrase the biconditional is ***“p if and only if q.”*** Which means refers to a statement that is true only when both the original statement (p implies q) and its converse (q implies p) are true.

"p implies q, and conversely" is not the same as "p if and only if q". While the first sentence implies both the original statement and its converse, it doesn't guarantee the converse is true. Consider:
- If it rains, the ground is wet.
- And conversely, if the ground is wet, it definitely rained.

This is true biconditionally.

To see this, compare the previous truth table with this one:

|p|q|$q \iff p$|
|---|---|---|
|T|T|T|
|T|F|F|
|F|T|F|
|F|F|T|

- Biconditional logic requires both the original statement and its converse to be true.
- Converse is the logical inversion of the consequent, not always equivalent to the original statement.
- "p if and only if q" is stronger than "p implies q, and conversely" because it demands equivalence in both directions.

#

### 1.5 Contrapositive Logic ###
The contrapositive of $p \implies q$ is formed by swapping the roles of p and q and negating both of them to get $\neg q \implies \neg p$. Unlike the converse, the contrapositive is equivalent to the original statement. Here’s a truth table showing why:

|p|q|$\neg q$| $\neg p$ |$\neg q \implies p$ |
|---|---|---|---|---|
|T|T|F|F|T
|T|F|T|F|F
|F|T|F|T|T
|F|F|T|T|T

Let’s consider what these variations look like in an English example:
- If it’s below zero, my car won’t start.
- converse: If my car won’t start, it’s below zero
- contrapositive: If my car will start, then it’s not below zero.

#

### 1.6 Complex Statement (complex set of propositions - compound propositions ) ###

Very complex statements can be made using combinations of connectives. E.g. “If it’s below zero or my car does not have gas, then my car won’t start and I can’t go get groceries.” This example has the form:

$$
(p \lor \neg q) \implies ( \neg r \land \neg s )
$$

Truth tables are a nice way to show equivalence for compound propositions which use only 2-3 variables. However, if there are k variables, your table needs $2^k$
lines to cover all possible combinations of input truth values. This is cumbersome for larger numbers of variables.

#

### 1.7 Logical Equivalence ($\equiv$) ###

Two (simple or compound) propositions p and q are logically equivalent if
they are true for exactly the same input values. The shorthand notation for
this is $p \equiv q$. 

- **A frequently useful fact is that $p \implies q$ is logically equivalent to $\neg p \lor q$.**

$T$ and $F$ are special constant propositions with no variables that are, respectively, always `true` and always `false`. So, since $p \land \neg p$ is always `false`, we have the following equivalence:

$$
p \land \neg p \equiv F
$$

Notice that, in mathematics, the equal operator $=$ can only be applied to objects such as numbers. When **comparing logical expressions that return true/false values, you must use $\equiv$**.

#

### 1.8 De Morgan’s Laws ###
**A frequently useful fact is that $p \implies q$ is logically equivalent to $\neg p \lor q$.**

Two very well-known equivalences are ***De Morgan’s Laws***. These state that
- $\neg(p \land q) \equiv \neg p \lor ¬q$. 
- $\neg (p \lor q) \equiv \neg p \land \neg q$.

Other Useful logical equivalences:
- $\neg (\neg p) \equiv p $
- $\neg (p \implies q) \equiv p \land \neg q$ 
- $p \implies q \equiv \neg p \lor q$

Similar rules in other domains (e.g. set theory) are also called ***De Morgan’s Laws***. They are especially helpful, because they tell you how to **simplify** the negation of a complex statement involving “and” and “or”.

#

### 1.9 Some Useful Logical equivalence (Commutativity and associativity) ###

Logical equivalences are powerful tools for simplifying arguments and proving statements in various fields. 

- $\lor$ and $\land$ are commutative, e.g. p ∧ q ≡ q ∧ p.

|Logical Equivalences|Description|
|---|---|
|$p \lor q \equiv q \lor p$|logical OR is commutative|
|$p \land q \equiv q \land p$|logical AND is commutative|
|$p \lor (q \lor r) \equiv (p \lor q) \lor r $|logical OR is associative|
|$p \land (q \land r) \equiv (p \land q) \land r$|logical AND is associative|


The distributive laws, however, work slightly differently from those in algebra where as in logic we have two rules:

$$
\begin{array}{c}
p \lor (q \land r) \equiv (p \lor q) \land (p \lor r) \\
p \land (q \lor r) \equiv (p \land q) \lor (p \land r) \\
\end{array}
$$

So, in logic, you can distribute either operator over the other. Also, arithmetic has a clear rule that multiplication is done first, so the righthand side doesn’t require parentheses. The order of operations is less clear for the logic, so more parentheses are required.

#

### 1.10 Negating propositions ###

An important use of logical equivalences is to help you correctly state the negation of a complex proposition, i.e. what it means for the complex proposition not to be true. Negating a claim is crucial in logic (and math) for several reasons:

**1. Exploring Alternative Truths:**

- Logic deals with exploring all possible truth values of statements and arguments. Negation allows us to consider the opposite of a claim, uncovering potential scenarios where the original claim might not hold true. This broadens our understanding of the statement and potential consequences.

**2. Constructing Valid Arguments:**

- Many logical arguments rely on negations to establish valid reasoning. Common argument forms like contrapositive and modus tollens use negation to logically derive conclusions from existing statements. These argument forms are essential for rigorous proofs and analysis in various fields.

**3. Identifying Contradictions:**

- Negation helps identify contradictions in a set of statements. If the negation of a claim logically follows from another claim, it reveals that both claims cannot be true together. This reveals inconsistent information and helps detect errors in reasoning.

**4. Specifying Precise Conditions:**

- Negation allows for precise formulation of conditions and constraints. By stating what is not true, we can more accurately define what must be true for a statement or argument to hold. This precision is crucial in fields like mathematics and computer science where clear definitions and relationships are vital.

**5. Building Complex Theorems:**

- Logic, including negation, provides the building blocks for constructing intricate theorems and proofs. Complex logical statements often involve nested negations to express nuanced conditions and relationships. The ability to manipulate and reason with negations becomes essential for advanced mathematical and logical reasoning.

**Overall, negation is not just about saying "no" to a claim. It's a powerful tool for exploring alternative truths, constructing valid arguments, identifying contradictions, specifying conditions, and building complex logical structures. Negation allows us to reason more comprehensively and rigorously, pushing the boundaries of what we can understand and prove.**

For example, suppose we have a claim like “If M is regular, then M is paracompact or M is not Lindel¨of.”

Let `r` be “M is regular”, `p` be “M is paracompact”, and `l` be “M is Lindel¨of.” Then the claim would be $r \implies (p \lor \neg l)$.

The negation of $r \implies (p \lor \neg l)$ is $\neg(r \implies (p \lor \neg l))$. 

$$
\begin{array}{c}
\neg(r \implies (p \lor \neg l)) \\
\equiv r \land \neg (p \lor \neg l) \\
\equiv r \land \neg p \land l \\
\end{array}
$$

So, The negation of $\neg(r \implies (p \lor \neg l))$ is  $r \land \neg p \land l $. So the negation of our original claim is “M is regular and M is not paracompact and M is Lindel¨of.” 
#

### 2.1 Quantifiers in Math: Exploring Existence and Universality 

Quantifiers are powerful tools in mathematics that allow us to talk about entire sets of numbers or objects at once. They help us express statements about "some" or "all" elements within a set, providing concise and powerful ways to describe mathematical properties. Here's a breakdown of the terms you mentioned:

**1. Existential Quantifier ($\exists$): "There exists..."**

- This symbol indicates that at least one element within a set satisfies a specific condition.
- For example, the statement $\exists x \in N, x^2 = 4$ means ***"there exists a natural number x such that its square is 4"***. The solution in this case is x = 2.
- This quantifier helps us express the possibility of an element with a certain property existing within the set.

**2. Universal Quantifier ($\forall$): "For all..."**

- This symbol indicates that every element within a set satisfies a specific condition.
- For example, the statement $\forall x \in R, x^2 ≥ 0$ means ***"for all real numbers x, their square is greater than or equal to 0"***. This is true for all real numbers.
- This quantifier helps us express the universality of a property, ensuring it holds true for every element without exception.

**3. Unique Existence Quantifier ($\exists !$): "There exists exactly one..."**

- This symbol combines the idea of existence with uniqueness. It indicates that exactly one element within a set satisfies a specific condition.
- For example, the statement $\exists !x \in Z, x^2 - 9 = 0$ means **"there exists exactly one integer x such that its square minus 9 is equal to 0"**. The solution is x = 3.
- This quantifier helps us express the existence of a single element with a specific property, eliminating ambiguity and ensuring its one-and-only existence within the set.

**Purpose of Quantifiers:**

- Quantifiers provide conciseness and clarity in expressing mathematical statements, allowing us to talk about entire sets of elements instead of listing them individually.
- They enable us to formulate general theorems and principles that apply to all or some elements within a specific set.
- They are crucial in various branches of mathematics, including calculus, number theory, and set theory.

**Understanding and utilizing quantifiers effectively is essential for building a strong foundation in mathematics. Practicing using them in different contexts will solidify your comprehension and unlock their power for describing intricate mathematical relationships.**

**Bonus Tip:** You can combine quantifiers to express even more complex relationships. For example, "∃x ∈ R ∀y ∈ N, x + y < 10" means "there exists a real number x such that for all natural numbers y, their sum is less than 10".

#

### 2.2 Useful Noation

If you want to state a claim about two numbers, you can use two quantifiers as in:

$$\forall x \in R, \forall y \in R, x + y \geq x \text{ (which is wrong!)}$$

In abbreviated form,
$$\forall x, y \in R, x + y \geq x$$

Shorthand of Qutifier are also use along with Implication,
$$\forall x, \text{if } p(x), \text{then } q(x)$$
#

### 2.3 Notation for other contents (Like in 2D points)

When writing mathematics that involves 2D points and quantifiers, you have several notational options.
- You can write something like $\forall x, y \in Z$ (“for any integers x and y”) and then later refer to the pair $(x, y)$. 
- Or you can treat the pair $(x, y)$ as a single variable, whose replacement set is all 2D points.

For example, the following says that the real plane ($R^2$) contains a point on the unit circle:

$$
\exists (x,y) \in R^2, x^2 + y^2 = 1
$$

Another approach is to write something like
$$
\exists p \in R^2, \text{ p is on the unit circle}
$$
When you later need to make precise what it means to be “on the unit circle,” you will have to break up p into its two coordinates. At this point, you say that since p is a point on the plane, it must have the form $(x, y)$, where x and y are real numbers. This defines the component variables you need to expand the definition of “on the unit circle” into the equation $x^2 + y^2 = 1$.

#

### 2.4 Negation statements with quantifiers
Suppose we've claim like $\forall x \in R, x^2 ≥ 0$. This claim will be `false` if there is at least one real number x such that $x^2 < 0$. Think about that, If x is Real number, what is the propositional statement of $0^2 <0$. It's `False`.

In general, a statement of the form **“for all x in A, P(x)”** is false exactly when there is some value x in A such that P(x) is `false`. In other words, when ***“there exists x in A such that P(x) is not true”***. In shorthand notation:

$$
\neg (\forall x, P(x)) \equiv \exists x, \neg P(x)
$$

this is a bit like the de Morgan’s laws: when you move the negation across the operator, you change it to the other similar operator. So, If we have something like this:
$$
\neg (\exists x, P(x)) \equiv \forall x, \neg P(x)
$$

And if we have something like this, 
$$\forall x, P(x) \implies (Q(x) \land R(x))$$

Its negation is

$$
\begin{array}{c}
\neg (\forall x, P(x) \implies (Q(x) \land R(x))) 
\equiv \exists x,\neg (P(x) \implies (Q(x) \land R(x))) \\
\equiv \exists x, P(x) \land \neg (Q(x) \land R(x)) \\
\equiv \exists x, P(x) \land (\neg Q(x) \lor \neg R(x))
\end{array}
$$

#
### 2.5 Binding and scope

A quantifier is said to “bind” the variable it defines. The “bound” variable in a quantification is only defined for a limited time, called the “scope” of the binding. This is usually the end of the quantified statement or the end of the line, but you sometimes have to use common sense about what the author intended. Parentheses are often used to indicate that
the author intends a shorter scope.

Variables in computer programs also have a “scope” over which their declaration is valid, e.g. the entire program, a single code file, or local to an individual function/procedure/method. If you try to use a variable outside the scope of its definition, you’ll get a compiler error.

**Free Variables:**

* **Definition:** A variable that doesn't have a specific value assigned to it within the context of a statement or expression. It remains "unbound," meaning its value can vary freely.
* **Example:** In the expression "x + y > 3," both `x` and `y` are free variables because they haven't been given definite values or restricted by quantifiers. The truth of the statement depends on the specific values chosen for `x` and `y`.

**Bound Variables:**

* **Definition:** A variable that has been "bound" to a specific range of values using a quantifier (like "for all" or "there exists") or assigned a definite value.
* **Examples:**
    * In the statement "∀x ∈ R, x^2 ≥ 0," the variable `x` is bound by the universal quantifier ∀, indicating it applies to all real numbers.
    * In the equation "y = 2x + 1," the variable `y` is bound to the value determined by the expression "2x + 1".

**Key Points:**

- Free variables allow for flexibility and generality in expressions and statements, representing potential variations in values.
- Bound variables provide specificity and clarity, ensuring that certain properties or relationships hold true within defined constraints.
- Understanding the distinction between free and bound variables is crucial for accurately interpreting and manipulating mathematical statements, especially in logic, set theory, and computer science.

**Additional Insights:**

- Free variables often play a key role in defining functions and formulas, where their values can be inputted to produce different outputs.
- Bound variables are essential for constructing proofs and theorems, as they allow us to establish general truths that hold for a defined set of elements.
- In programming, free variables are often used as parameters for functions, while bound variables are used within function bodies to represent specific values or data structures.
#
