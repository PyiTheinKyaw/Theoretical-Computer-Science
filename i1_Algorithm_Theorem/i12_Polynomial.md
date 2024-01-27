# Ploynomials and irreducibility

**Polynomials**

Defination:

Let $x$ be an unknown variable. A polynomial is a function

$$
f(x) = a_nx^n + a_{n-1} x^{n-1} + \ldots + a_2x^2+a_1x + a_0
$$

The highest non-zero power of n is called the ***degree of f***.

We can assume that all of our coefficients $a_i$ lie in a field $F$.

For example, if each $a_i \in \mathbb{Z}$, we could alternatively says that $a_i \in \mathbb{Q}$.

Let $F[x]$ denote the set of polynomials with coefficients in $F$. We call this the set of ***polynomials of F***.

### Radicals

The roots of low-degree polynomials can be expressed using **arithmetic** and **radicals**.

For example, the roots of the polynomial $f(x) = 5x^2 - 18x^2 -27$ are

$$
x_{1,2} = \pm \sqrt{\frac{6 \sqrt{6} + 9}{5}}, 
x_{3,4} = \pm \sqrt{\frac{9 - 6\sqrt{6}}{5}}
$$

Remark: The operations of **arithmetic** and **radicals** are really the ***"only way"*** we have to write down generic complex numbers.

Thus, if there is some number that cannot be expressed using radicals, we have no way to express it, unless we invent a special symbol for it. (Example: $\pi, e$)

### Algebraic numbers

Defination: The complex number is ***algebraic***, if it is **the root** of some polynomial in $\mathbb{Q}[x]$. The set $\mathbb{A}$ of all algebraic numbers forms a field.

A number that is not algebaric over $\mathbb{Q}$(e.g., $\pi,e$) is called ***transcendental***.

Example:

Every number that can be expressed form the natural numbers using arithmetic and radicals is algebraic. For example, consider:

$x = 5\sqrt{1+ \sqrt{-3}} \longleftrightarrow x^5 = 1 + \sqrt{-3}$

$\longleftrightarrow x^5 -1 = \sqrt{-3}$

$\longleftrightarrow (x^5 -1)2 = -3$

$\longleftrightarrow x^{10} -2x^5 + 4 = 0$

**Algebraically closed**

This is some fun facts about Complex numbers.

Definition: A field $F$ is **algebraically closed** if for any polynomial $f(x) \in F[x]$, the roots of $f(x)$ lie in $F$.

Non-Example:

- $\mathbb{Q}$ is not algebraically closed because $f(x) = x^2 - 2 \in \mathbb{Q}[x]$ has a root $\sqrt{2} \notin \mathbb{Q}$

- $\mathbb{R}$ is not algebraically closed because $f(x) = x^2 + 1 \in \mathbb{R}[x]$ has a root $\sqrt{-1} \notin \mathbb{R}$

***Fundamental theorem of algebra***

The field $\mathbb{C}$ is algebraically closed.

Thus, every polynomial $f(x) \in \mathbb{Z}[x]$ completely factors or split over $\mathbb{C}$:

$$
f(x) = (x-r_1) (x-r_1) \ldots (x-r_n), r_i \in \mathbb{C}
$$

Conversely, if $F$ is not algebraically closed, then there are polynomials $f(x) \in F[x]$ that do not split into linear factors over $F$.

### Irreducibility

Definition: A polynomial $f(x) \in F[x]$ is reducible over $F$ if we can factor $f(x) = g(x)h(x)$ for some $g(x),h(x) \in F[x]$ of strictly lower degree. If $f(x)$ is not reducible, we say it is irreducible over F.

**Examples:**

$x^2 -x -6 = (x+2)(x-3)$ is reducible over $\mathbb{Q}$.

$x^4 + 5x^2 + 4 = (x^2+1)(x^2+4)$ is reducible over $\mathbb{Q}$, but it has no roots in $\mathbb{Q}$.

$x^3 - 2$ is irreducible over $\mathbb{Q}$. if we could factor it, then one of the factors would have degree 1. But $x^3-2$ has no root in $\mathbb{Q}$.

**Facts:**

- If $deg(f) \geq 1$ and has a root in $F$, then it's reducible over $F$.

- Every polynomial in $\mathbb{Z}[x]$ is reducible over $\mathbb{C}$.

- If $f(x) \in F[x]$ is a degree 2 or 3 polynomial, then $f(x)$ is reducible over $F$ if and only if $f(x)$ has root in $F$.


### Lemma

There are no easy to tell polynomial is irreducible or not. But there are some useful technique to tell polynomial is ireeducible or not.

Lemma state that "let $f \in \mathbb{Z}[x]$ be irreducible. Then $f$ is also irreducible over $\mathbb{Q}$".

Because Lemma is very useful for proving the "**Eisenstein's criterion**".

***Eisenstein's criterion Theorem*** states that -

A polynomial $f(x) = a_nx^n + a_{n-1}x^{n-1} + \ldots + a_1x + a_0 \in \mathbb{Z}[x]$ is irreducible if for some prime $p$, the following all hold:

- $p \nmid a_n$ ($p$ can't be a divisor of leading coefficient.)
- $p | a_k \text{ for } 0,\ldots, n-1$;
- $p^2 \nmid a_0$; ($p^2$ cannot be a divisor of last coefficient.) 

But "Eisenstein's criterion" doesn't cover all of the "Irreducible polynomial".

Example: 

According to "Eisenstein's criterion" states "$x^10 + 4x^7 + 18x + 14$" is irreducible.

Because let's pick p as 2. 

- $2 \nmid 1$ which is true.
- $n=3, p|a_k = 2|14 = 2|18= 2|4$ which is true.
- $p^2 \nmid a_0, 2^2 \nmid 14$ which is also true.

But If ***Eisentein's criterion*** fails for all prime $p$, that does not necessarily imply that $f$ is reducible. For example, $f(x) = x^2 + x + 1$ is reducible over $\mathbb{Q}$ but Eisenstein cannot detect it.