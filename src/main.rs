use bigint::uint::U256;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: U256,
    y: U256
}

impl Point {
    fn new(x: U256, y: U256) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    fn new_small(x: u64, y: u64) -> Point {
        Point::new(U256{0: [x, 0, 0, 0]}, U256{0: [y, 0, 0, 0]})
    }

}

#[derive(Debug, Copy, Clone)]
struct EllipticCurve {
    p: U256,
    a: U256,
    b: U256,
    g: Point,
    n: U256,
    h: u8
}

impl EllipticCurve {
    fn new(p: U256, a: U256, b: U256, gx: U256, gy: U256, n: U256, h: u8) -> EllipticCurve {
        let g = Point::new(gx, gy);
        EllipticCurve {
            p: p,
            a: a,
            b: b,
            g: g,
            n: n,
            h: h
        }
    }

    fn new_small(p: u64, a: u64, b: u64, gx: u64, gy: u64, n: u64, h: u8) -> EllipticCurve {
        EllipticCurve::new(
            U256{0: [p, 0, 0, 0]},
            U256{0: [a, 0, 0, 0]},
            U256{0: [b, 0, 0, 0]},
            U256{0: [gx, 0, 0, 0]},
            U256{0: [gy, 0, 0, 0]},
            U256{0: [n, 0, 0, 0]},
            h,
        )
    }
}


fn slope_tangent(x: U256, y: U256, a: U256) -> U256 {
    let n = (U256::from(3) * x.pow(U256::from(2))) + U256::from(2);
    ((n % a) * (U256::from(2) * y).mod_inverse(a)) % a
}


fn point_double(x: U256, y: U256, a: U256) {
    let s = slope_tangent(x, y, a);
    let x2 = (s.pow(U256::from(2)) - (U256::from(2) * x)) % a;
    // let y2 = (((s * x) - (s * x2)) - y) % a;
    let y2 = x - x2;
    println!("{:?}", x2);
    println!("{:?}", y2);

}


fn main() {
    let a = EllipticCurve::new_small(17, 2, 2, 5, 1, 19, 1);
    let b = U256{0: [2, 0, 0, 0]};
    let c = U256{0: [17, 0, 0, 0]};
    println!("test {:?}", point_double(U256::from(5), U256::from(1), U256::from(17)));
}