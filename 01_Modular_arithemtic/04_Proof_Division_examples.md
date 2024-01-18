# Division in Modular arithemetic

### Introduction to Division in MA.

While solving congurent in division, there are three possible outcomes:

- Unsolvable, no solutions
- One solution
- Multiple solutions

***Congurent form:***

$$
ax + b \cong \pmod{m}
$$

In order to reconcile the situation, we have to answer following two questions. 

- Is $a, m$ have $GCD = 1$? 
- Is $GCD(a,m)|b$ (a is divisor of b)?

***1. Unsolvalbe, no solutions.***

$2x \cong 51 \pmod{8}$

- First reconcile, $GCD(2,8) = 2$
- Second reconcile, $2|51$,Ans: No.

In this case, there is no solution to this congurent.
#

***2. one solution.***

$2x \cong 13 \pmod{7}$

- First reconcile, $GCD(2,7) = 1$
- Second reconcile, $1|13$,Ans: ***Yes***.

Tips: If GCD is 1, there is one solution. In this case,

$2x \cong 13 \pmod{7}$

$2x \cong 6 \pmod{7}$

$\frac{2x}{2} \cong \frac{6}{2} \pmod{7}$

$x \cong 3 \pmod{7}$
#

***3. Multiple solution (Three solutions).***

$9x \cong 42 \pmod{6}$

First reconcile, $GCD(9,6) = 3$

Second reconcile, $3|42$,Ans: ***Yes***.

$9x \cong 42 \pmod{6}$

$\frac{9x}{3} \cong \frac{42}{3} \pmod{\frac{6}{3}}$

$3x \cong 14 \pmod{2}$

$3x \cong 0 \pmod{2}$

$\frac{3x}{3} \cong \frac{0}{3} \pmod{2}$

$x \cong 0 \pmod{2}$

Another ways to solve using parameterize method,

$$
Y_{1T} = 2.T + R
$$

which is $b = q . t + r$, in this case:

$x \cong 6 \pmod{7}$

$b = 7*t + 6$

$6 = 7*(0) + 6$

$13 = 7*(1) + 6$

$20 = 7*(2) + 6$

So, $6 \cong 13 \cong 20 \pmod{7}$
#



***If*** 
$$
ad \cong bd \pmod{n}
$$

***Then,***

$$
a \cong b (\frac{\pmod{n}}{\text{gcd of d and n}})
$$

***If*** $\text{gcd of d and n}$ ***is 1, d and n are relatively prime. Then normal division applied:*** (Following statement is only true when gcd of d & c is)

$$
a \cong b \pmod{n}
$$

#

a) $20 \cong 2 \pmod{6}$

- 20 is multiple of 10,2,5,4,20,1
- 2 is multiple of 1,2
- 6 is multiple of 3,2,1,6

Gratest common divisor is 2, then:

$$ \begin{array}{c}
20 \cong 2 \pmod{6} \\
10.2 \cong 2.1 \pmod{3.2} \\
\text{So, } 10 \cong 1 \pmod{3}
\end{array}
$$
#

b) Solve for x, $3x \cong 12 \pmod{7}$

- 3 is multiple of 1,3.
- 12 is multiple of 1,2,3,4,6,12
- 7 is multiple of 1,7.

GCD is 1, ***then***:

$$ \begin{array}{c}
3x \cong 12 \pmod{7} \\
\frac{3x}{3} \cong \frac{12}{3} \pmod{7} \\
x \cong 4 \pmod{7}
\end{array}
$$

#


c) Solve for x (Cycling), $3x \cong 4 \pmod{7}$


$$ \begin{array}{c}
3x \cong 4 \pmod{7} \\
3x \cong 4 \cong 11 \cong 18 \pmod{7} \\
3x \cong 18 \pmod{7} \\
x \cong 6 \pmod{7} \\
\end{array}
$$