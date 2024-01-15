For any element a in the ring, there is always the negative element −a such that a + (−a) ≡ 0 mod m, i.e., the additive inverse always exists.

Guess,

$6.x \cong 1 \pmod{9}$

$6.x$ have to be between {0,1,....,8}, because the integer ring $Zm = \{0, 1, 2, . . . , m − 1\}$

It takes some effort to find the inverse (usually employing the Euclidean algo-
rithm). However, there is an easy way of telling whether an inverse for a given element a exists or not by using GCD.

in this case, GCD(6,9), only GCD is 1 means there is an element a exists.

### Properties of rings.

**Multiplicative Inverse:**

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


