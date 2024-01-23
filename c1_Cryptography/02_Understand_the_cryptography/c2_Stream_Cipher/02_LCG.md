# Linear Congruential Generator Method (LCG)

A Linear Congruential Generator (LCG) is a widely used pseudo-random number generator algorithm. It generates a sequence of seemingly random numbers based on a simple mathematical formula. While not truly random, LCGs are efficient and easy to implement, making them suitable for various applications, including simulations, games, and cryptography (with careful design).

Let $m$ be the large integer, Choose two interger $a$ and $b$, select seed $s_0$.

In cryptography, they used following one

$$
S_0 = \text{seed} \\
S_{i+1} \cong AS_i + B \pmod{m}, i = 0,1,2,\ldots
$$

For simility sake, followin equations are the same:

$$
S_{i} \cong A.S_{i-1} + B \pmod{m}
$$

**Example:**

Let $m = 123, A=5, B=2, s_0 = 73$

$s_1 \cong 5*73 + 2 \cong 365+2 \cong 367 \cong 121 \pmod{123}$

$s_2 \cong 5*121 + 2 \cong 607 \cong 115 \pmod{123}$

$s_3 \cong 5*115 + 2 \cong 85 \pmod{123}$

$s_4 \cong 5*85 + 2 \cong 58 \pmod{123}$

In order to get the result in turns of bits, we have to do:

$s_1 = 121, b_1 \cong 121 \cong 1 \pmod{2}$

$s_2 = 607, b_2 \cong 607 \cong 1 \pmod{2}$

$s_3 = 85, b_3 \cong 85 \cong 1 \pmod{2}$

$s_4 = 58, b_4 \cong 58 \cong 0 \pmod{2}$

So, result is b = {$1,1,1,0$}. 

#