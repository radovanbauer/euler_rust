use std::collections::HashSet;

use num::{Integer, Zero};
use num_bigint::BigInt;

fn main() {
    println!("{:?}", solve());
}

fn solve() -> BigInt {
    let mut not_found: HashSet<u64> = HashSet::new();
    const p: u64 = 10_u64.pow(9) + 7;
    for d in 1..=100000 {
        not_found.insert(p - d);
    }
    let mut n: u64 = 1;
    let mut res = BigInt::zero();
    let mut residues = not_found.iter().filter(|i| legendre(**i, p) == 1).count();
    while !not_found.is_empty() {
        let based = to_base(n * n, p);
        for i in based.clone() {
            if not_found.contains(&i) {
                not_found.remove(&i);
                res += n;
                if legendre(i, p) == 1 {
                    residues -= 1;
                }
                println!("found={:?} n={:?} based={:?} remaining={:?} residues={:?}", i, n, based, not_found.len(), residues);
            }
        }
        if n % 1000000 == 0 {
            println!("n={:?} not_found={:?} based={:?}", n, not_found.len(), based);
        }
        n += 1;
        if residues == 0 {
            break;
        }
    }
    for i in not_found.iter() {
        let mut min = BigInt::zero();
        for a in 0..p {
            let x = BigInt::from(a) * p * p + i * p;
            let sqrt = x.sqrt();
            if sqrt.pow(2) == x {
                min = sqrt;
                println!("i={:?} min={:?} a={:?}", i, &min, a);
                break;
            } else if (sqrt.clone() + 1_u64).pow(2) < x + p {
                min = sqrt + 1;
                println!("i={:?} min={:?} a={:?}", i, &min, a);
                break;
            }
        }
        res += &min;
    }
    return res;
}

fn legendre(a: u64, p: u64) -> u64 {
    return BigInt::from(a).modpow(&BigInt::from((p - 1) / 2), &BigInt::from(p)).try_into().unwrap();
}

fn to_base(n: u64, base: u64) -> Vec<u64> {
    let mut rem = n;
    let mut res: Vec<u64> = Vec::new();
    while rem > 0 {
        let (d, m) = rem.div_mod_floor(&base);
        res.push(m);
        rem = d;
    }
    return res;
}