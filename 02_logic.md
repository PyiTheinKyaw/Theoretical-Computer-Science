# 2. Logic #
Two systems of logic are commonly used in mathematics: propositional logic and predicate logic.

1. Propositional Logic

- A proposition is a statement which is true or false (but never both).
- It also can’t contain variables, e.g. ($x \geq 5$ isn't proposition )
- Sentence fragments without verbs (e.g. “bright blue flowers”)
- arithmetic expressions (e.g. $5 + 17$ ), aren’t propositions

Because they don’t state a claim.

2. Predicate logic:  is an upgrade that adds variables. e.g. ($x \geq 5$ )
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

|p|q|$ q \implies p$|
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

### 1.9 Mechanical Rules ###