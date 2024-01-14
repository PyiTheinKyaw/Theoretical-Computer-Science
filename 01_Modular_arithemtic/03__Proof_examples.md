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

#

## Multiplication and Exponent
2. Find the remainder when the following are divided by 4:
- $15^{15}$
- $121.122.123$
- 100!

### $15^{15}$
$$
15 ÷ 4 = 3 \text{ R 3}, 15 \cong 3 \pmod{4} \\
15 \cong 3 \cong -1 \pmod{4} \\
(-1)^{15} = -1
$$


### $121.122.123$
$$
121 - 4 * 30 = 1, 121 \cong 1 \pmod{4} \\

121 \cong 1 \pmod{4} \\ 
122 \cong 2 \pmod{4} \\
123 \cong 3 \pmod{4} \\

121.122.123 \cong 1 \pmod{4} . 2 \pmod{4} . 3 \pmod{4} \\
\cong 6 \pmod{4} \\
\text{Mod out, } \cong 2 \pmod{4}
$$

### $100!$
$$
100! = 100.99! \\
100 \cong 0 \pmod {4} \\
\\
100.99! \cong 0 \pmod{4} . 99! \cong 0 \pmod{4}
$$

#
3. The remainder when the positive integers are divided by 5 are 1,2, and 3. Find the remainder when the product of the interger is divided by 5.

    Let the integers be a, b, and c, where:
$$
a \cong 1 \pmod 5 \\
b \cong 2 \pmod 5 \\
c \cong 3 \pmod 5
$$

$$
a * b * c \cong 1.2.3 \pmod{5} \\
6 \pmod {5} \\
\text{Mod out, } 1 \pmod {6}
$$
#
4. find the remainder when $317 * 5^{51}$ is divided by 6.

Approach 1 (Pattern observation): 

***4.1 Resolve $5^{51}$ as first,***
$$
5^1 \cong 5 \pmod{6} \\
5^2 \cong 1 \pmod{6} \\
5^3 \cong 5 \pmod{6} \\
\ldots \\
5^{51} \text{since, power of 5 is odd, then } 5^{51} \cong 5 \pmod{6}
$$

***4.2 Resolve $317$ as last,***
$$
317 - 6 * 52 = 5 \\
317 \cong 5 \pmod{6}
$$

***Solve***
$$
317 * 5^{51} \cong 5 \pmod{6} * 5 \pmod{6} \\
317 * 5^{51} \cong -1 \pmod{6} * -1 \pmod{6} \\
317 * 5^{51} \cong 1 \pmod{6} \\
\text{Since, remainder(R) is 1}
$$