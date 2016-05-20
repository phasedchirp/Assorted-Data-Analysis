{-# LANGUAGE RankNTypes #-}

import Numeric.AD
import Numeric.AD.Mode.Forward


opt1D :: (Num a, Fractional a) => (forall s. AD s (Forward a) -> AD s (Forward a)) -> a -> [a]
opt1D cost start = iterate step start
                  where step x = x - 0.75*((diff cost) x)
