# Fermet Primality test

The Fermat Primality Test is a probabilistic (meaning not 100% reliable) test used to determine whether a given number is prime. It relies on **Fermat's Little Theorem**, which states:

"If p is a prime number and a is any integer relatively prime to p (a and p have no common factors except 1), then 

$$
a^{(p-1)} \cong 1 \pmod{p}
$$

Fermet Primality test's formula

$a^p-a \implies p$, where 'p' is prime if this is a multiple of 'p' for all $1 \leq a \lt p$.

**Example:**

Question 1: Is '5' prime?

Given that: $p = 5$ and we've to iterate for all $1 \leq a \lt p$, in this case, iterate over 1 to 4.

$a=1, 1^5 - 1 = 0$ (0 is multiple of 5 [true])

$a=2, 2^5 - 2 = 32 - 2 =30$ (30 is multiple of 5 [true])

$a=3, 3^5 - 3 = 243 - 3 = 240$ (240 is multiple of 5 [true])

$a=4, 4^5 - 4 = 1024 - 4 = 1020$ (1020 is multiple of 5 [true])

So, $p=5$ is prime.

***Draw back:*** in order to test given $p$ is prime or not, we have to iterate over from 1 to before $p$, so, if given $p$ is big prime number, we have to run massive loop.

