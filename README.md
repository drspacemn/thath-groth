# Thath Groth

Learning/Teaching repository for Groth16 proof systems.

## Overview

Protocol relies on [pairing-friendly](./docs/definitions.md#pairing-friendly) elliptic curves:

- BN254
- BLS12-381
- BLS12-377

Proof size consists of 3 elliptic curve elements and is among the fastest to verify, but
it requires a trusted setup per program.

### Protocol

1. Setup Phase
1. Key Generation
1. [R1CS](./docs/definitions.md#r1cs)/[QAP](./docs/definitions.md#quadratic-arithmetic-program)
1. Proof Generation/[PCS](./docs/definitions.md#polynomial-commitment-scheme)
1. Verification

## References

- Intro to QAPs & SNARKs : https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649
- https://eprint.iacr.org/2016/260.pdf?ref=blog.lambdaclass.com
- https://blog.lambdaclass.com/groth16/
- Implementation of the above article : https://github.com/lambdaclass/lambdaworks/tree/main/provers/groth16
- https://www.rareskills.io/post/groth16
- https://www.zeroknowledgeblog.com/index.php/groth16
- https://medium.com/ppio/how-to-generate-a-groth16-proof-for-forgery-9f857b0dcafd
- https://arxiv.org/pdf/2212.01855.pdf
