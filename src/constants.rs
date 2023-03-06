use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::RistrettoPoint;
use custom_derive::custom_derive;
use newtype_derive::*;
use sha2::{Digest, Sha512};

custom_derive! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, NewtypeFrom, NewtypeAdd, NewtypeMul(Scalar),
             NewtypeDeref, NewtypeDerefMut)]
    pub struct Point(RistrettoPoint);
}

impl Point {
    pub fn new(pt: RistrettoPoint) -> Self {
        Point(pt)
    }

    pub fn BASE_POINT1() -> Self {
        Point::new(constants::RISTRETTO_BASEPOINT_POINT)
    }

    pub fn BASE_POINT2() -> Self {
        let g = constants::RISTRETTO_BASEPOINT_POINT;
        let h = Scalar::hash_from_bytes::<Sha512>(b"base_point2");
        Point::new(h * g)
    }
}

fn main() {
    let p1: RistrettoPoint = constants::RISTRETTO_BASEPOINT_POINT;
    let p2: Point = Point::BASE_POINT1();
    let k: Scalar = Scalar::hash_from_bytes::<Sha512>(b"asdf");
    println!("k = {k:?}");

    let h1 = p1 * k;
    let h2 = p2 * k;

    println!("{h1:?}");
    println!("{h2:?}");
}
