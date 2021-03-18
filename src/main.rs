use piecewise_polynomial::{Evaluate, IntOfLogPoly4, Segment};

fn main() {
    let poly = IntOfLogPoly4 {
        k: 0.0052622775209652944,
        coeffs: [
            -0.8000546687895224,
            2.7643402381079487,
            26.287021612040366,
            109.18921102766446,
        ],
        u: 22684.248437352602,
    };
    poly.evaluate(1.1276353206339418) == 0.12764305320171115_f64;
}
