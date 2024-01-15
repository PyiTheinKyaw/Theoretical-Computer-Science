# Congruence in modular arithmetic

## $a \cong b \pmod{n} $

Pharase: "a is congruent to b module n".

- "a ကို n နဲ့စားတဲ့အခါ အကြွင်း Remainder b ရတယ်" 
- Two numbers are congruent modulo n if they leave the same remainder when divided by n.

List 3 ways to represent what this means,

1. $a-b$ is multiple of $n$
2. $a$ is $b$ more than multiple of n. : ***means that when a is divided by n, it leaves a remainder of b***.
3. $a ÷ n$ has same remainder as $b ÷ n$.

#

### Verify $15 \cong 3 \pmod{6}$ using 3 ways above.

$$
\begin{array}{c}
15 \cong 3 \pmod{6}, 
\text{let a = 15, b = 3, n = 6} \\
\end{array}
$$

- According to list-1, $a-b$ is multiple of $n$, $15 - 3 = 12$, since $12$ is multiple of n. (Correct.)

- Accordting to list-2, $a$ is $b$ more than multiple of n,
$15$ is $3$ more than multiple of $6$. (correct)

- According to list-3, $a ÷ n$ has same remainder as $b ÷ n$, $15 ÷ 6$ has same remainder as $3 ÷ 6$ (Correct),

    $$
        15 ÷ 6 = 2 \text{ R 3}, 
        3 ÷ 6 = 0 + \frac{3}{6} = 0 \text{ R 3}
    $$
#

### Finding Serveral Congruences

Finding several congruences can involve different methods depending on the specific problem and the desired outcomes. Here are a few general approaches:

1. Brute Force:

    List all possible values for one variable within the congruence modulo and check if the equation holds true for each value. This approach is straightforward but can be tedious for large numbers or complex expressions.

2. Modular Addition/Subtraction:

    Use the properties of modular arithmetic to manipulate the equation and attempt to isolate one variable. This requires understanding modular addition, subtraction, and multiplication laws.

3. Modular Inverse:

    Find the modular inverse of a coefficient to multiply both sides of the equation and simplify. This requires calculating the modular inverse, which can be done using algorithms like the **Extended Euclidean Algorithm**.

4. Chinese Remainder Theorem:

    When dealing with multiple congruences with different moduli, apply the Chinese Remainder Theorem to find a solution that satisfies all congruences simultaneously. This method works well for systems of linear congruences but requires understanding and applying the theorem's calculations.

5. Computer Algebra Systems:

    Utilize symbolic algebra software like Wolfram Alpha or Maxima to symbolically solve the equations and obtain all possible solutions. This option is convenient for complex expressions but might not offer a step-by-step explanation of the solution process.


### Find serveral congruences to $3 \pmod{7}$

***By using Modular Addition/Subtraction:***

$$
\ldots \cong -11 \cong -4 \cong 3 \cong 10 \cong 17 \ldots
$$

Verify $-11$ is congruent to $3$ in mod $7$, $-11 \cong 3 \pmod{7}$

- List 1: $-11-3$ is multiple of $7$, $-11-3 = -14$. (Correct.) 
- List 2: $-11$ is $3$ more than multiple of 7. (Correct.)
- List 3: $-11 ÷ 7$ has same remainder as $3 ÷ 7$. (Correct.)

Wrong Calculation: (***When you get the remainder, R've to be positive in Number theory***)

$$
-11 ÷ 7 =  - \frac{7}{11} = - \frac{7}{7} - \frac{4}{7} \text{ WRONG}
$$

Correct Calculation for $a ÷ n$:

$$
-11 ÷ 7 =  - \frac{11}{7} = - \frac{14}{7} + \frac{3}{7} = - \frac{14}{7} \text{ R 3}
$$

Calculation for $b ÷ n$:
$$
3 ÷ 7 =  \frac{7}{11} = 0 + \frac{3}{7} = 0 \text{ R 3}
$$
#
## Mod n set of integers and "MOD OUT"

"Substitute with any number but in the list the set of integers needed for $\pmod{6}$" which means we are listing the integers with cycle 6.

1. Always start with 0.
2. Always end one less than mod.

Then, We go the following set,
$$
\{0,1,2,3,4,5\}
$$

### Modding Out

- "Modding out" is a progress used to find the non-negative integer which is congruent to $x \pmod{y}$

1. "Mod out" 22 in mod 6, $22 ÷ 6 = \text{3 R4}, 22 \cong 4 \pmod{6}$
2. "Mod out" 7631 in mod 6, $7631 ÷ 6 = \text{1276 R5}, 7631 \cong 5 \pmod{6}$

#

## Congruence Properties

If $a \equiv b \pmod{n}$, then the following 6 properties will be hold.

1) **Add propertie :** $a + c \equiv b + c \pmod{n}$

2) **Substract propertie :** $a - c \equiv b - c \pmod{n}$

3) **Multiply propertie :** $a.c \equiv b.c \pmod{n}$

4) **Exponent propertie :** $a^c \equiv b^c \pmod{n}$

5) **Distribute properties:**

$$
(a+b) \pmod n  \equiv a \pmod{n} + b \pmod{n}
$$

6) **Factor properties:**

$$
(a.b) \pmod n  \equiv a \pmod{n} .  b \pmod{n}
$$

***Note: Division Properties is much more complicated in congruences. In general division is not defined, But there are some special cases to consider.***
#
