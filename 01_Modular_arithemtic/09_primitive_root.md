# Primitive root

Determine wheter a number is a primitive root of a given prime number or not. The diffie-hellman key exchange process use this primitive root. 

A number '$\alpha$' is a primitive root modulo $n$ if every number coprime to $n$ is congruent to a power of '$\alpha$' modulo $n$.

Which means: '$\alpha$' is said to be a primitive root of prime number '$p$', if $a' \pmod{p}, a^2 \pmod{p}, a^3 \pmod{p}, \ldots, a^{p-1} \pmod{p}$ are distinct. 

To be a **primitive root**, an integer must generate all distinct remainders that are relatively prime to n. "distinct values" refer to the unique remainders that emerge when raising a candidate primitive root to consecutive powers modulo n. 

- The *number* of distinct values generated must equal the value of ***the Euler Totient Function***, $\varphi(n)$, for a candidate to be a primitive root.

- Distinct values ensure that a primitive root ***can systematically produce all possible remainders that are relatively prime to n,*** making it valuable for various applications like cryptography and number theory.



***Example:***

***Is $2$ a primitive root of prime number $5$?***

Let be $\alpha = 2, p = 5$, then

|$\alpha^{1 \ldots p-1} \pod{n}$|Simplify|$R$|Is uniformly distributed?|
|---|---|---|---|
|$2^{1} \pmod{5}$|$2 \pmod{5}$| 2 | True| 
|$2^{2} \pmod{5}$|$4 \pmod{5}$| 4 | True|
|$2^{3} \pmod{5}$|$8 \pmod{5}$| 3 | True|
|$2^{4} \pmod{5}$|$16 \pmod{5}$| 1 | True|

Number $R = \{1,2,3,4\}$ are uniformly distributed or distinct values and $\varphi(5) = 4$. Answer is **"yes, 2 is a primitive root of prime number 5".**

#
***Is $3$ a primitive root of prime number $7$?***

Let be $\alpha = 3, p = 7$, then

|$\alpha^{1 \ldots p-1} \pod{n}$|Simplify|$R$|Is uniformly distributed?|
|---|---|---|---|
|$3^{1} \pmod{7}$|$3 \pmod{7}$| 3 | True| 
|$3^{2} \pmod{7}$|$9 \pmod{7}$| 2 | True|
|$3^{3} \pmod{7}$|$6 \pmod{7}$| 6 | True|
|$3^{4} \pmod{7}$|$4 \pmod{7}$| 4 | True|
|$3^{5} \pmod{7}$|$12 \pmod{7}$| 5 | True|
|$3^{6} \pmod{7}$|$15 \pmod{7}$| 1 | True|


Number $R = \{1,2,3,4,5,6\}$ are uniformly distributed or distinct values and $\varphi(7) = 6$. 

Answer is **"yes, 3 is a primitive root of prime number 7".**
#

***Is $2$ a primitive root of prime number $7$?***

Let be $\alpha = 2, p = 7$, then

|$\alpha^{1 \ldots p-1} \pod{n}$|Simplify|$R$|Is uniformly distributed?|
|---|---|---|---|
|$2^{1} \pmod{7}$|$2 \pmod{7}$| 2 | True| 
|$2^{2} \pmod{7}$|$4 \pmod{7}$| 4 | True|
|$2^{3} \pmod{7}$|$8 \pmod{7}$| 1 | True|
|$2^{4} \pmod{7}$|$16 \pmod{7}$| 2 | False|
|$2^{5} \pmod{7}$|$4 \pmod{7}$| 4 | False|
|$2^{6} \pmod{7}$|$8 \pmod{7}$| 1 | False|


Number $R = \{1,2,4\}$ are not uniformly distributed or distinct values and $\varphi(7) = 6$. And they aren't same.

Answer is **"No, 2 is not a primitive root of prime number 7".**
#
***Is $2$ a primitive root of prime number $11$?***

Let be $\alpha = 2, p = 11$, then

|$\alpha^{1 \ldots p-1} \pod{n}$|Simplify|$R$|Is uniformly distributed?|
|---|---|---|---|
|$2^{1} \pmod{11}$|$2 \pmod{11}$| 2 | True| 
|$2^{2} \pmod{11}$|$4 \pmod{11}$| 4 | True| 
|$2^{3} \pmod{11}$|$8 \pmod{11}$| 8 | True| 
|$2^{4} \pmod{11}$|$16 \pmod{11}$| 5 | True| 
|$2^{5} \pmod{11}$|$10 \pmod{11}$| 10 | True| 
|$2^{6} \pmod{11}$|$20 \pmod{11}$| 9 | True| 
|$2^{7} \pmod{11}$|$18 \pmod{11}$| 7 | True| 
|$2^{8} \pmod{11}$|$14 \pmod{11}$| 3 | True| 
|$2^{9} \pmod{11}$|$6 \pmod{11}$| 6 | True| 
|$2^{10} \pmod{11}$|$12 \pmod{11}$| 1 | True| 

Number $R = \{1,2,3,\ldots,10\}$ are uniformly distributed or distinct values and $\varphi(11) = 10$.

Answer is **"Yes, 2 is not a primitive root of prime number 11".**