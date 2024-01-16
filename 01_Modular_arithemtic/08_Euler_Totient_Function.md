# Euler's Totient Function (Phi Function)

Denoted as $\varphi(n)$. **"Phi of n"** which means number of positive integers less than 'n' that are relatively prime to n.

**Example:**

Find $\varphi(5)$, $N = \{1,2,3,4\}$, Find how many number are relatively prime to n (in this case 5).

|$GCD$|Relatively prime?|
|---|---|
|GCD(1,5)|True|
|GCD(2,5)|True|
|GCD(3,5)|True|
|GCD(4,5)|True|

So, the answer for $\varphi(5) = 4$ which is the total valid count that are relatively prime.

#

Find $\varphi(11)$, $N = \{1,2,3,4,5,6,7,8,9,10\}$, Find how many number are relatively prime to n (in this case 11).

|$GCD$|Relatively prime?|
|---|---|
|GCD(1,11)|True|
|GCD(2,11)|True|
|GCD(3,11)|True|
|GCD(4,11)|True|
|GCD(5,11)|True|
|GCD(6,11)|True|
|GCD(7,11)|True|
|GCD(8,11)|True|
|GCD(9,11)|True|
|GCD(10,11)|True|

So, the answer for $\varphi(11) = 10$ which is the total valid count that are relatively prime.
#

## Alternative formula

In order to solve '***Euler's Toient Function***', Formula are as follow:

||Criteria of 'n'|Description|Formula|
|---|---|---|---|
|$\varphi(n)$|-|'n' is prime|$\varphi(n) = (n-1)$|
|$\varphi(n)$|$n = p * q$|p & q are primes | $\varphi(n) = (p-1) * (q-1)$|
|$\varphi(n)$|$n = a * b$|Either $a$ or $b$ is composite (or) Both $a$ and $b$ are composite. where $p_{1},p_{2},\ldots$ are distinct primes| $\varphi(n) = n* (1-\frac{1}{p_{1}}) (1-\frac{1}{p_{2}}) \ldots $|
---
### Case 1: n is prime

Find $\varphi(31)$. Solution:

Here $n = 31$. let say $a_{1}, a_{2}, \ldots, a_{30} \in \{1,2,3,\dots,30\}$.

'n' is prime nubmber, so formula is $(n-1)$ which is $31-1 = 30$. So there are 30 numbers that are less than 31 and relatively prime to 31.
#

### Case 2: product of primes
In this case p and q should be different prime number. Find $\varphi(35)$. Solution:

Here $n = 35$. let say $a_{1}, a_{2}, \ldots, a_{34} \in \{1,2,3,\dots,34\}$.

'n' is product of prime nubmbers, so formula is $(p-1) * (q-1)$ which is $(5-1)(7-1)$. So there are 24 numbers that are less than 35 and relatively prime to 35.

#
### Case 3: Composite

Find $\varphi(1000)$. 

***Solution:***

Here $n = 1000$. let say $a_{1}, a_{2}, \ldots, a_{999} \in \{1,2,3,\dots,999\}$.

$n = 1000 = 2^3 * 5^3$.

***Distinct prime factor*** are $2,5$. So $p_{1} = 2, p_{2} = 5$ respectvially. Remember the formula for this situation is:

$$
\varphi(n) = n * (1-\frac{1}{p_{1}}) (1-\frac{1}{p_{1}})\ldots
$$

***Then,***

$\varphi(1000) = 1000 * (1-\frac{1}{2}) (1-\frac{1}{5})$

$\varphi(1000) = 1000 * (\frac{1}{2}) (\frac{4}{5})$

$\varphi(1000) = 1000 * (\frac{4}{10})$ 

$\varphi(1000) = 400$

So there are 400 numbers that are less than 1000 and relatively prime to 1000.

#

### Case 4: (Similar to c3)


Find $\varphi(7000)$. 

***Solution:***

Here $n = 7000$. let say $a_{1}, a_{2}, \ldots, a_{6999} \in \{1,2,3,\dots,6999\}$.

$n = 7000 = 2^3 * 5^3 * 7^1$

***Distinct prime factor*** are $2,5,7$. So $p_{1} = 2, p_{2} = 5, p_{3} = 7$ respectvially. Remember the formula for this situation is:

$$
\varphi(n) = n * (1-\frac{1}{p_{1}}) (1-\frac{1}{p_{1}})\ldots
$$

***Then,***

$\varphi(7000) = 7000 * (1-\frac{1}{2}) (1-\frac{1}{5}) (1-\frac{1}{7})$

$\varphi(7000) = 7000 * (\frac{1}{2}) (\frac{4}{5}) (\frac{6}{7})$

$\varphi(7000) = 7000 * (\frac{4}{5}) (\frac{3}{7})$ 

$\varphi(7000) = 1400 * (4) (\frac{3}{7})$ 

$\varphi(7000) = 200 * 4 * 3$ 

$\varphi(7000) = 2400$

So there are 2400 numbers that are less than 7000 and relatively prime to 7000.
#

# Euler theorm

Euler theorm state that "*For every positive integer 'a' & 'n', which are said to be relative prime, then* 

$$
a^{\varphi(n)} \cong 1 \pmod{n}
$$

Euler's totient function is used to prove Euler theorm.

Euler theorm is true, when $a$ and $n$ is relative prime. which means $GCD(a,n) = 1$

***Example,***

Prove Euler's theorm hold true for $a=3, n=10$. As first, check a and n are relatively prime?

$GCD(a,n) = GCD(3,10) = 1$, then

$a^{\varphi(n)} \cong 1 \pmod{n}$

Let's prove that: $3^{\varphi(10)} \cong 1 \pmod{10}$ is true.

$3^{\varphi(10)} \cong 1 \pmod{10}$

***Solve for $\varphi(10)$ as frist,***

$\varphi(10) = 10 * (1-\frac{1}{2}) (1-\frac{1}{5})$

$\varphi(10) = 10 * \frac{1}{2} * \frac{4}{5}$

$\varphi(10) = 2 * 1 * 2$

$\varphi(10) = 4$

***Then let prove Eluer Theorm,***

$3^{\varphi(10)} \cong 1 \pmod{10}$

$3^{4} \cong 1 \pmod{10}$

$81 \cong 1 \pmod{10}$, Thefore, Euler's theorm holds true for $a=3, n=10$.