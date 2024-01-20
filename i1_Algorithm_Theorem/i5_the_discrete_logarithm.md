# The Discrete Logarithm Problem (DLP)

### Understanding the Discrete Logarithm Problem.

The Discrete Logarithm Problem (DLP) is a foundational problem in computational cryptography and has numerous applications in secure communication and data protection.

**Defination**

Given a finite cyclic group G with generator g and an element h within the group, find the integer x such that 

$$
g^x = h
$$

DLP took the advantages of "Primitive Root". Which means '$\alpha$' is said to be a primitive root of prime number '$p$', if $a' \pmod{p}, a^2 \pmod{p}, a^3 \pmod{p}, \ldots, a^{p-1} \pmod{p}$ are distinct. 

For Example:

Let's say, we have cycle 17 clock, which is 17 is prime number. Primitive root of $p=17$ is "5".

So, $5^x \pmod{17} \cong$  distinct value (or) equally distributed.

In this case, we call $5$ as ***"Primitive root"*** or ***"Generator"***

***Why DLP?***

$$
5^x \pmod{17} \cong a
$$

Above congruent is easy to calculate $a$ to substitute by $x$. But in reverse situation like:

$$
5^x \pmod{17} \cong 12
$$

Now, finding the value of $x$ is tough. Because x can be Set $x =$ {$9, 25, 41, 57, 73$}. That's **why easy in one direction, but hard to reverse direction**. It's also called one way function.

**Example:**

Let's solve: $g^x \pmod{p}$

$g$ is the primitive root of $p$ and $x$ can be any numbers. 

let say, $2^x \pmod{7} \cong 4$, Question states that what the value of $x$ which is power of $2$ is congurent to $4$.

So,x can be every value within the set $x=$ {$2,5, \dots$}.

For smaller value of 'p' is may be easy to find 'X'. But if 'p' is very large, then finding 'X' is hard. The strength of one-way function is depending on how much time it takes to break it.
#

***Example:***

***Solve $\log_2 9 \pmod{11}$.***

***Equation to solve:***
$$
\log_g X \cong n \pmod{p} 
$$

***Given,***

let $g=2, p=11, X = 9$

$X \cong g^n \pmod{p}$

$9 \cong 2^n \pmod{11}$

Try 'n' = $1,2,3,\ldots$

Finally, 6 works out. $2^6$ is solved.

$9 \cong 2^6 \pmod{11}$.

Answer is $6$.