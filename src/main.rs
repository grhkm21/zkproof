use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::RistrettoPoint;
use rand_core::OsRng;
use sha2::{Digest, Sha512};
// use zkproof::constants;
use zkproof::{constants::Point, traits::*};

// Pedersen commitment scheme
// m -> (g^m h^r, r)
pub struct PedersenCommitment;

impl Commitment<Point> for PedersenCommitment {
    fn commit(message: Scalar) -> (Point, Scalar) {
        // let g = &constants::BASE_POINT1;
        let g = Point::BASE_POINT1();
        let h = Point::BASE_POINT2();
        let o = Scalar::random(&mut OsRng);
        let c = g * message + h * o;
        (c, o)
    }

    fn verify(message: Scalar, commit: Point, opening: Scalar) -> bool {
        let g = Point::BASE_POINT1();
        let h = Point::BASE_POINT2();
        commit == g * message + h * opening
    }
}

// Schnorr protocol prover
// Proves knowledge of x such that y = g^x
pub struct Prover {
    // Private key
    x: Option<Scalar>,
    // Public key
    y: Option<RistrettoPoint>,
}

impl Prover {
    fn setXY(mut self, x: Scalar, y: RistrettoPoint) {
        self.x = Some(x);
        self.y = Some(y);
    }

    fn ok(self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    fn message1(self) {
        assert!(self.ok());
    }
}

fn main() {
    let message1 = b"commit me !!!!";
    let message2 = b"commit me pls!";
    let message1_bn = Scalar::hash_from_bytes::<Sha512>(message1);
    let message2_bn = Scalar::hash_from_bytes::<Sha512>(message2);

    // Commit to message
    let (commit1, opening1) = PedersenCommitment::commit(message1_bn);

    // Ensure commitment is validated
    assert!(
        PedersenCommitment::verify(message1_bn, commit1, opening1),
        "Failed to validate commitment of message1"
    );

    // Commit different message
    let (commit2, opening2) = PedersenCommitment::commit(message2_bn);

    assert!(
        !PedersenCommitment::verify(message2_bn, commit1, opening1),
        "Binding property failed!"
    );
    assert!(
        PedersenCommitment::verify(message2_bn, commit2, opening2),
        "Failed to validate commitment of message2"
    );

    println!("Suceess!");
}
