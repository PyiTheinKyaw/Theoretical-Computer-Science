# Matrix Inverse


In linear algebra, the inverse of a matrix can be used to solve a system of linear equations. Here's how it works:

**System of Linear Equations:**

Consider a system of linear equations represented by the matrix equation:
$$
Ax = b
$$
where:

- A is the coefficient matrix, a square matrix with dimensions (m x n).
- x is the variable vector, a column vector with dimensions (n x 1).
- b is the constant vector, a column vector with dimensions (m x 1).

**Matrix Inverse:**

The inverse of matrix A, denoted by $A^{-1}$, is another matrix with the same dimensions as A, such that:

$$
A^{-1} * A = I
$$
where I is the identity matrix, a diagonal matrix with 1s on the diagonal and 0s else where.


## Matrix Invesion Method

Let's say there are given equations:

$x + y + z = 6$

$3x + 3y + 4z = 20$

$2x + y + 3z = 13$

System of equation can be written in matrix form as,

$$
\left[
\begin{matrix}
1 & 1 &  1 \\
3 & 3 &  4 \\
2 & 1 &  3
\end{matrix}
\right]
\left[
\begin{matrix}
x\\
y\\
z
\end{matrix}
\right] =

\left[
\begin{matrix}
6 \\
20 \\
13
\end{matrix}
\right]
$$

This looks like matrix equation $A.x = B$, where
- A is the coefficient matrix, a square matrix with dimensions (m x n).
- x is the variable vector, a column vector with dimensions (n x 1).
- b is the constant vector, a column vector with dimensions (m x 1). 

In order to find $x$, we need

$$
A.x = B \\
x = A^{-1}B \\
$$

In order to find $A^{-1}$, 

$$ 
A^{-1} = \frac{1}{|A|}A^T
$$

***Step 1: Find determinant $|A|$,***

$$
|A| = 
1 
\left[ 
\begin{matrix} 
3 & 4 \\ 
1 & 3
\end{matrix}
\right] -
1
\left[ 
\begin{matrix} 
3 & 4 \\ 
2 & 3
\end{matrix}
\right] +
1
\left[ 
\begin{matrix} 
3 & 3 \\ 
2 & 1
\end{matrix}
\right] \\
= 1(9-4) -1(9-8) +1(3-6) \\
= 5 - 1 - 3 \\
|A| = 1 \neq 0
$$

**The value of |A| is not equal to 0, which means inverse of A is exists.**

***Step 2: Find the transport(cofactor) of the matrix $A^T$,***

The matrix of A,


$$
A =
\left[
\begin{matrix}
1 & 1 &  1 \\
3 & 3 &  4 \\
2 & 1 &  3
\end{matrix}
\right]
$$



#

## Cramer's Rule

Cramer's Rule provides an explicit formula for solving a system of linear equations when there are as many equations as unknowns and the system has a unique solution. It uses determinants to calculate the solution for each variable individually.

Cramer's Rule expresses the solution for each variable ($x_i$) as the quotient of two determinants:
$$
x_i = det(A_i) / det(A)
$$

where:

- $A_i$ is a new matrix formed by replacing the i-th column of A with the constant vector b.
- $det(A)$ is the determinant of the coefficient matrix A.
- $det(A_i)$ is the determinant of the matrix $A_i$.

**Example:**

We have following linear equation those are -

$2x + y -z = 1$

$3x +2y +2z = 13$

$4x -2y +3z = 9$

$$
a_1x + b_1y + c_1z = d_1 \\
a_2x + b_2y + c_2z = d_2 \\
a_3x + b_3y + c_3z = d_3 
$$

In above equation, letter $a,b,c$ is the coefficent. Then

$x = \frac{Dx}{D}$, $y = \frac{Dy}{D}$, $z = \frac{Dz}{D}$

***Step1 : Calculate the Determinant (D)***

So, Following are calculate 3x3 determinant.

$$
D =
\left[
\begin{matrix}
a_1 & b_1 & c_1 \\
a_2 & b_2 & c_2 \\
a_3 & b_3 & c_3
\end{matrix}
\right]
$$

$$
D =
a_1
\left[
\begin{matrix}
a_2 & c_2 \\
a_3 & c_3
\end{matrix}
\right] -
b_1
\left[
\begin{matrix}
a_1 & c_1 \\
a_3 & c_3
\end{matrix}
\right] +
c_1
\left[
\begin{matrix}
a_2 & b_2 \\
a_3 & b_3
\end{matrix}
\right] 
$$

