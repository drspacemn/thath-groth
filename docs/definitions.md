# Definition of terms

### `pairing-friendly`

Elliptic curves with a large prime-order subgroup `r` and a small embedding degree `k`.
A family of pairing-friendly elliptic curves (PF-EC) must satisfy the following conditions:

1) `r` is large enough so that it is hard to solve DLP by Pollard’s rho method
2) `k` is big enough so that subexponential methods for computing DLP in Fqk is
equally hard as in E(Fq)
3) `k` is relatively small enough so that the arithmetic (pairing) in Fqk is easily computable.

### `R1CS`

R1CS ( rank-1 constraint system ) is a sequence of vectors `(a, b, c)` such that any solution vector `s` must satisfy the equation s . a * s . b - s . c = 0 ( . = dot product & * = scalar multiplication ).

IE R1CS can be thought of as a set of 3 matricies `A`, `B`, and `C` that provide constraints on valid solutions or witnesses.

### `Quadratic Arithmetic Program`

QAPs are a system polynomial equations for which valid solutions result in a single polynomial equality.

Using QAPs enables ZK-snarks to be succinct. This is due to the large amount of matrix multiplication needed to evaluate a R1CS solution vector.

### `Polynomial Commitment Scheme`
