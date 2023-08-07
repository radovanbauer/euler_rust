use std::collections::HashSet;

use num_bigint::BigInt;

fn main() {
    println!("{:?}", solve());
}

fn solve() -> u128 {
    let n: u128 = 8_u128.pow(12) * 12_u128.pow(8);
    let p = calc_p(n);
    // println!("p={:?}", p);
    let m = 1000000007;
    let mut res: u128 = 0;
    for i in p {
        res = (res
            + u128::try_from(BigInt::from(2).modpow(&BigInt::from(i), &BigInt::from(m))).unwrap())
            % m;
    }
    return res;
}

fn calc_p(n: u128) -> HashSet<u128> {
    if n == 0 {
        let mut res = HashSet::new();
        res.insert(0);
        return res;
    }
    if n % 2 == 0 {
        let x = calc_p(n / 2);
        let mut res = HashSet::new();
        for b in x {
            res.insert(2 * b);
        }
        return res;
    } else {
        let x = calc_p(n - 1);
        let s1 = shift(&x, 1);
        let s3 = shift(&x, 3);
        return add(&add(&x, &s1), &s3);
    }
}

fn add(a: &HashSet<u128>, b: &HashSet<u128>) -> HashSet<u128> {
    let mut res = HashSet::new();
    res.extend(a);
    for x in b {
        if res.contains(x) {
            res.remove(x);
        } else {
            res.insert(*x);
        }
    }
    return res;
}

fn shift(a: &HashSet<u128>, n: u128) -> HashSet<u128> {
    let mut res = HashSet::new();
    for x in a {
        res.insert(x + n);
    }
    return res;
}
