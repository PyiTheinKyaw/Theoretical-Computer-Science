# Divisibility and euclidean algorithm

### Divisibility algorithm

Defination: Let $d,a$ be integers we say d is divisor of a there is an integer k so that

$$
a = dk
$$

**Integers d and a**: $d$ is the potential divisor, and $a$ is the number being divided.

Integer $k$: This factor ensures that the division results in another integer, upholding the concept of divisibility.

Equation $a = dk$: This equation states that $d$ evenly divides $a$, meaning **there's no remainder.**

***Example:***

- 12 is a divisor of 24 because 24 = 12 * 2 (k = 2).

- 7 is not a divisor of 20 because 20 / 7 leaves a remainder of 6.

***Properties***: let $d,a,b$ be integers and suppose  $d|a, d | b$. Then for any integers $x,y,d$ divides. So that,

$$
ax + by
$$

Which means If a number divides one part of a sum, it also divides the entire sum, as long as the other parts are integers.

for example: if 6 divides 12 (which it does), then it also divides 5x + 12y, regardless of the values of x and y.

### Proof that
- There is an integer $k$ so that $a = kd$.
- There is an integer $k'$ so that $b = k'd$.

So, $ax + by = (kd).x + (k'd).y = d(kx+k'y)$

whatever the folowing expresion between () is, we got the factor $d$. which means $d$ is the factor expression and we can $d|(kx+k'y)$.


#

## Find GCD by using Eculidean Algorithm

- Eculidean algorithm or Eculid's algorithm
- For computing the GCD aka HCF.


### 1. Understanding GCD.

Let's say we would like to find out what is the GCD of 12 and 33.

|-|12|33|
|---|---|---|
|Divisor| $1,2,3,4,6,12$| $1,3,11,33$ |
|Common Divisor|$1,3$||
|Greates common divisor (GCD)|3||

So, $GCD(12,33) = 3$

***GCD is 1 for two different prime number.*** For example $GCD(13,31)$.

|-|13|31|
|---|---|---|
|Divisor| $1,13$| $1,31$ |
|Common Divisor|$1$||
|Greates common divisor (GCD)|1||

#

### 2. Method 1: Basic idea of Eculidean Algorithsm

Let say, we are finding the GCD of 12 and 33.
In eculidean algoritsm, it's accept four parameters. $q$ is quotient, $a,b$ is the inputed two numbers, $r$ is remainder.

When we accept $a,b$, always put biggest number to a and smaller one to b. In this case, $a=33$ and $b=12$.

Then imagine the table with those four parameters. It will be look like:
|q|a|b|r|
|----|----|----|----|
||33|12||

Then, do the division operation a over b. whenever you find the b parameters with 0 as the value whatever is there in a. That's GCD.

So, $33 ÷ 12 = 2 \text{ R 9}$, then place $b$ as $a$ and $r$ as $b$. Do this operation until you find out $b$ is 0. After first division operation, result may looks like this:

|q|a|b|r|
|----|----|----|----|
|2|33|12|9|
||12|9||

Do, the operation until you find out the GCD.

|q|a|b|r|
|----|----|----|----|
|2|33|12|9|
|1|12|9|3|
|3|9|3|0|
|x|3|0|x|

Now, $b$ parameter becomes 0 and the value place in $a$ is GCD. In this case, GCD is 3. And we can say that $GCD(12,33) = 3$
#

### 3. Method 2: Abstract Eculidean Algorithsm

As prerequisite $a \gt b$.

Then recursively call the `Eculid_GCD` function to find out GCD by mod untils parameters b is 0. If Parameter b is zero then a is GCD.

```
Eculid_GCD(a,b):
    if b = 0 then,
        return a;
    else 
        return Eculid_GCD(b, a mod b)
```

Example find the GCD (50,12),

Here $a=50, b=12$

$$ \begin{array}{c}
GCD(a,b) = GCD(b, a \pmod{b}) \\
GCD(50,12) = GCD(12, 50 \pmod{12}) = GCD(12,2) \\
GCD(12,2) = GCD(2, 12 \pmod{2}) = GCD(2, 0) = 2
\end{array}
$$

Answer: The $GCD(50,12) = 2$.

#


## 4. Method 3 (Using division algorithm)
Remember division algorithm where $a$ and $q$ are non-negative integer and $m$ is modulos and $r$ is remainder.

$$
a = (q) * m  + r
$$

Example:

Let $a = 56, m = 15$,

$56 = (q) * 15 + r$, In order to find $q$ and $r$, we have to divide $\frac{a}{m}$.

Then we have to switch the coefficent $m$ into $a$, $r$ into $m$ and find $q$ and $r$ again. we got this, we will iterate this operation until we got $r$ is 0. whatever the value in $m$, it's GCD.

$56 = (3) * 15 + 11$

$15 = (1) * 11 + 4$

$11 = (2) * 4 + 3$

$4 = (1) * 3 + 1$

$3 = (3) * 1 + 0$

In this case, the value in $m$ position is 1, so, we can say $GCD(56,15) = 1$.