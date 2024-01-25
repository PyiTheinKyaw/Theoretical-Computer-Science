# Predictability

Predictability in randomness, while seemingly contradictory, is a crucial concept in understanding the nature of pseudo-randomness and its limitations. 


***Short periods:*** Some algorithms generate sequences that repeat after a relatively small number of iterations. Analyzing the sequence can reveal the period and predict future values.

***Statistical biases:*** Certain algorithms might produce numbers favoring specific ranges or patterns, deviating from truly random behavior and giving potential insight into the sequence.

***Algorithmic vulnerabilities:*** If the algorithm or its implementation has weaknesses, attackers might be able to exploit them to predict future numbers or reconstruct the entire sequence.

Question is what makes a sequene is predictable?

- ***Repetition (Fixed Point)***
    - $15,22,48,30,10,10,10,\dots$
    - This happended in M-Square randomness.

- ***Preiodicity***
    - $24,57,24,57,\ldots$

- ***Monotonicity***
    - $63,96,21,44,93,64,9,8,6,3,0,\ldots$
    - Monotonicity is easy to avoid by using mod n.
    