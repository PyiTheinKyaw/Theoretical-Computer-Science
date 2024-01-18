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
#

### Example 1:

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
#

### Example 2:

***Solve the following congruent using CRT.***

$4X \cong 5 \pmod{9}$

$2X \cong 6 \pmod{20}$

***Simplify the 1st congrest as first:***

$4X \cong 5 \pmod{9}$

$4X * 4^{-1} \cong 4^{-1} * 5 \pmod{9}$

$X \cong 4^{-1} * 5 \pmod{9}$

$X \cong 4^{-1} \pmod{9} * 5 \pmod{9} $

$X \cong 35 \cong 8 \pmod{9}$

***Simplify the 2nd congrest as second:***

$2X \cong 6 \pmod{20}$

$X \cong 3 \pmod{20}$

Now, we got two congernt

$X \cong 8 \pmod{9}$

$X \cong 3 \pmod{20}$

***Given that,***

let $a_1 = 8, a_2 = 3$ and $m_1 = 9, m_2 = 20$ respectively.

We have to find {$M_1, M_2, M_1^{-1}, M_2^{-1}, M$} as first.


***Equation:***

$$
\sum_{i=1}^{3} a_i, M_i = (a_{i}.M_{i}.M_{i}^{-1}) \pmod{M}
$$

**Formula to find $M, M_i$,**

1. $M = m_1 * m_i$

2. $M_i = \frac{M}{m_i}$

**Find $M$,**

$M = 9 * 20 = 180$

**Find $M_i$,**

$M_1 = \frac{180}{9} = 20$

$M_2 = \frac{180}{20} = 9$


**Find Inverse $M_i^{-1}$,**

$$
M_i. M_i^{-1} \cong 1 \pmod{m_i}
$$

**For $M_{1}^{-1}$,**

$M_1. M_1^{-1} \cong 1 \pmod{m_1}$

$20 * M_1^{-1} \cong 1 \pmod{9}$

$20 * 5 \cong 1 \pmod{9}$

So, $M_{1}^{-1} = 5$

**For $M_{2}^{-1}$,**

$M_2. M_2^{-1} \cong 1 \pmod{m_2}$

$9 * M_2^{-1} \cong 1 \pmod{20}$

$9 * 9 \cong 1 \pmod{20}$

So, $M_{2}^{-1} = 9$

***Find $X$,***

$X \cong (a_{1}.M_{1}.M_{1}^{-1}) + (a_{2}.M_{2}.M_{2}^{-1}) \pmod{M}$

$X \cong (8 * 20 * 5) + (3 * 9 * 9) \pmod{180}$

$X \cong 800 + 243 \pmod{180}$

$X \cong 1043 \pmod{180}$

$X = 143$

**Answer x is 143.**