Substituate the value,

$$
D = 
2
\left[
\begin{matrix}
2 & 2 \\
-2 & 3
\end{matrix}
\right] -
1
\left[
\begin{matrix}
3 & 2 \\
4 & 3
\end{matrix}
\right] 
-1
\left[
\begin{matrix}
3 & 2 \\
4 & -2
\end{matrix}
\right] 
$$

$$
D = 2 \left[(2)(3) - (2)(-2) \right]
-1 \left[(3)(3) - (4)(2)\right]
-1 \left[(3)(-2) - (4)(2)\right] \\
= 2(10) -1(1) -1(-14) \\
= 20 -1 +14 \\
D = 33
$$

***Step2 : Calculate the Dx***
$$
Dx = 
\left[
\begin{matrix}
d_1  & b_1 & c_1 \\
d_2  & b_2 & c_2 \\
d_3  & b_3 & c_3 \\
\end{matrix}
\right]
= 
\left[
\begin{matrix}
1  & 1 & -1 \\
13  & 2 & 2 \\
9  & -2 & 3 \\
\end{matrix}
\right]
$$

$$
Dx =
d_1
\left[
\begin{matrix}
b_2 & c_2 \\
b_3 & c_3
\end{matrix}
\right] -
b_1
\left[
\begin{matrix}
d_2 & c_2 \\
d_3 & c_3
\end{matrix}
\right] +
c_1
\left[
\begin{matrix}
d_2 & b_2 \\
d_3 & b_3
\end{matrix}
\right] 
$$

$$
Dx =
1
\left[
\begin{matrix}
2 & 2 \\
-2 & 3
\end{matrix}
\right] -
1
\left[
\begin{matrix}
13 & 2 \\
9 & 3
\end{matrix}
\right] 
-1
\left[
\begin{matrix}
13 & 2 \\
9 & -2
\end{matrix}
\right] 
$$

$$
Dx = 1(6+4) -1(39-18) -1(-26-18) \\
= 10 -21 +44 \\
Dx = 33 \\
$$

***Step3 : Calculate the Dy***

$$
Dy =
\left[
\begin{matrix}
a_1  & d_1 & c_1 \\
a_2  & d_2 & c_2 \\
a_3  & d_3 & c_3 \\
\end{matrix}
\right]
= 
\left[
\begin{matrix}
2  & 1 & -1 \\
3  & 13 & 2 \\
4  & 9 & 3 \\
\end{matrix}
\right]
$$

$$
Dy =
2
\left[
\begin{matrix}
13 & 2 \\
9 & 3
\end{matrix}
\right] -
1
\left[
\begin{matrix}
3 & 2 \\
4 & 3
\end{matrix}
\right] 
-1
\left[
\begin{matrix}
3 & 13 \\
4 & 9
\end{matrix}
\right] 
$$
$$
Dy = 2(39-18) -1(9-8) -1(27-52) \\
= 2(21) -1 +25 \\
= 41 +25 \\
Dy = 66 \\
$$

***Step4 : Calculate the Dz***


$$
Dz =
\left[
\begin{matrix}
a_1  & b_1 & d_1 \\
a_2  & b_2 & d_2 \\
a_3  & b_3 & d_3 \\
\end{matrix}
\right]
= 
\left[
\begin{matrix}
2  & 1 & 1 \\
3  & 2 & 13 \\
4  & -2 & 9 \\
\end{matrix}
\right]
$$

$$
Dz =
2
\left[
\begin{matrix}
2 & 13 \\
-2 & 9
\end{matrix}
\right] -
1
\left[
\begin{matrix}
3 & 13 \\
4 & 9
\end{matrix}
\right] +
1
\left[
\begin{matrix}
3 & 2 \\
4 & -2
\end{matrix}
\right] 
$$
$$
Dz = 2(18+26) -1(17-52) +1(-6-8) \\
= 2(44) +25 -14 \\
= 88 + 11 \\
Dz = 99 \\
$$

**Step 5, Calculate $x = \frac{Dx}{D}$, $y = \frac{Dy}{D}$, $z = \frac{Dz}{D}$**


$x = \frac{Dx}{D} = \frac{33}{33} = 1$, $y = \frac{Dy}{D} = \frac{66}{33} = 2$, $z = \frac{Dz}{D} = \frac{99}{33} = 3$

So, soultion is (1,2,3).
#
