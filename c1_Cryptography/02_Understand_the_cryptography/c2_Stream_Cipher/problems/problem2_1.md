## Problem 2.1

**Definition 2.1.1**

Stream Cipher Encryption and Decryption. The plaintext, the ciphertext and the key stream consist of individual bits,
i.e., $x_i , y_i , s_i \in {0, 1}$

Encryption: $y_i = e_{s_i} (x_i) \cong x_i + s_i \pmod{2}$

Decryption: $x_i = d_{s_i} (y_i) \cong y_i + s_i \pmod{2}$

**Problem**

The stream cipher described in Definition 2.1.1 can easily be generalized to work in alphabets other than the binary one. For manual encryption, an especially useful one is a stream cipher that operates on letters.

1. Develop a scheme which operates with the letters $A, B,. . ., Z$ represented by the numbers $0,1,. . .,25$. What does the key (stream) look like? What are the encryption and decryption functions?

2. Decrypt the following cipher text:
`bsaspp kkuosp` which was encrypted using the key: `rsidpy dkawoa`

3. How was the young man murdered?

#

### Ans 2.1.1

Let says $A,B,\dots,Z$ is number $0,1,2,\ldots,25$.

***1. Encryption, Decryption functions***

Encryption: $y_i = e_{s_i} (x_i) \cong x_i + s_i \pmod{25}$

Decryption: $x_i = d_{s_i} (y_i) \cong y_i - s_i \pmod{26}$

***2. Plain text is `kaspar hauser`***

***3. How was the young man murdered?***

Fatal stab wound

Five days later, on 14 December 1833, Hauser came home with a deep wound in his left breast. By his account, he had been lured to the Ansbach Court Garden, where a stranger stabbed him while giving him a bag.