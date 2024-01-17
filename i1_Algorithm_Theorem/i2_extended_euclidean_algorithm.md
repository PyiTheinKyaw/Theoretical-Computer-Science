# Extended Eculidean algorithm (EEA)

The **Extended Euclidean Algorithm (EEA)** is a mathematical algorithm that extends the standard Euclidean Algorithm for finding the greatest common divisor (GCD) of two integers. This EEA have been used in "Bezout Identity" to find a,b and also used to find GCD and MI.

## Finding Multiplicative inverse using EEA (Approach 1)

The table scheme of EEA as described as follow table header:

|$q$|$a$|$b$|$r$|$T_{1}$|$T_{2}$|$T$|
|---|---|---|---|---|---|---|

In EEA, there are new things which are $T_{1}$, $T_{2}$ and $T$. For the first iteration the value of $T_{1}$ and $T_{2}$ are 0 and 1 respectively. 

$$
T_{1} = 0, T_{2} = 1
$$

Then, compute the value of $T$, by using $T_{1}$ and $T_{2}$. The formula to calculate the $T$ is:

$$
T = T_{1} - T_{2} * Q
$$

After completing the iteration you will be encounter the situation that you cannot move further. You cannot proceed the algorithm further. In this case, whaterver value in $T_{1}$ is multiplicitive inverse (MI).

#

**Example 1:**

What is the multiplicative inverse (MI) of 3 mod 5.

let $a = 5, b = 3$, Since a is grather than b.

|$q$|$a$|$b$|$r$|$T_{1}$|$T_{2}$|$T$|
|---|---|---|---|---|---|---|
|1|5|3|2|0|1|-1|
|1|3|2|1|1|-1|2|
|2|2|1|0|-1|2|-5|
|x|1|0|x|2|-5|x|

Now, you are stopping the algorithm, whatever the value in $T_{1}$, it's the MI and $a$ is $GCD$. In this case,

$$
T_{1} = 2, a = 1 
$$

Which means $GCD(5,3) = 1$ and mulitiplicative inverse (MI) of $3 \pmod{5}$ is 2.

#

**Example 2:**

***What is the multiplicative inverse (MI) of 11 mod 13.***

let $a = 13, b = 11$, Since a is grather than b.

|$q$|$a$|$b$|$r$|$T_{1}$|$T_{2}$|$T$|
|---|---|---|---|---|---|---|
|1|13|11|2|0|1|-1|
|5|11|2|1|1|-1|6|
|2|2|1|0|-1|6|-13|
|x|1|0|x|6|-13|x|

In this case,

$$
T_{1} = 6, a = 1 
$$

Which means $GCD(13,11) = 1$ and mulitiplicative inverse (MI) of $11 \pmod{13}$ is 6.
#










