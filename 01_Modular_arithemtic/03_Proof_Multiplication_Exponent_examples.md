## $a \cong b \pmod{n} $

Pharase: "a is congruent to b module n".

- "a ကို n နဲ့စားတဲ့အခါ အကြွင်း Remainder b ရတယ်" 
- Two numbers are congruent modulo n if they leave the same remainder when divided by n.
#
1. Proof that 121 is congruence to 1 in mod 4. (ဆိုလိုတာက 4 နဲ့စားရင် 121 က 1 နဲ့ Congruent ဖြစ်လား ?)

    Approach 1 (Division with Remainder): 
    $121 ÷ 4 = 30 \text{ R 1}, 121 \cong 1 \pmod{4}$
    . Both 121 and 1 leave a remainder of 1 when divided by 4.

    Aprroach 2(Subtracting Multiples of 4)

    1. Subtracting Multiples of 4:

        Subtract multiples of 4 from 121 until you reach a remainder between 0 and 3 (Set of mod 4 is $\{0,1,2,3\}$):

        $121 - 4 * 30 = 1, 121 \cong 1 \pmod{4}$

## Multiplication and Exponent
2. Find the remainder when the following are divided by 4:
- $15^{15}$
- $121.122.123$
- 100!

### $15^{15}$
$$ \begin{array}{c}
15 ÷ 4 = 3 \text{ R 3}, 15 \cong 3 \pmod{4} \\
15 \cong 3 \cong -1 \pmod{4} \\
(-1)^{15} = -1
\end{array}
$$


### $121.122.123$
$$ \begin{array}{c}
121 - 4 * 30 = 1, 121 \cong 1 \pmod{4} \\
121 \cong 1 \pmod{4} \\ 
122 \cong 2 \pmod{4} \\
123 \cong 3 \pmod{4} \\
121.122.123 \cong 1 \pmod{4} . 2 \pmod{4} . 3 \pmod{4} \\
\cong 6 \pmod{4} \\
\text{Mod out, } \cong 2 \pmod{4}
\end{array}
$$

### $100!$
$$ \begin{array}{c}
100! = 100.99! \\
100 \cong 0 \pmod {4} \\
\\
100.99! \cong 0 \pmod{4} . 99! \cong 0 \pmod{4}
\end{array}
$$

#
3. The remainder when the positive integers are divided by 5 are 1,2, and 3. Find the remainder when the product of the interger is divided by 5.

    Let the integers be a, b, and c, where:

$$ \begin{array}{c}
a \cong 1 \pmod 5 \\
b \cong 2 \pmod 5 \\
c \cong 3 \pmod 5
\end{array}
$$

$$ \begin{array}{c}
a * b * c \cong 1.2.3 \pmod{5} \\
6 \pmod {5} \\
\text{Mod out, } 1 \pmod {6}
\end{array}
$$
#
4. find the remainder when $317 * 5^{51}$ is divided by 6.

***Solve with Pattern observation***

***4.1 Resolve $5^{51}$ as first,***

$$ 
\begin{array}{c}
5^1 \cong 5 \pmod{6} \\
5^2 \cong 1 \pmod{6} \\
5^3 \cong 5 \pmod{6} \\
\ldots \\
5^{51} \text{since, power of 5 is odd, then } 5^{51} \cong 5 \pmod{6}
\end{array}
$$

***4.2 Resolve $317$ as last,***

$$ 
\begin{array}{c}
317 - 6 * 52 = 5 \\
317 \cong 5 \pmod{6}
\end{array}
$$

***Solve***

$$ \begin{array}{c}
317 * 5^{51} \cong 5 \pmod{6} * 5 \pmod{6} \\
317 * 5^{51} \cong -1 \pmod{6} * -1 \pmod{6} \\
317 * 5^{51} \cong 1 \pmod{6} \\
\text{Since, remainder(R) is 1}
\end{array}
$$

#

5. find the remainder when $24^{50} - 15^{50}$ is divided by 13.

***5.1 Resolve $24^{50}$ as first,***


$$ \begin{array}{c}
24 ÷ 13 = 1 \text{ R 11} \\
24 \cong 11 \pmod{13} \\
\text{mod out, } 24 \cong 11 \cong -2 \pmod{13}
\end{array}
$$

***5.2 Resolve $15^{50}$ as first,***

$$ \begin{array}{c}
15 ÷ 13 = 1 \text{ R 2} \\
15 \cong 2 \pmod{13}
\end{array}
$$

**5.3 Approach 1: Apply Fermat's Little Theorem**

***Fermat's Little Theorem states: If p is a prime number and a is any integer not divisible by p, then $a^{p−1} \cong 1 \pmod{p}$.***

In this case: $13$ is prime, and neither 11 nor 2 is divisible by 13, so:

$$ \begin{array}{c}
    11^{13-1} = 11^{12} \cong 1 \pmod{13} \\
    2^{13-1} = 2^{12} \cong 1 \pmod{13}
    \end{array}
$$

***Then,***

$$ \begin{array}{c}
    24^{50} - 15^{50} \cong (11)^{50} - (2)^{50} \pmod{13} \\
    24^{50} - 15^{50} \cong (11^{12})^4 . 11^{2} - (2^{12})^{4} . 2^{2} \pmod{13} \\
    24^{50} - 15^{50} \cong (1)^4 . 11^{2} - (1)^{4} . 2^{2} \pmod{13} \\
    24^{50} - 15^{50} \cong 121 - 4 \pmod{13} \\
    24^{50} - 15^{50} \cong 117 \pmod{13} \\
    \text{Mod out, } 117 \cong 13 \cong 0 \pmod{13} \\
    \text{Remainder is 0}
    \end{array}
$$


**5.4 Approach 2: Substitute with set of n**

***Then,***

$$ \begin{array}{c}
    24^{50} - 15^{50} \cong (11)^{50} - (2)^{50} \pmod{13} \\
    24^{50} - 15^{50} \cong (-2)^{50} - 2^{50} \pmod{13} \\

    24^{50} - 15^{50} \cong +2^{50} - 2^{50} \pmod{13} \\
    \text{Remainder is 0}
    \end{array}
$$
#