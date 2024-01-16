# Integer rings in Modular Arithemetic
Ring: The integer ring $Z_{m}$ consists of:
1. The set $Z_{m}=\{0,1,2,...,m−1\}$
2. Two operations “+” and “×” for all $a,b ∈ Z_{m}$ such that:

    - $a+b \cong c \pmod{m},(c \in Z_{m})$
    - $a×b \cong d \pmod{m},(d \in Z_{m})$
### Properties of rings.

1. We can **add and multiply** any two numbers and the result is always in the ring. A ring is said to be closed.

2. **Addition and multiplication are associative**, e.g., $a+(b+c)=(a+b)+c$, and $a·(b·c)=(a·b)·c$ for all $a,b,c \in Z_{m}$.

3. There is the **neutral element 0 with respect to addition**, i.e., for every element $a \in Z_{m}$ it holds that $a+0 \cong a \pmod{m}$.

4. For *any element $a$ in the ring, there is always the negative element $−a$* such that $a+(−a)≡0 \pmod{m}$ ,i.e., the **additive inverse always exists**.

5. There is the **neutral element 1 with respect to multiplication**, i.e., for every element $a \in Z{m}$ it holds that $a×1 \cong a \pmod{m}$.

6. The **multiplicative inverse exists only for some, but not for all, elements**. Let $a \in Z$,the inverse $a−1$ is defined such that

$$
a·a^{−1} \cong 1 \pmod{m}
$$

If an inverse exists for $a$, we can divide by this element since $\frac{b}{a} \cong b·a−1 \pmod{m}$.
#

## So, What's for additive inverse and multiplicative inverse?

In modular arithmetic, we don't have direct subtraction or division operations in the traditional sense. Instead, we ***use additive inverses to achieve subtraction-like results*** and ***multiplicative inverses to perform division-like operations***.

### **1. Substraction-like result.**

To "subtract" $b$ from $a \pmod{m}$, we add the ***additive inverse*** of b to a. The additive inverse of b is the number that, when added to b, leaves a remainder of 0 when divided by m.

**Example:**

To calculate $7 - 4 \pmod{5}$, ***we add the inverse of 4 (which is 1)*** to 7: $7 - 4 \cong 7 + 1 \cong 8 ≡ 3 \pmod{5}$.

Here is why, the set of mod 5 is $Z_{m} = \{0,1,2,3,4\}$. Then,

To solve this, $7 - 4 \pmod{5}$, we consider to replace with $-4$ with $1$ because properties no.4 (4. For *any element $a$ in the ring, there is always the negative element $−a$* such that $a+(−a)≡0 \pmod{m}$ ,i.e., the **additive inverse always exists**.) 

Note that: The term $(-a)$ is not negative $a$. In simpler terms, it's the **"reciprocal"** of the element that **"cancels out"** a when additied together.

According to properties 4, for each element a in the set, its **additive inverse** is the number $(-a)$ that, when added to a, leaves a remainder of 0 when divided by m which means additive inverse of $a$ have to be truth in following equation -

$$
a+(−a)≡0 \pmod{m}
$$

So, in order to solve $7 - 4 \pmod{5}$, we have to find $-a$, in this case what is the inverse of $-4$ where set is $Z_{m} = \{0,1,2,3,4\}, m = 5, a = 4$.

$4 + 1 \cong 0 \pmod{5}$, That's why the inverse of 4 is 1.

### **2. Division-like result.**

While modular arithmetic doesn't have a direct division operation, multiplicative inverses enable division-like results. To "divide" a by b modulo m, **we multiply a by the modular inverse of b**. 

Be aware of properties 5, the **multiplicative inverse exists only for some, but not for all elements** for only $a$ and $n$ have to be relatively prime each other.

**Example:** 

To calculate $10 ÷ 3 \pmod{7}$, we multiply 10 by the inverse of 3 (which is 5): $10 ÷ 3 \cong 10 * 5 \cong 50 \cong 1 \pmod{7}$

**How this is happen?**

**As first, we have to make sure inverse is esists:** In order to find the inverse of a, denoted as $(a^{-1})$, we've to make sure inverse is exists in set. In order to confrim we can use $\frac{b}{a} \cong b·a−1 \pmod{m}$ or $GCD(a,m)$. If $GCD(a, m) = 1$, then a has a multiplicative inverse modulo m.

Find the inverse, denoted as a⁻¹, using one of these methods:

- **Extended Euclidean Algorithm**: This algorithm efficiently f**inds both the GCD and the coefficients** that can be used to express the GCD as a linear combination of a and m. The **coefficient of a becomes the inverse**.

- **Trial and Error**: This method involves systematically multiplying a by numbers within the modulus range until you find a product that leaves a remainder of 1 when divided by m.

- **Table of Inverses**: For small moduli, you can pre-compute a table of inverses to look up values quickly.

**Only if inverse is exists in set**, inverse value have to make true following equation:

$$
a·a^{−1} \cong 1 \pmod{m}
$$

In this case, $3^{-1} = 5$ which solve this $(3 * 5 \cong 1 \pmod{7})$.


### Key Difference from Additive Inverses:

- While additive inverses *always* exist in integer rings modulo m, multiplicative inverses *only exist for certain elements*. This is a crucial distinction.

#


