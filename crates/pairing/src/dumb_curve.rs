use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

#[derive(Debug, Default, Clone, PartialEq)]
struct Point {
    x: BigInt,
    y: BigInt,
}

#[derive(Debug, Clone, PartialEq)]
struct DumbCurve {
    a: BigInt,
    b: BigInt,
    p: BigInt, // prime order of the finite field
    n: BigInt, // order of the curve
}

trait ECOps {
    fn point_add(&self, p: &Point, q: &Point) -> Point;
    fn point_double(&self, p: &Point) -> Point;
    fn scalar_multiply(&self, p: &Point, scalar: &BigInt) -> Point;
}

impl ECOps for DumbCurve {
    // adding two points involves calculating the slope of the line
    // passing through two points
    fn point_add(&self, p: &Point, q: &Point) -> Point {
        let zero = BigInt::zero();
        if p.x == zero && p.y == zero {
            return q.clone();
        }
        if q.x == zero && q.y == zero {
            return p.clone();
        }

        // lambda represents the slope of the points
        let lambda = if p == q {
            let first = BigInt::from(3) * p.x.clone().pow(2) + &self.a;
            let second =
                (BigInt::from(2) * p.y.clone()).modpow(&(&BigInt::from(2) * p.x.clone()), &self.b);
            first * second
        } else {
            let first = q.y.clone() - &p.y;
            let second = (q.x.clone() - &p.x).modpow(&(&self.b - BigInt::from(2)), &self.b);
            first * second
        };

        let x3 = lambda.clone().pow(2) - &p.x - &q.x;
        let y3 = lambda.clone() * (&p.x - &x3) - &p.y;

        Point { x: x3, y: y3 }
    }

    fn point_double(&self, p: &Point) -> Point {
        self.point_add(p, p)
    }

    fn scalar_multiply(&self, p: &Point, scalar: &BigInt) -> Point {
        let mut res = Point::default();
        let mut curr = p.clone();

        for _ in num_iter::range_inclusive(BigInt::zero(), scalar.clone()) {
            res = self.point_add(&res, &curr);
            curr = res.clone();
        }

        res
    }
}

pub fn demo() {
    // Parameters for elliptic curve y^2 = x^3 + ax + b
    let curve = DumbCurve {
        a: BigInt::from(1),
        b: BigInt::from(1),
        p: BigInt::from(23),
        n: BigInt::from(24),
    };

    // Base point for the curve
    let g = Point {
        x: BigInt::from(1),
        y: BigInt::from(2),
    };

    let private_key = BigInt::from(3);
    let public_key = curve.scalar_multiply(&g, &private_key);

    println!("private key = {:?}", private_key);
    println!("public key = {:?}", public_key);
}
