# Group & Abelian Group in number theory

### Group

A group '$G$' denoted by {$G, \cdot$} is a set under some operation ($\cdot$) if it satisfies the ***CAIN properties***.

Which means, Group $G$ isn't set. But there are set of numbers under some operation (.) and that can satisfies the ***CAIN Properties***. That set of numbers are called Group($G$).

**CAIN Properties:**

- C: Closure 
- A: Associative
- I: Identity
- N: iNverse

### Abelian Group

A group is said to be Abelian if it already a group and ***Commutative property*** is also satisfied i.e $(a \cdot b) = (b \cdot a)$ for all $a,b$ in $G$.

|Property|Explanation|Description|
|---|---|---|
|Closure|$a,b \in G, \text{ then} (a.b) \in G$|$a,b$ is in Group then the result of ($a.b$) is also in the Group.|
|Associative|$a.(b.c) = (a.b).c \text{ for all} a,b,c \in G$| $a.(b.c) = (a.b).c$ is equal and a,b,c is belong to $G$.
|Identity Element|$(a.e) = (e.a) = a \text{ for all } a,e \in G$| the operation (a.e), (e.a) is equal to a and e,a also in $G$|
|Inverse Element|$(a.a') = (a'.a) = e \text{ for all } a,a' \in G$| (a,a') is equal to (a',a) the result equal to identity element and a,a' are belong to $G$.
|Commutative|$(a.b) = (b.a) \text{ for all } a,b \in G$| The operation (a.b) and (b.a) are the same and a,b is belong to $G$.
#

**Example:**

Question: Is $(Z,+)$ a group?

Solution:

$Z=${$\ldots,-3,-2,-1,0,1,2,3,\ldots$}, Z means "Set of all integers".

|Property|Explanation|Proof|T/F|
|---|---|---|---|
|Closure|$a,b \in G, \text{ then} (a.b) \in G$|$1 + 2 = 3$, 3 is also in $Z$|True|
|Associative|$a.(b.c) = (a.b).c \text{ for all} a,b,c \in G$|$1+(5+4) = (1+5)+4$|True|
|Identity Element|$(a.e) = (e.a) = a \text{ for all } a,e \in G$|$(5+0) = (0+5) = 5$|True|
|Inverse Element|$(a.a') = (a'.a) = e \text{ for all } a,a' \in G$|$(5+ -5) = (-5 + 5) = 0$|True|
|Commutative|$(a.b) = (b.a) \text{ for all } a,b \in G$|$(5+4) = (4+5)$|True|

Ans: $(Z,+)$ is a group and also Abelian group.