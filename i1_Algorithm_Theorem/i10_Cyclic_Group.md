# Cyclic Group

A group $G$ denoted by {$G, \cdot$} is said to be a cyclic group, if it contains at-least one generator elements.

For example:

Prove that ($G,*$) is a cyclic group where $G=\{1, \Omega, \Omega^2\}$.

As first we have to draw composition table for virtual purposes.

|*|$1$|$\Omega$|$\Omega^2$|
|---|---|---|---|
|1|1|$\Omega$|$\Omega^2$|
|$\Omega$|$\Omega$|$\Omega^2$|1|
|$\Omega^2$|$\Omega^2$|$1$|$\Omega$|

***Generator elements*** means we can create other elements in group by using that generator elements. In this case, in set $G$, there are three elements exists. We need to prove that at least one (or) more element(s) is generator element.

***Evluate the elements.***

For element $1$,

1 = 1

$1^2 = 1$

$1^{9999} = 1$

Obviouslly, $1$ isn't generator element.

For element $\Omega$,

$\Omega^1 = \Omega$

$\Omega^2 = \Omega^2$

$\Omega^3 = 1$

$\Omega^4 = \Omega$

So, $\Omega$ is generator element because we can use $\Omega$ to generate other elements in set.

For element $\Omega^2$,

$\Omega^2 = \Omega^2$

$(\Omega^2)^2 = \Omega$

$(\Omega^2)^3 = 1$

$(\Omega^2)^4 = \Omega^2$

So, $\Omega^2$ is also generator element because we can use $\Omega^2$ to generate other elements in set.

Ans: $\Omega, \Omega^2$ is generator element of $(G,*)$ and $(G,*)$ is cyclic group.

#

Question 2: When does group $G$ with operation '*', is said to be a cyclic group?

မေးခွန်းက Group '$G$' က under operation '*' မှာဆိုရင် Cyclic group ဖြစ်လား?

We have to under stand what is properties of group. Properties is '**CAIN**'. Let assume there are '$x$' elements in Group $G$, set will look like this:

$G =$ {$\ldots,-x^3,-x^2,-x,1,x,x^2, x^3,\ldots$}

$x$ is element, $-x$ is inverse element. $1$ is identity element for "*" operation. Also $x$ is generator element.

Ans: If $G(x)$ for some $x$, then we call $G$ a cyclic group.





