# Linear congruence

In modular arithmetic, a **linear congruence** is an equation of the form:

$$
ax \cong b \pmod{m}
$$

where:

* **a** and **b** are integers (constants)
* **x** is the unknown integer variable
* **m** is a positive integer called the **modulus**

The equation essentially describes a relationship between **x** and its remainders when divided by **m**. It states that when **ax** is divided by **m**, it leaves the same remainder as **b**.

Here are some key features of linear congruences:

* **Solutions:** They can have multiple solutions or no solutions depending on the values of **a, b, and m**.
* **Modulus:** The modulus dictates the "cycle" of possible remainders. For example, in mod 7, the remainders after division by 7 are 0, 1, 2, 3, 4, 5, and 6.
* **Addition and Subtraction:** Congruences can be added or subtracted without affecting the solutions.
* **Multiplication:** Multiplying both sides by an integer with an inverse modulo m preserves the solutions.

Understanding linear congruences has various applications in different fields, including:

* **Number theory:** Solving Diophantine equations, cryptography, coding theory
* **Computer science:** Hash functions, error correction codes
* **Abstract algebra:** Rings, fields

#

## Solve systems of linear congurence

Solve:
- $x \cong 2 \pmod{3}$
- $x \cong 2 \pmod{4}$

we can express $x \cong 2 \pmod{3}$ as $x = 3a + 2$. Here is why,

Rearrange the congruence to form an equation with terms: $a = m*k + b$ where:
- a is the original number
- m is the modulus
- k is an integer representing an unknown multiple of m
- b is the remainder

1. Less Common Multiple (LCM), 

- in mod 3, 4 have $3^1, 2^2$, so LCM is 12.

2. Substitute back to find specific solutions for x:

$$ \begin{array}{c}
x \cong 2 \pmod{3} \text{ can express as } x = 3a + 2, \\
x \cong 2 \pmod{4} \text{ can express as } x = 4b + 2,
\end{array}
$$
