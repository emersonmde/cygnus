extern crate uint;
use uint::construct_uint;
construct_uint! {
	pub struct U256(4);
}

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

    fn slope_tangent() {}

    fn point_double(a: U256) {

    }
}
fn mod_inv(a: i32, module: i32) -> i32 {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
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


fn test() {
}
pub fn egcd(a: U256, b: U256) -> (U256, U256, U256) {
    assert!(a < b);
    let zero = U256{0: [0, 0, 0, 0]};
    if a == zero {
        return (b, zero, zero);
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        return (g, y - (b / a) * x, x);
    }
}

pub fn modinverse(a: U256, m: U256) -> Option<U256> {
    let (g, x, _) = egcd(a, m);
    println!("{:?}", g);
    println!("{:?}", x);
    let one = U256{0: [1, 0, 0, 0]};
    if g != one {
        return None;
    }
    else {
        return Some(x % m);
    }
}

fn main() {
    let a = EllipticCurve::new_small(17, 2, 2, 5, 1, 19, 1);
    let b = U256{0: [2, 0, 0, 0]};
    let c = U256{0: [17, 0, 0, 0]};
    println!("test {:?}", b % c);
    println!("test {:?}", modinverse(b, c));
}