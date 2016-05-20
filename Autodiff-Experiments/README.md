# Autodiff-Experiments

Trying some things out with the `ad` module in Haskell. Currently just a simple example.

`test.hs`: Simple function for optimizing a one-dimensional function. Try loading in to ghci and then running the following the following:
```haskell
pi == (last $ take 60 (opt1D (\x -> cos x) 1))
```
