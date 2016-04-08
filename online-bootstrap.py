import numpy as np
from numpy.random import rand, poisson

# algorithm apparently from Knuth, by way of Wikipedia
# https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
def onlineMeanVar(xs):
    n = 0
    m = 0.0
    M2 = 0.0

    for x in xs:
        n += 1
        de

def onlineMeanVarBoot(xs,reps):
    pass

test = rand(500)

np.mean(test)
np.var(test)
