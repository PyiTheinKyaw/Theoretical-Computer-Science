# Divisibility and euclidean algorithm

## Divisibility

Def: Let $d,a$ be integers we say d is divisor of a there is an integer k so that

$$
a = dk
$$

Prop: let $d,a,b$ be integers and suppose $d ÷ b$. Then for any integers $x,y,d$ divides. So that,

$$
ax + by
$$

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

### 2. Method 1: Eculidean Algorithsm

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

### 3. Method 2: Eculidean Algorithsm

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
