***For any element a in the ring, there is always the negative element −a such that a + (−a) ≡ 0 mod m, i.e., the additive inverse always exists.***

Guess,

$6.x \cong 1 \pmod{9}$

$6.x$ have to be between {0,1,....,8}, because the integer ring $Z{m} = \{0, 1, 2, . . . , m − 1\}$

It takes some effort to find the inverse (usually employing the Euclidean algorithm). However, there is an easy way of telling whether an inverse for a given element a exists or not by using GCD.

in this case, GCD(6,9), only GCD is 1 means there is an element a exists.

#
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
a·a−1 \cong 1 \pmod{m}
$$

If an inverse exists for $a$, we can divide by this element since $\frac{b}{a} \cong b·a−1 \pmod{m}$.
#

## So, What's for additive inverse and multiplicative inverse?

In modular arithmetic, we don't have direct subtraction or division operations in the traditional sense. Instead, we ***use additive inverses to achieve subtraction-like results*** and ***multiplicative inverses to perform division-like operations***.

### **1. Substraction-like result.**

To "subtract" $b$ from $a \pmod{m}$, we add the ***additive inverse*** of b to a. The additive inverse of b is the number that, when added to b, leaves a remainder of 0 when divided by m.

Example: To calculate $7 - 4 \pmod{5}$, ***we add the inverse of 4 (which is 1)*** to 7: $7 - 4 \cong 7 + 1 \cong 8 ≡ 3 \pmod{5}$.

Here is why, the set of mod 5 is $Z_{m} = \{0,1,2,3,4\}$. Then,

To solve this, $7 - 4 \pmod{5}$, we consider to replace with $-4$ with $1$ because properties no.4 (4. For *any element $a$ in the ring, there is always the negative element $−a$* such that $a+(−a)≡0 \pmod{m}$ ,i.e., the **additive inverse always exists**.) 

Note that: The term $(-a)$ is not negative $a$. In simpler terms, it's the **"reciprocal"** of the element that **"cancels out"** a when additied together.

According to 

#

- **Multiplicative Inverse:**

    **Here's a breakdown of the concept of multiplicative inverses in integer rings modulo m:**

**1. Multiplicative Inverse:**

- In mathematics, a multiplicative inverse of an element a is another element, typically denoted as a^(-1), that, when multiplied by a, yields the identity element for multiplication (which is 1 in most cases).
- In simpler terms, it's the "reciprocal" of the element that "cancels out" a when multiplied together.

**2. Key Difference from Additive Inverses:**

- While additive inverses *always* exist in integer rings modulo m, multiplicative inverses *only exist for certain elements*. This is a crucial distinction.

**3. Definition in Integer Rings Modulo m:**

- In an integer ring modulo m, an element a has a multiplicative inverse a^(-1) if and only if:
    - a * a^(-1) ≡ 1 (mod m)
- This means that multiplying a and a^(-1) together leaves a remainder of 1 when divided by m.

**4. Determining Existence:**

- To determine if a multiplicative inverse exists for a given element a in modulo m, we use the concept of **greatest common divisors (GCDs):**
    - If the GCD of a and m is 1 (meaning they share no common factors other than 1), then a has a multiplicative inverse modulo m.
    - If the GCD of a and m is greater than 1, then a does not have a multiplicative inverse modulo m.

**5. Examples:**

- **Modulo 7:**
    - 3 has a multiplicative inverse (eventually 5), as 3 * 5 ≡ 1 (mod 7), a * a^(-1) ≡ 1 (mod m).
    - 6 does not have a multiplicative inverse, as the GCD of 6 and 7 is 1.
- **Modulo 10:**
    - Only 1, 3, 7, and 9 have multiplicative inverses modulo 10.

**6. Applications:**

- Multiplicative inverses are essential in:
    - Cryptography (e.g., RSA algorithm)
    - Number theory (e.g., solving congruences)
    - Abstract algebra (e.g., structure of fields and groups)

**7. Key Points:**

- The existence of multiplicative inverses depends on the specific element and the modulus m.
- They are crucial for various mathematical operations and applications that rely on division within integer rings modulo m.


