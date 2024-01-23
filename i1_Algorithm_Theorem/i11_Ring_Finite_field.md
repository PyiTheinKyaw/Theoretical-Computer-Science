# Ring, Field and Finite Field

## Ring

A ring $R$ denoted by {$R, +, *$} is a set of elements with two binary operation, called addition and multiplication, such that for all $a,b,c \in R$ the following axioms are obeyed:

- In order to be a ring, in must be ***Group,Abelian Group***,
- ***Closure*** under multiplication:
    - if $a,b \in R \text{ then } a.b \in R$
- ***Associativity*** of multiplication:
    - if $a(bc) = (ab)c \text{ for all } a,b,c \in R$ 
- ***Distributivity*** of multiplication over addition:
    - $a(b+c) = ab+ac \text{ for all } a,b,c \in R$
    - $(a+b)c = ac+bc \text{ for all } a,b,c \in R$

## Commutative Ring

A ring is said to be commutative, if it satisfied the following additional condition:

- ***Commutative*** of multiplication:
    - $ab = ba \text{ for all } a,b \in R$

Commutative ring which is also sastified the properties of **Group**, **Abelian Group** and also **Ring**.

## Integral Domain

An integral domain is a commutative ring that obeys the following axioms:

- ***Multiplicative Identiy***: 
    - There is an element $1 \in R$ such that $a*1 = 1*a = a \text{ for all } a \in R$.
- ***No Zero divisors***:
    - If $a,b \in R$ and $ab = 0$, then either $a = 0$ or $b=0$.

## Fields
A field $F$, sometimes denoted by {$F,+,*$}, is a set of elements with two binary operations, called addition and multiplication, such that for all $a,b,c \in F$ the following axioms are obeyed:

- Must satisfied the ***Integral domain's properties.***
- Multiplicative inverse:
    - For each a in $F$, expect $0$, there is an element $a^{-1}$ in $F$ such that
    
$$
a*a^{-1} = (a^{-1})*a = 1
$$

Familiar example of Fields,
- Rational numbers
- Real numbers
- Complex Numbers

## Finite Fields

A finite field or Galois field is a field that contains a finite number of elements. As with any finite, a finite field is a set on which the operations of multiplication, addition, subtraction and division are defined and satisfy certain basic rules. 

The most common examples of finite fields are given by the integers (mod p) where p is prime number.



