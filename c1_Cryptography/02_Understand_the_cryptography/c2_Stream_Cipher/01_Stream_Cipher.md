# Stream Cipher

Under Symmetric Cyptography, there are two categories, they are:
1) Stream Cipher  
2) Block Cipher

***Defination in this chapter:***

***Unconditional Security:***
A cryptosystem is unconditionally or information-theoretically secure if it cannot be broken even with infinite computational resources.

***Computational Security***
A cryptosystem is computationally secure if the best known algorithm for breaking it requires at least $t$ operations.

#
### Stream Ciphers
Stream ciphers ***encrypt bits individually***. This is achieved by **adding a bit from a key stream to a plaintext bit.**  There are 
- **synchronous stream ciphers** where the key stream *depends only on the key*, and 
- **asynchronous** ones where the key stream also depends on the ciphertext. An example of an asynchronous stream cipher is ***the cipher feedback (CFB)*** mode.


### Block Ciphers
Block ciphers **encrypt an entire block of plaintext bits at a time with the same key**. This means that the encryption of any plaintext bit in a given block depends on every other plaintext bit in the same block. In practice, the vast majority of block ciphers either have a 
- **block length of 128 bits (16 bytes)** such as the ***advanced encryption standard (AES)***, or 
- **block length of 64 bits (8 bytes)** such as the ***data encryption standard (DES)*** or ***triple DES (3DES)*** algorithm.

### Facts of stream ciphers vs. block ciphers

1. In practice, in particular for encrypting computer communication on the Internet, ***block ciphers are used more often than stream ciphers***.

2. Because stream ciphers **tend to be small and fast**, they are particularly relevant for applications with *little computational resources*, e.g., for cell phones or other small embedded devices. A prominent example for ***a stream cipher is the A5/1 cipher***, which is part of the GSM mobile phone standard and is used for voice encryption. However, *stream ciphers are sometimes also used for encrypting Internet traffic, especially the stream cipher RC4*.

3. Traditionally, **it was assumed that stream ciphers tended to encrypt more efficiently than block ciphers**. 

- Efficient for software-optimized stream ciphers means
that they need fewer processor instructions (or processor cycles) to encrypt one bit of plaintext. 
- For hardware-optimized stream ciphers, efficient means they need fewer gates (or smaller chip area) than a block cipher for encrypting at the same data rate. 

    However, modern block ciphers such as **AES are also very efficient in software**. Moreover, for **hardware, there are also highly efficient block ciphers,such as PRESENT**, which are as efficient as very compact stream ciphers.

## Encryption and Decryption with Stream Ciphers

stream ciphers encrypt plaintext bits individually by encrypting each bit $x_i$ by adding a secret key stream bit $s_i \pmod{2}$.

***Stream Cipher Encryption and Decryption:***

The plaintext, the ciphertext and the key stream consist of individual bits,

$\text{i.e., } x_i,y_i, s_i \in \{0,1\}$.
$$
y_i = e_{s_i} (x_i) ≡ x_i+s_i \pmod{2} \text{ Encryption} \\
x_i = d_{s_i} (y_i) ≡ y_i+s_i \pmod{2} \text{ Decryption}
$$

In above equation, both encryption and decryption formula is same. there are three points about the stream cipher encryption and decryption function which we should clarify:

- Encryption and decryption are the same functions!
- Why can we use a simple modulo 2 addition as encryption?
- What is the nature of the key stream bits $s_i$?

### Encryption and decryption are the same functions!

Because, they are opertion on Cycle 2. Which means under mod 2, both addition and subtraction operation are same. Following are the proof of why they are same,

$y_i, x_i, s_i \in$ {$0,1$}

$d_{s_i} (y_i) \cong y_i + s_i \pmod{2}$

$\cong (x_i + s_i) + s_i \pmod{2}$

$\cong x_i + 2s_i \pmod{2}$

$\cong x_i + 0 \pmod{2}$

$\cong x_i \pmod{2}$

### Why can we use a simple modulo 2 addition as encryption?

As you can see, above encryption, decryption are done under modulo 2 addition.

