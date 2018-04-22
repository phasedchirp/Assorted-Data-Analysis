from __future__ import division
import numpy as np
from numpy.random import rand, poisson
from sys import argv


# based on:
# Hanley & MacGibbon (2006) (http://www.ncbi.nlm.nih.gov/pubmed/16730851)
# and
# http://www.unofficialgoogledatascience.com/2015/08/an-introduction-to-poisson-bootstrap_26.html
# see also: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Weighted_incremental_algorithm

def increment(x,reps,ns,ms):
    counts = poisson(1,reps)
    temp = ns + counts
    deltas = x - ms
    Rs = [d*c / t if n > 0 else 0 for n,c,t,d in zip(ns,counts,temp,deltas)]
    return (Rs,deltas,temp)


def onlineMeanVarBoot(xs,reps):
    ns = np.zeros(reps,dtype=np.int)
    ms = np.zeros(reps)
    M2s = np.zeros(reps)

    # for x in xs:
    while xs:
        Rs,deltas,temp = increment(x,reps,ns,ms)
        ms += Rs
        M2s += ns * deltas * Rs
        ns = temp

    if np.min(ns) < 2:
        return np.nan
    else:
        return M2s / ns




if __name__== "__main__":
    test = rand(500)
    testBoot = onlineMeanVarBoot(test,4000)
    print "numpy est: %s, boot est: %s" %(np.var(test),np.mean(testBoot))
