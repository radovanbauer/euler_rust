// G(n, max_p) = sum(G(n - p, p) * D(p), 1 <= p <= max_p)

use std::cmp::min;
use once_cell::sync::Lazy;
use primal::Sieve;
use memoize::memoize;

static sieve: Lazy<Sieve> = Lazy::new(|| Sieve::new(100000));
const MOD: u64 = 999676999;
const N: usize = 50000;

#[memoize]
fn D(n: u64) -> u64 {
    // println!("D({:?})", n);
    if n == 1 {
        return 1;
    }
    let p = sieve.factor(n as usize).unwrap()[0].0 as u64;
    let q = n / p;
    if q == 1 {
        return 1;
    }
    let res = (D(p) * q + p * D(q)) % MOD;
    return res;
}

#[memoize]
fn G(n: u64, max_p: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    // println!("G({:?}, {:?})", n, max_p);
    let mut res = 0;
    for p in 1..=min(max_p, n) {
        res += G(n - p, min(p, n - p)) * D(p);
        res %= MOD;
    }
    return res;
}

fn S(n: u64) -> u64 {
    let mut res = 0;
    for i in 1..=n {
        println!("S({:?})", i);
        res += G(i, i);
        res %= MOD;
    }
    return res;
}

fn main() {
    println!("{:?}", S(N as u64));
}