If we do arithmetic modulo 2, the only possible values are 0 and 1 (because if you divide by 2, the only possible remainders are 0 and 1). Thus, we can treat arithmetic modulo 2 as Boolean functions such as AND gates, OR gates, NAND gates, etc. Let’s look at the truth table for modulo 2 addition:

This should look familiar to most readers: It is the truth table of the exclusive-OR, also called XOR, gate. This is in important fact: **Modulo 2 addition is equivalent to the XOR operation**.

***Why is the XOR operation so useful, as opposed to, for instance, the AND operation?***

|$x_i$|$s_i$|$y_i$|
|---|---|---|
|0|0|0|
0|1|1|
1|0|1|
1|1|0|

Depending on the key bit, the ciphertext $y_i$ is either a zero ($s_i =0$) or one ($s_i =1$). If the key bit si behaves perfectly randomly, i.e., it is unpredictable and has exactly a 50% chance to have the value 0 or 1, then both possible ciphertexts also occur with a 50% likelihood. Likewise, if we encrypt the plaintext bit $x_i = 1$, we are on line 3 or 4 of the truth table. Again, depending on the value of the key stream bit $s_i$, there is a 50% chance that the ciphertext is either a 1 or a 0.

This the reason why XOR gate is used. We just observed that the **XOR function is perfectly balanced**, i.e., by observing an output value, there is exactly a 50% chance for any value of the input bits. This distinguishes the XOR gate from other Boolean functions such as the OR, AND or NAND gate. Moreover, AND and NAND gates are not **invertible**.

### What Exactly Is the Nature of the Key Stream?

It turns out that the generation of the values $s_i$, which are called ***the key stream***, is the ***central issue for the security of stream ciphers***. In fact, the security of a stream cipher completely depends on the key stream.
Generating the key stream is
pretty much what stream ciphers are about.

However, we can already guess that a central requirement for the key stream bits should be that they appear like a random sequence to an attacker.Otherwise, an attacker Oscar could guess the bits and do the decryption by himself. Hence, we first need to learn more about random numbers.

## Random Number Generator (RNG)

We distinguish between 3 types of RNGs.

***True Random Number Generator (TRNG)***

TRNG are characterized by the fact that their output cannot be reproduced. TRNGs are based on physical processes. Examples: include ***coin flipping, rolling of dice, semiconductor noise,clock jitter in digital circuits and radioactive decay.*** 
    
In cryptography, TRNGs are often needed for generating session keys, which are then distributed between Alice and Bob, and for other purposes.

***(General) Pseudorandom Number Generators (PRNG)***

Pseudorandom number generators (PRNGs) generate sequences which are computed from an initial seed value. Often they are computed recursively as following:

$$
s_0 = \text{seed} \\
s_i+1 = f(s_i), i = 0,1, \ldots
$$

The idea is "**Seed**" is TRNG and following $s_i$ are recursively generated base on $s_0$. Note that PRNGs are not random in a true sense because they can be computed and are thus completely deterministic. A popular example ***is the linear congruential generator***:

$$
s_0 = \text{seed} \\
s_i+1 \cong a_{s_i} + b \pmod{m}, i = 0,1, \dots
$$

***Example (Which is really used in rand() function used in ANSI C)***

$$
s_0 = 12345 \\
s_{i + 1} \cong 1103515245s_i + 12345 \pmod{2^{31}}, i = 0,1, \dots
$$

A common requirement of PRNGs is that they possess good statistical properties, meaning their output approximates a sequence of true random numbers.

***Cryptographically Secure Pseudorandom Number Generators (CSPRNG)***

Cryptographically secure **pseudorandom number generators (CSPRNGs)** are a **special type of PRNG** which possess the following additional property: ***A CSPRNG is PRNG which is unpredictable***.

Note that the need for unpredictability of CSPRNGs is unique to cryptography.

In virtually all other situations where pseudorandom numbers are needed in computer science or engineering, unpredictability is not needed. As a consequence, the distinction between PRNG and CSPRN and their relevance for stream ciphers is often not clear to non-cryptographers. Almost all PRNG that were designed without the clear purpose of being stream ciphers are not CSPRNGs.