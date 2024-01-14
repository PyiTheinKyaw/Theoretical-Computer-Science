# Division in Modular arithemetic

### Introduction to Division in MA.

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

- There are no common divisor and also normal division doesn't work.

$$ \begin{array}{c}
3x \cong 4 \pmod{7} \\
3x \cong 4 \cong 11 \cong 18 \pmod{7} \\
3x \cong 18 \pmod{7} \\
x \cong 6 \pmod{7} \\
\end{array}
$$