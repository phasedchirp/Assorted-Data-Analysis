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
#def onlineMeanVarBoot(xs,reps):
    #ns = np.zeros(reps,dtype=np.int)
    #ms = np.zeros(reps)
    #M2s = np.zeros(reps)
    
    #for x in xs:
        #counts = poisson(1,reps)
        #ns += counts
        #deltas = counts .* x
        #ms += deltas/ns
        #M2s += deltas .**2
    
    #if stuff:
        #pass
    #else:
        #pass
        

test = rand(500)

np.mean(test)
np.var(test)
