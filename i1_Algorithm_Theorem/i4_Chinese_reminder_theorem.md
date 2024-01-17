# Chinese reminder theorem

The Chinese Remainder Theorem (CRT) is used to solve a set of different congruent equations with one variable ***BUT different moduli which are relatively prim***e as shown below:

$X \cong a_{1} \pmod{m_{1}}$

$X \cong a_{2} \pmod{m_{2}}$

$\ldots$

$X \cong a_{n} \pmod{m_{n}}$

CRT states that above equations **have a unique solution of the moduli are relatively prime**.

So, equation is:

$$
\sum_{i=1}^{n} a_i, M_i = (a_{i}.M_{i}.M_{i}^{-1}) \pmod{M}
$$

we can express as,
$$
X = (a_{1}.M_{1}.M_{1}^{-1}) + (a_{2}.M_{2}.M_{2}^{-1}) + \ldots +(a_{n}.M_{n}.M_{n}^{-1}) \pmod{M}
$$

***Example:***

***Solve the following congruent using CRT.***

$X \cong 2 \pmod{3}$

$X \cong 3 \pmod{5}$

$X \cong 2 \pmod{7}$

***Solution:***

$$
\sum_{i=1}^{3} a_i, M_i = (a_{i}.M_{i}.M_{i}^{-1}) \pmod{M}
$$

***Given that:***

let $a_1 = 2, a_2 = 3, a_3 = 2$ and $m_1 = 3, m_2 = 5, m_3=7$ respectively.

We have to find {$M_1, M_2, M_3, M_1^{-1}, M_2^{-1}, M_3^{-1}, M$} as first.

**Formula to find $M, M_i$,**

1. $M = m_1 * m_2 * m_3$

2. $M_i = \frac{M}{m_i}$

Let's substitute formula 1 first,

$M = 3 * 5 * 7 = 105$.

Now we can calculate $M_i$,

$M_1 = \frac{105}{3} = 35$

$M_2 = \frac{105}{5} = 21$

$M_3 = \frac{105}{7} = 15$

Now, Let's find the inverse of M ($M^-1$). In order to find inverse, we have to use following formula.

$$
M_i.M_i^{-1} \cong 1 \pmod{m_i}
$$

$35.M_i^{-1} \cong 1 \pmod{3}$, 

Check GCD is 1 or not. $GCD(35,3) = 1$ that means there is inverse in integer ring. ***(Please use EEA, if it's hard to find the M.I)***

***Find $M_1^{-1}$***

$35.M_1^{-1} \cong 1 \pmod{3}$

$35.2 \cong 1 \pmod{3}$, 

So $M_1^{-1} = 2$.
#

***Find $M_2^{-1}$***

$21.M_2^{-1} \cong 1 \pmod{5}$

$21.1 \cong 1 \pmod{5}$, 

So $M_2^{-1} = 1$.

#

***Find $M_3^{-1}$***

$15.M_3^{-1} \cong 1 \pmod{7}$

$15.1 \cong 1 \pmod{7}$, 

So $M_3^{-1} = 1$.
#

***Find $X$,***

In order to find $X$, 

$X \cong (a_{1}.M_{1}.M_{1}^{-1}) + (a_{2}.M_{2}.M_{2}^{-1}) + (a_{3}.M_{3}.M_{3}^{-1}) \pmod{M}$

$X \cong (2*35*2) + (3*21*1) + (2 * 15 * 1) \pmod{105} $

$X \cong 140 + 63 + 30 \pmod{105}$

$X \cong 233 \cong 23 \pmod{105}$,

**Answer x is 23.**

CRT allow us to find X with different congruent but moduli are relatively prime.