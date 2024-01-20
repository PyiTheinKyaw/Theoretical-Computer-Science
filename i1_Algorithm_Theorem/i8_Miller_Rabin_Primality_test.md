# Miller-Rabin Primarlity Test

The Miller-Rabin Primality Test, also known as Rabin-Miller Primality Test, is a probabilistic algorithm used to determine whether a positive integer is prime. Unlike the Fermat Primality Test, it offers higher accuracy with a very low chance of error.

This primarlity test is similar to **Fermat primarlity** or **Solovay-Strassen primarlity** test. In the terms of 'similar', they are "Checks whether a specific property, which is known to hold for prime values, holds for the number under testing.

### Liitation
Not 100% guaranteed: There is a small chance of a composite number passing the test for all chosen bases. However, this probability is negligible for most use cases.

### Application

Widely used in cryptography for generating large prime numbers needed for secure key generation and encryption algorithms. Applicable in various other fields like number theory, computer science, and coding theory.

### How algorithm works?

***Step 1:*** Find $n-1 = 2^k * m$.

***Step 2:*** Choose 'a' such that $1 \lt a \lt n-1$

***Step 3:*** Compute 
$$
b_0 = a^m \pmod{n}, \ldots, b_i = b^2_{i-1} \pmod{n}
$$

$$
b_0
\begin{cases}
Composite & \text{if } b_0 = +1 \\
\text{Probably Prime} & \text{if } b_0 = -1 \\
& \text{Otherwise, continue calculate } b_i
\end{cases}
$$
#
***Example:***

**Question: Is 561 prime?**

***Step 1: Find $n-1 = 2^k * m$.***

Given that $n = 561$ then,

$560 = 2^k * m$

Calculate 'k' and 'm',

$\frac{560}{2^1} = 280, \frac{560}{2^2} = 140, \frac{560}{2^3} = 70, \frac{560}{2^4} = 35, \frac{560}{2^5} = 17.5 \neq \text{Integer}$

So, $k = 4, m = 35$.

$560 = 2^4 * 35$
#
***Step 2:*** Choose 'a' such that $1 \lt a \lt n-1$

let $a = 2 \text{ for all a } 1 \lt a \lt n-1$.

#

***Step 3:*** Compute 
$$
b_0 = a^m \pmod{n}, \ldots, b_i = b^2_{i-1} \pmod{n}
$$

$$
b_0
\begin{cases}
Composite & \text{if } b_0 = +1 \\
\text{Probably Prime} & \text{if } b_0 = -1 \\
& \text{Otherwise, continue calculate } b_i
\end{cases}
$$

Compute $b_0 = a^m \pmod{n}$, where $a = 2, m = 35, n = 561$

$b_0 = 2^35 \pmod{561} = 263 \neq +1 \text{ or } -1$

So, continue $b_i = b^2_{i-1} \pmod{n}$

$b_1 = b^2_0 \pmod{561} = 263^2 \pmod{561} = 166 \neq +1 \text{ or } -1$

$b_2 = b_1^2 \pmod{561} = 166^2 \pmod{561} = 67 \neq +1 \text{ or } -1$

$b_3 = b_2^2 \pmod{561} = 67^2 \pmod{561} = 1 \text{ Composite}$

***Ans: 561 is composite.***