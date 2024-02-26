# Elliptic Curve Pairings

## Reading List

1. https://medium.com/@VitalikButerin/exploring-elliptic-curve-pairings-c73c1864e627
1. https://hackmd.io/@benjaminion/bls12-381
1. https://static1.squarespace.com/static/5fdbb09f31d71c1227082339/t/5ff394720493bd28278889c6/1609798774687/PairingsForBeginners.pdf

## Notes

Pairings allow us to check more complicated equations on elliptic curve points without leaking
crucial information about curve points.

## Notes Dump:

- Bilinear maps
- "point at infinity" (O) which is the equivalent of zero
- always the case thet P + O = P
- "order" P * n = 0
    - order for secp256k1 == 2 ^ 256 (order + 1 * p = P)
    - generator point 
- 