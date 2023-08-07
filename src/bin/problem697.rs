// 2: c - c*log(c)
// 3: c - c*log(c) + 1/2*c*log(c)^2
// 4: c - c*log(c) + 1/2*c*log(c)^2 - 1/6*c*log(c)^3 
// n: c * sum((-log(c))^i / i!, i from 0 to n-1)

use std::ops::Neg;

use rug::{Float, ops::CompleteRound};

fn main() {
    println!("{:.2}", solve());
}

const PREC: u32 = 100;

fn solve() -> f64 {
    let n = 10000000;
    // let n = 100;
    let target_prob = Float::with_val(PREC, 0.25);
    let mut loln = Float::with_val(PREC, f64::NEG_INFINITY);
    let mut hiln = Float::with_val(PREC, -1e-10_f64);
    while (&loln).is_infinite() || (&hiln - &loln).complete(PREC) > Float::with_val(PREC, 1e-12) {
        let mid = if (&loln).is_infinite() {
            &hiln * Float::with_val(PREC, 2_f64)
        } else {
            (&loln + &hiln).complete(PREC) / Float::with_val(PREC, 2_f64)
        };
        let prob = calc_prob(n, &mid);
        if prob > target_prob {
            hiln = mid;
        } else {
            loln = mid;
        }
        println!("{:?} {:?} {:?}", loln, hiln, prob);
    }
    return (Float::with_val(PREC, 1_f64).log10() - loln / Float::with_val(PREC, 10_f64).ln()).to_f64();
}

fn calc_prob(n: u64, cln: &Float) -> Float {
    let mut term = Float::with_val(PREC, 1_f64);
    let mut res = term.clone();
    let cln_neg = cln.clone().neg();
    for i in 1..n {
        term *= (&cln_neg) / Float::with_val(PREC, i);
        res += term.clone();
    }
    return res * cln.clone().exp();
}