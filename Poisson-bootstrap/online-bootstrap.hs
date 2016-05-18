-- module OnlineEstimators
--     (--moments,
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
