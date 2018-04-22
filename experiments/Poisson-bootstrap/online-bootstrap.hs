{--
Module for doing one-pass estimates of distributional parameters
Currently not-so-optimized.
--}
-- module OnlineEstimators
--     (moments,
--     meanVar,
--     meanVarWeighted
--     ) where

est :: (Floating a) => [a] -> (a,a,a)
est [] = (0,0,0)
est xs = foldr step (0,0,0) xs

step :: (Floating a) => a -> (a,a,a) -> (a,a,a)
step x (n,m,m2) = (nNew,mNew,m2New)
                  where delta = x-m
                        nNew = n+1
                        mNew = m+delta/nNew
                        m2New = m2+delta*(x-mNew)
-- step x (n,m,m2) = (n+1,m+delta/(n+1),m2+delta*(x-(m+delta/(n+1))))
                  -- where delta = x-m

onlineMeanVar :: (Floating a) => [a] -> (a,a)
onlineMeanVar [] = (0,0)
onlineMeanVar xs = (m,m2/n)
                   where (n,m,m2) = est xs

-- might have an efficiency tradeoff?
-- onlineMeanVar xs = onlineMeanVarWeighted xs (repeat 1)

estWeighted :: (Floating a) => [a] -> [a] -> (a,a,a)
estWeighted [] []  = (0,0,0)
estWeighted xs ws = foldr stepWeighted (0,0,0) $ zip xs ws

stepWeighted :: (Floating a) => (a,a) -> (a,a,a) -> (a,a,a)
stepWeighted (x,w) (n,m,m2) = (nNew,mNew,m2New)
                      where delta = x-m
                            nNew = n + w
                            r = delta*w/nNew
                            mNew = m + r
                            m2New = n*delta*r

onlineMeanVarWeighted :: (Floating a) => [a] -> [a] -> (a,a)
onlineMeanVarWeighted xs ws = (m,m2/n)
                              where (n,m,m2) = estWeighted xs ws

covStep :: (Floating a) => (a,a) -> (a,a,a,a) -> (a,a,a,a)
covStep (x1,x2) (n,m1,m2,m12) = (nNew,m1New,m2New,m12New)
                                where nNew = n+1
                                      delta1 = (x1-m1)/nNew
                                      delta2 = (x2-m2)/nNew
                                      m1New = m1 + delta1
                                      m2New = m2 + delta2
                                      m12New = m12 + n*delta1*delta2 - m12/nNew

onlineCovariance :: (Floating a) => [a] -> [a] -> (a,a,a,a)
onlineCovariance xs ys = foldr covStep (0,0,0,0) (zip xs ys)
