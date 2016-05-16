est :: (Floating a) => [a] -> (a,a,a)
est [] = (0,0,0)
est xs = foldl step (0,0,0) xs

step :: (Floating a) => a -> (a,a,a) -> (a,a,a)
step x (n,m,m2) = (n+1,m+delta/(n+1),m2+delta*(x-(m+delta/(n+1))))
                  where delta = x-m

onlineMeanVar :: (Floating a) => [a] -> (a,a)
onlineMeanVar [] = (0,0)
onlineMeanVar xs = (m,m2/n)
                   where (n,m,m2) = est xs
