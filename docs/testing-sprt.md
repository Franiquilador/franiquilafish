fastchess testing:
LOS 92% - likelyhood of superiority. there's a 92% chance the winning engine is stronger. 
It is less rigorous than SPRT

SPRT: (Sequential Probability Ratio Test)
elo0 and elo1 are the two theories being tested:
elo0=0 - engine1 is not an improvement over the other (look at the Elo: ... with the margin of error) - H0 hypothesis
elo1=5 - engine1 is better by 5 Elo - H1 hypothesis

alpha and beta are how often we're okay with being wrong:
alpha=0.05 - you accept a 5% chance of being told "Engine1 is better" when they're actually equal (false alarm)
beta=0.05 - you accept a 5% chance of being told "they are equal" when one actually IS better (missed result)
Both at 0.05 is the standard used by stockfish testing

Elo: 1.39 +/- 0.96
Sabemate is estimated to be 1.39 Elo stronger than Engine2. 
The +/- 0.96 is the margin of error (95% confidence interval because alpha=0.05), meaning the true difference is likely somewhere between +0.43 and +2.35 Elo. 
Since the entire range is above 0, this is actually starting to look like a real advantage.

LLR - Log Likelihood Latio
the percentage after LLR 41.8% - how far it is toward a conclusion (0% = just started, 100% = done)
lbound -2.94 - if llr hits this → H0 accepted (engines1 is not an improvement, look at the Elo: ... with the margin of error)
ubound 2.94 - if llr hits this → H1 accepted (one is better)

Don't set concurrency higher than half of the number of cpu cores/threads so that the engines dont starve each other of cpu time, making the results inacurate. For my vitus it can be maximum 8