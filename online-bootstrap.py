import numpy as np
from numpy.random import rand, poisson

# algorithm apparently from Knuth, by way of Wikipedia
# https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
# Here giving *biased* estimate for the mean
def onlineMeanVar(xs):
    n = 0
    m = 0.0
    M2 = 0.0

    for x in xs:
        n += 1
        delta = x - m
        m += delta/n
        M2 += delta**2

    if n < 2:
        return (np.nan,np.nan)
    else:
        return (m, M2/n)

# online version based on:
# Hanley & MacGibbon (2006) (http://www.ncbi.nlm.nih.gov/pubmed/16730851)
# and
# http://www.unofficialgoogledatascience.com/2015/08/an-introduction-to-poisson-bootstrap_26.html
# see also: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Weighted_incremental_algorithm
def onlineMeanVarBoot(xs,reps):
    ns = np.zeros(reps,dtype=np.int)
    ms = np.zeros(reps)
    M2s = np.zeros(reps)

    for x in xs:
        counts = poisson(1,reps)
        temp = ns + counts
        deltas = x - ms
        Rs = np.zeros(reps)
        for i in range(len(ms)):
            if ns[i] > 0:
                Rs[i] = deltas[i] * counts[i] / temp[i]
            else:
                Rs[i]  = 0
        ms += Rs
        M2s += ns * deltas * Rs
        ns = temp

    if np.min(ns) < 2:
        return np.nan
    else:
        return M2s / ns


test = rand(500)

np.mean(test)
np.var(test)
