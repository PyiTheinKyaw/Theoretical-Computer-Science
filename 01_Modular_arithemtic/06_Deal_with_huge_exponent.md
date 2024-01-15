# How to deal with Really big exponent.

There are 3 methods to deal. They are -

1. Modular exponentiation
2. Fremat's little theorm
3. Euler's Theorm

Which to pick?

Assume you are dealing with following form:

$$
x^y \pmod{n} 
$$

If $n$ is ***prime***, then pick **"Fremat's little theorm"**.

Otherwise, we've to pick up "Modular exponentiation" or "Euler's theorm".
- If $x$ is small, use **"Modular exponentiation"**.
- Otherwise use **"Euler's theorm"**.

#

## Fermat's little theorm


***Fermat's Little Theorem states: If p is a prime number and a is any integer not divisible by p, then $a^{p−1} \cong 1 \pmod{p}$ .***

For example, $3^{1000} \pmod{23}$,

n is 23 and it's prime. So let's use ***"Fermet's little theorm"***.

***1. Calculate the exponent.***

$$ \begin{array}{c}
a^{p-1} \cong 1 \pmod{p} \\
p=23, p-1 = 23-1 = 22 \\
\end{array}
$$

We want $3^{22.k}$, where $k$ is constant we have to find so that it makes $3^{22.k} + 3^{i} = 3^{1000}$.

$33^{22.k} \rightarrow \frac{1000}{22} \rightarrow 45 \rightarrow 22.45 = 990 \rightarrow (33^{22.45} + 33^{10})$

***2. Substitute the exponent.***

$$ \begin{array}{c}
3^{1000} \pmod{23} \\
33^{22.45} .33^{10} \pmod{23} \\
1 . 33^{10} \pmod{23}  \text{ (According to fermat's little)} \\
\end{array}
$$

***3. Simplify.***

$$ \begin{array}{c}
3^{8}.3^{2} = 177147 \cong 8 \pmod{23}
\end{array}
$$
#



