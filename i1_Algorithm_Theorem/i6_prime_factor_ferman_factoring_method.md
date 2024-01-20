# Prime Factorization (Ferman Factoring Method)

Prime factorization is nothing but finding the prime factor of given number. For example, if given value is 10, then the factor of this value is {$1,2,5,10$} and $2$ and $5$ is the prime numbers between those set of factors. Those are called **"Prime factor"**. The way you factorization is called ***"Prime factorization"***.

The idea is find factor of given $n$, where $n = X . Y$.

Ferman Factoring algorithm have pro and cons. This algorithm works well when $X$ and $Y$ are close.

***Formula:***

$$
n = X^2 - Y^2
$$

$X^2 = n + Y^2$

$$
X = \sqrt{n+Y^2}
$$

Try different values for 'Y' from 1 up.
#

***Example:***

Factor n = 187.

***Solution:***

Find $X,Y$ as first,

$X = \sqrt{n+Y^2}$

$Y=1, X = \sqrt{187+1^2} = \sqrt{188} \neq \text{Integer}$,

$Y=2, X = \sqrt{187+2^2} = \sqrt{191} \neq \text{Integer}$,

$Y=3, X = \sqrt{187+3^2} = \sqrt{196} = 14$

$Y=3, X =14$

By using value of $X,Y$, find the factor of $n$.

$n = X^2 - Y^2$

$n = (X+Y) (X-Y)$

$n = (14+3) (14-3)$

$187 = 17 . 11$

***Answer the factor of $187$ is $17,11$.***

#
